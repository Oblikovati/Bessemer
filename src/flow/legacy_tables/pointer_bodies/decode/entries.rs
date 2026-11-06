use super::*;

impl PointerBodyReader<'_> {
    pub(super) fn entry(&mut self, group: usize, ordinal: usize) -> Result<()> {
        let start = self.position();
        let first = self.vectors.len();
        let (kind, payload, stored_count) = if self.version >= 3 {
            self.tagged()?
        } else {
            self.legacy(group, ordinal)?
        };
        self.entries.push(BodyEntrySpan {
            full: self.interval(start),
            payload,
            kind,
            stored_count,
            vectors: BodySpan {
                start: first,
                end: self.vectors.len(),
            },
        });
        Ok(())
    }

    fn legacy(&mut self, group: usize, ordinal: usize) -> Result<(Kind, BodySpan, Option<u32>)> {
        if group < 2 {
            return Ok((Kind::SingleVector, self.vector()?, None));
        }
        let code = self.options.entry_codes.as_deref().and_then(|codes| codes.get(ordinal)).copied()
            .ok_or_else(|| body_invalid(format!("runtime code ordinal {ordinal}; expected one current code per legacy third-collection entry")))?;
        let count = self.cursor.u8()? as usize;
        if is_wide_code(code) {
            return Ok((
                Kind::WideTriplets,
                self.wide(count * 3)?,
                Some(count as u32),
            ));
        }
        Ok((
            Kind::MultipleVectors,
            self.multiple(count)?,
            Some(count as u32),
        ))
    }

    fn multiple(&mut self, count: usize) -> Result<BodySpan> {
        let start = self.position();
        for _ in 0..count {
            self.vector()?;
        }
        Ok(self.interval(start))
    }

    fn tagged(&mut self) -> Result<(Kind, BodySpan, Option<u32>)> {
        let tag = self.cursor.u8()?;
        if tag != 1 && tag != 2 {
            let end = self.position();
            return Ok((Kind::UnknownTag, BodySpan { start: end, end }, None));
        }
        let count = self.cursor.nonnegative_count(1, "tagged pointer body")?;
        if self.version == 3 && tag == 2 {
            let words = count
                .checked_mul(3)
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| {
                    body_invalid(format!(
                        "tagged count {count}; expected three times count plus one to fit usize"
                    ))
                })?;
            return Ok((Kind::TaggedWide, self.wide(words)?, Some(count as u32)));
        }
        Ok((Kind::TaggedRaw, self.raw(count)?, Some(count as u32)))
    }
}

fn is_wide_code(code: u8) -> bool {
    [
        3, 7, 30, 31, 32, 40, 41, 42, 50, 51, 52, 60, 61, 62, 73, 77, 82, 83, 84, 85, 86, 87,
    ]
    .contains(&code)
}
