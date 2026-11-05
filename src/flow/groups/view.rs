use super::*;

impl FlowGroupView<'_> {
    /// Returns the physical group ordinal; e.g. `let index = group.index()`.
    pub fn index(&self) -> usize {
        self.ordinal
    }
    /// Returns the decoded count-prefix offset; e.g. `let start = group.start()`.
    pub fn start(&self) -> usize {
        self.span.start
    }
    /// Returns the exclusive complete decoded group end; e.g. `let end = group.end()`.
    pub fn end(&self) -> usize {
        self.span.end
    }
    /// Returns the validated member count; e.g. `let count = group.len()`.
    pub fn len(&self) -> usize {
        self.span.members
    }
    /// Distinguishes an empty complete group; e.g. `assert!(group.is_empty())`.
    pub fn is_empty(&self) -> bool {
        self.span.members == 0
    }
    /// Retains version-4 auxiliary words, including empty groups; e.g. `let raw = group.auxiliary()`.
    pub fn auxiliary(&self) -> Option<i32> {
        (self.revision.version == FlowGroupVersion::V4)
            .then(|| fields::word(&self.revision.decoded, self.span.start + 4) as i32)
    }
    /// Iterates all raw u32 patterns in wire order; e.g. `let words = group.words().collect::<Vec<_>>()`.
    pub fn words(&self) -> impl ExactSizeIterator<Item = u32> + '_ {
        (0..self.span.members).map(move |member| {
            let offset = fields::member_offset(
                self.revision.version,
                self.span,
                member,
                fields::MemberField::Word,
            );
            fields::word(&self.revision.decoded, offset)
        })
    }
    /// Iterates raw u64 patterns without floating-point conversion; e.g. `let bits = group.wide_words().collect::<Vec<_>>()`.
    pub fn wide_words(&self) -> impl ExactSizeIterator<Item = u64> + '_ {
        (0..self.span.members).map(move |member| {
            let offset = fields::member_offset(
                self.revision.version,
                self.span,
                member,
                fields::MemberField::WideWord,
            );
            fields::wide_word(&self.revision.decoded, offset)
        })
    }
    /// Preserves absent versus present-empty wire flags; e.g. `let flags = group.flags().map(Iterator::collect::<Vec<_>>)`.
    pub fn flags(&self) -> Option<impl ExactSizeIterator<Item = u8> + '_> {
        fields::has_flags(self.revision.version).then(|| {
            (0..self.span.members).map(move |member| {
                let offset = fields::member_offset(
                    self.revision.version,
                    self.span,
                    member,
                    fields::MemberField::Flag,
                );
                self.revision.decoded[offset]
            })
        })
    }
}
