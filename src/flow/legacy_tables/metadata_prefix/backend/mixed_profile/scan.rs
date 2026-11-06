use super::*;
pub(super) struct MixedReader<'a> {
    cursor: ArchiveCursor<'a>,
    backend: &'a FlowMediumBackendPrefix,
    options: FlowMediumMixedOptions,
    first_keys: [BTreeSet<[u8; 16]>; 2],
    second_keys: BTreeSet<[u8; 16]>,
    pub remaining_references: usize,
    pub remaining_words: usize,
    pub remaining_patterns: usize,
    pub remaining_names: usize,
}
impl<'a> MixedReader<'a> {
    pub fn new(
        backend: &'a FlowMediumBackendPrefix,
        options: FlowMediumMixedOptions,
    ) -> Result<Self> {
        let mut cursor = ArchiveCursor::new(backend.metadata_prefix().payload_bytes());
        cursor.take(backend.metadata_prefix().coverage().known_prefix_bytes)?;
        zero(&mut cursor)?;
        Ok(Self {
            cursor,
            backend,
            options,
            first_keys: std::array::from_fn(|_| BTreeSet::new()),
            second_keys: BTreeSet::new(),
            remaining_references: options.profile.maximum_references,
            remaining_words: options.profile.maximum_total_wide_words,
            remaining_patterns: options.maximum_total_word_patterns,
            remaining_names: options.profile.maximum_total_name_units,
        })
    }
    pub fn complete(
        self,
        first: Vec<FlowMediumMixedEntry>,
        second: Vec<FlowMediumMixedReference>,
        outer: OuterReferences,
        backend_end: usize,
    ) -> FlowMediumMixedProfile {
        FlowMediumMixedProfile {
            backend: self.backend.clone(),
            options: self.options,
            first: first.into(),
            second: second.into(),
            outer,
            backend_end,
            references: self.options.profile.maximum_references - self.remaining_references,
            words: self.options.profile.maximum_total_wide_words - self.remaining_words,
            word_patterns: self.options.maximum_total_word_patterns - self.remaining_patterns,
            name_units: self.options.profile.maximum_total_name_units - self.remaining_names,
        }
    }
    pub fn position(&self) -> usize {
        self.backend.metadata_prefix().payload_bytes().len() - self.cursor.remaining()
    }
    pub fn finish(&self) -> Result<()> {
        self.cursor.finish()
    }
    pub fn first_objects(&mut self) -> Result<Vec<FlowMediumMixedEntry>> {
        let count = table_count(
            &mut self.cursor,
            20,
            self.options.profile.maximum_first_objects,
            "mixed first objects",
        )?;
        (0..count).map(|_| self.first_object()).collect()
    }
    fn first_object(&mut self) -> Result<FlowMediumMixedEntry> {
        let start = self.position();
        let kind = FlowMediumKind::try_from(self.cursor.i32()?)?;
        let key = guid(&mut self.cursor)?;
        let body = decode::body(self.backend, self.position(), kind, self.options)?;
        let (words, patterns, names) = body.totals();
        spend(&mut self.remaining_words, words, "total body wide words")?;
        spend(
            &mut self.remaining_patterns,
            patterns,
            "total body word patterns",
        )?;
        spend(&mut self.remaining_names, names, "total body name units")?;
        self.cursor
            .take(body.payload_interval().1 - self.position())?;
        self.first_keys[kind.index()].insert(*key.as_bytes());
        Ok(FlowMediumMixedEntry {
            full: (start, self.position()),
            key,
            body,
        })
    }
    pub fn reference_objects(&mut self) -> Result<Vec<FlowMediumMixedReference>> {
        let count = table_count(
            &mut self.cursor,
            26,
            self.options.profile.maximum_reference_objects,
            "mixed reference objects",
        )?;
        (0..count).map(|_| self.reference_object()).collect()
    }
    fn reference_object(&mut self) -> Result<FlowMediumMixedReference> {
        let start = self.position();
        let kind = FlowMediumKind::try_from(self.cursor.i32()?)?;
        zero(&mut self.cursor)?;
        zero(&mut self.cursor)?;
        let key = guid(&mut self.cursor)?;
        let state = if kind == FlowMediumKind::One {
            Some((self.position(), self.cursor.u8()?))
        } else {
            None
        };
        let (references_start, references) = self.first_references(kind)?;
        self.second_keys.insert(*key.as_bytes());
        Ok(FlowMediumMixedReference {
            full: (start, self.position()),
            kind,
            key,
            state,
            references_start,
            references: references.into(),
        })
    }
    fn first_references(&mut self, kind: FlowMediumKind) -> Result<(usize, Vec<FlowGuid>)> {
        let count = self
            .cursor
            .nonnegative_count(16, "typed mixed references")?;
        spend(
            &mut self.remaining_references,
            count,
            "total reference occurrences",
        )?;
        let references_start = self.position();
        let references = references(
            &mut self.cursor,
            count,
            &self.first_keys[kind.index()],
            "selected fresh first-table kind map",
        )?;
        Ok((references_start, references))
    }
    pub fn outer_references(&mut self) -> Result<OuterReferences> {
        profile_wire::outer_references(
            &mut self.cursor,
            self.backend.metadata_prefix().payload_bytes().len(),
            &self.second_keys,
            &mut self.remaining_references,
            "mixed-kind",
        )
    }
}
