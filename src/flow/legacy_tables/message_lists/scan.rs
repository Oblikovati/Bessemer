use super::*;
pub(super) struct MessageReader<'a> {
    cursor: ArchiveCursor<'a>,
    raw: &'a [u8],
    options: FlowMessageListOptions,
    encoding: FlowMessageTextEncoding,
    pub texts: Vec<MessageTextSpan>,
    pub totals: [usize; 3],
}
impl<'a> MessageReader<'a> {
    pub(super) fn new(
        raw: &'a [u8],
        options: FlowMessageListOptions,
        encoding: FlowMessageTextEncoding,
    ) -> Self {
        Self {
            cursor: ArchiveCursor::new(raw),
            raw,
            options,
            encoding,
            texts: Vec::new(),
            totals: [0; 3],
        }
    }
    fn position(&self) -> usize {
        self.raw.len() - self.cursor.remaining()
    }
    pub(super) fn record(&mut self, version: i32) -> Result<Vec<MessageItemSpan>> {
        self.scope()?;
        self.scalar()?;
        let count = self.count()?;
        bounded(0, count, self.options.maximum_items, "items")?;
        self.minimum(count, if version == 3 { 16 } else { 12 })?;
        let mut items = Vec::with_capacity(count);
        for ordinal in 0..count {
            items.push(self.item(ordinal, version == 3)?);
        }
        self.cursor.finish()?;
        Ok(items)
    }
    fn item(&mut self, ordinal: usize, second_present: bool) -> Result<MessageItemSpan> {
        self.scope()?;
        let start = self.position();
        self.scalar()?;
        let primary = self.text(FlowMessageTextRole::Primary(ordinal))?;
        let first = self.list(ordinal, false)?;
        let second = if second_present {
            Some(self.list(ordinal, true)?)
        } else {
            None
        };
        Ok(MessageItemSpan {
            interval: (start, self.position()),
            primary,
            first,
            second,
        })
    }
    fn list(&mut self, item: usize, second: bool) -> Result<MessageListSpan> {
        let (start, count) = self.list_header()?;
        let first = self.texts.len();
        for index in 0..count {
            let role = if second {
                FlowMessageTextRole::SecondList(item, index)
            } else {
                FlowMessageTextRole::FirstList(item, index)
            };
            self.text(role)?;
        }
        Ok(MessageListSpan {
            interval: (start, self.position()),
            texts: (first, self.texts.len()),
        })
    }
    fn list_header(&mut self) -> Result<(usize, usize)> {
        self.scope()?;
        let start = self.position();
        let count = self.count()?;
        self.minimum(count, 4)?;
        bounded(
            self.texts.len(),
            count,
            self.options.maximum_strings,
            "strings",
        )?;
        Ok((start, count))
    }
    fn text(&mut self, role: FlowMessageTextRole) -> Result<usize> {
        self.scope()?;
        bounded(self.texts.len(), 1, self.options.maximum_strings, "strings")?;
        let start = self.position();
        let (count, prefix_units) = self.text_transfer(start)?;
        let ordinal = self.texts.len();
        self.texts.push(MessageTextSpan {
            role,
            interval: (start, self.position()),
            count,
            prefix_units,
        });
        Ok(ordinal)
    }
    fn text_transfer(&mut self, start: usize) -> Result<(usize, Option<usize>)> {
        let count = self.count()?;
        let width = self.encoding.element_width();
        self.minimum(count, width)?;
        self.spend(
            0,
            count * width,
            self.options.maximum_text_bytes,
            "transferred text bytes",
        )?;
        self.spend(1, count, self.options.maximum_values, "stored values")?;
        let raw = self.cursor.take(count * width)?;
        let prefix_units = if self.encoding == FlowMessageTextEncoding::Utf16Units {
            Some(wire::utf16_text_prefix(301, start, raw)?)
        } else {
            None
        };
        Ok((count, prefix_units))
    }
    fn scalar(&mut self) -> Result<usize> {
        self.spend(1, 1, self.options.maximum_values, "stored values")?;
        let start = self.position();
        self.cursor.take(4)?;
        Ok(start)
    }
    fn count(&mut self) -> Result<usize> {
        let start = self.scalar()?;
        let count = wire::signed_at(self.raw, start);
        if count >= 0 {
            return Ok(count as usize);
        }
        Err(message_invalid(format!(
            "count {count} at byte {start}; expected nonnegative signed-i32 count"
        )))
    }
    fn minimum(&self, count: usize, width: usize) -> Result<()> {
        bounded(
            0,
            count,
            self.cursor.remaining() / width,
            "complete physical elements",
        )
    }
    fn scope(&mut self) -> Result<()> {
        self.spend(2, 1, self.options.maximum_scopes, "scope descriptors")
    }
    fn spend(&mut self, axis: usize, count: usize, maximum: usize, role: &str) -> Result<()> {
        bounded(self.totals[axis], count, maximum, role)?;
        self.totals[axis] += count;
        Ok(())
    }
}
fn bounded(used: usize, additional: usize, maximum: usize, role: &str) -> Result<()> {
    if used <= maximum && additional <= maximum - used {
        return Ok(());
    }
    Err(message_invalid(format!(
        "{role} additional {additional}, previous total {used}; expected aggregate at most {maximum}"
    )))
}
