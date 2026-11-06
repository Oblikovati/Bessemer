use super::*;
pub(super) struct ProfileReader<'a> {
    cursor: ArchiveCursor<'a>,
    backend: &'a FlowMediumBackendPrefix,
    options: FlowMediumProfileOptions,
    first_keys: BTreeSet<[u8; 16]>,
    second_keys: BTreeSet<[u8; 16]>,
    pub remaining_references: usize,
    pub remaining_words: usize,
    pub remaining_names: usize,
}
impl<'a> ProfileReader<'a> {
    pub fn new(
        backend: &'a FlowMediumBackendPrefix,
        options: FlowMediumProfileOptions,
    ) -> Result<Self> {
        let mut cursor = ArchiveCursor::new(backend.metadata_prefix().payload_bytes());
        cursor.take(backend.metadata_prefix().coverage().known_prefix_bytes)?;
        zero(&mut cursor)?;
        Ok(Self {
            cursor,
            backend,
            options,
            first_keys: BTreeSet::new(),
            second_keys: BTreeSet::new(),
            remaining_references: options.maximum_references,
            remaining_words: options.maximum_total_wide_words,
            remaining_names: options.maximum_total_name_units,
        })
    }
    pub fn position(&self) -> usize {
        self.backend.metadata_prefix().payload_bytes().len() - self.cursor.remaining()
    }
    pub fn finish(&self) -> Result<()> {
        self.cursor.finish()
    }
    pub fn first_objects(&mut self) -> Result<Vec<FirstObject>> {
        let count = table_count(
            &mut self.cursor,
            20,
            self.options.maximum_first_objects,
            "first objects",
        )?;
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(self.first_object()?);
        }
        Ok(result)
    }
    fn first_object(&mut self) -> Result<FirstObject> {
        let start = self.position();
        kind_one(&mut self.cursor)?;
        let key = guid(&mut self.cursor)?;
        let body = FlowMediumKindOne::read_at(self.backend, self.position(), self.options.body)?;
        self.spend_body(&body)?;
        self.cursor
            .take(body.payload_interval().1 - self.position())?;
        self.first_keys.insert(*key.as_bytes());
        Ok(FirstObject {
            full: (start, self.position()),
            key,
            body,
        })
    }
    fn spend_body(&mut self, body: &FlowMediumKindOne) -> Result<()> {
        let coverage = body.coverage();
        spend(
            &mut self.remaining_words,
            coverage.wide_words,
            "total body wide words",
        )?;
        spend(
            &mut self.remaining_names,
            coverage.name_units,
            "total body name units",
        )?;
        Ok(())
    }
    pub fn reference_objects(&mut self) -> Result<Vec<ReferenceObject>> {
        let count = table_count(
            &mut self.cursor,
            27,
            self.options.maximum_reference_objects,
            "reference objects",
        )?;
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(self.reference_object()?);
        }
        Ok(result)
    }
    fn reference_object(&mut self) -> Result<ReferenceObject> {
        let start = self.position();
        kind_one(&mut self.cursor)?;
        zero(&mut self.cursor)?;
        zero(&mut self.cursor)?;
        let key = guid(&mut self.cursor)?;
        let state = self.position();
        self.cursor.u8()?;
        let (references_start, references) = self.first_references()?;
        self.second_keys.insert(*key.as_bytes());
        Ok(ReferenceObject {
            full: (start, self.position()),
            key,
            state,
            references_start,
            references: references.into(),
        })
    }
    fn first_references(&mut self) -> Result<(usize, Vec<FlowGuid>)> {
        let count = self.cursor.nonnegative_count(16, "kind-one references")?;
        spend(
            &mut self.remaining_references,
            count,
            "total reference occurrences",
        )?;
        let references_start = self.position();
        let references = references(
            &mut self.cursor,
            count,
            &self.first_keys,
            "fresh first table",
        )?;
        Ok((references_start, references))
    }
    pub fn outer_references(&mut self) -> Result<OuterReferences> {
        let count = self.cursor.nonnegative_count(20, "outer references")?;
        spend(
            &mut self.remaining_references,
            count,
            "total reference occurrences",
        )?;
        let start = self.position();
        let keys = references(
            &mut self.cursor,
            count,
            &self.second_keys,
            "fresh second table",
        )?;
        let (attributes_start, attributes) = self.read_attributes(count)?;
        Ok(OuterReferences {
            start,
            keys: keys.into(),
            attributes_start,
            attributes: attributes.into(),
        })
    }
    fn read_attributes(&mut self, count: usize) -> Result<(usize, Vec<i32>)> {
        let attributes_start = self.position();
        let mut attributes = Vec::with_capacity(count);
        for _ in 0..count {
            attributes.push(self.cursor.i32()?);
        }
        final_empty(&mut self.cursor)?;
        final_empty(&mut self.cursor)?;
        Ok((attributes_start, attributes))
    }
}
fn references(
    cursor: &mut ArchiveCursor<'_>,
    count: usize,
    keys: &BTreeSet<[u8; 16]>,
    label: &str,
) -> Result<Vec<FlowGuid>> {
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        let key = guid(cursor)?;
        if !keys.contains(key.as_bytes()) {
            return Err(metadata_invalid(format!(
                "reference key {key}; expected membership in {label}"
            )));
        }
        result.push(key);
    }
    Ok(result)
}
fn guid(cursor: &mut ArchiveCursor<'_>) -> Result<FlowGuid> {
    let mut key = [0; 16];
    key.copy_from_slice(cursor.take(16)?);
    Ok(FlowGuid::from_bytes(key))
}
fn zero(cursor: &mut ArchiveCursor<'_>) -> Result<()> {
    let version = cursor.u8()?;
    if version != 0 {
        return Err(metadata_invalid(format!(
            "profile table/reference version {version}; expected zero"
        )));
    }
    Ok(())
}
fn kind_one(cursor: &mut ArchiveCursor<'_>) -> Result<()> {
    let kind = cursor.i32()?;
    if kind != 1 {
        return Err(metadata_invalid(format!(
            "profile object kind {kind}; expected kind one"
        )));
    }
    Ok(())
}
fn table_count(
    cursor: &mut ArchiveCursor<'_>,
    minimum: usize,
    maximum: usize,
    label: &str,
) -> Result<usize> {
    zero(cursor)?;
    let count = cursor.nonnegative_count(minimum, label)?;
    if count > maximum {
        return Err(metadata_invalid(format!(
            "{label} count {count}; expected at most {maximum}"
        )));
    }
    Ok(count)
}
fn final_empty(cursor: &mut ArchiveCursor<'_>) -> Result<()> {
    let count = cursor.i32()?;
    if count != 0 {
        return Err(metadata_invalid(format!(
            "final outer collection count {count}; expected zero for supported kind-one profile"
        )));
    }
    Ok(())
}
fn spend(remaining: &mut usize, count: usize, label: &str) -> Result<()> {
    if count > *remaining {
        return Err(metadata_invalid(format!(
            "{label} count {count}; expected at most {} remaining",
            *remaining
        )));
    }
    *remaining -= count;
    Ok(())
}
