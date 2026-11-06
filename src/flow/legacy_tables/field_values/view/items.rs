use super::*;

impl<'a> FlowFieldObject<'a> {
    /// Returns the supplied global-object ordinal; e.g. `let ordinal = object.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the admitted external object width; e.g. `let width = object.width().value()`.
    pub fn width(self) -> FlowFieldCount {
        self.width
    }
    /// Returns the body interval, excluding the version prefix; e.g. `let interval = object.body_interval()`.
    pub fn body_interval(self) -> (usize, usize) {
        self.span.full
    }
    /// Borrows all raw wide patterns without float normalization; e.g. `let bits = object.wide_words().raw_bits(0)`.
    pub fn wide_words(self) -> FlowTableWideArray<'a> {
        field_wide(&self.owner.wire, self.span.wide)
    }
}

impl<'a> FlowFieldRegion<'a> {
    /// Returns the physical runtime region ordinal; e.g. `let ordinal = region.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns declared rows independently of active storage; e.g. `let rows = region.row_count().value()`.
    pub fn row_count(self) -> FlowFieldCount {
        self.context.row_count
    }
    /// Returns the stored width checked against explicit context; e.g. `let width = region.width().value()`.
    pub fn width(self) -> FlowFieldCount {
        self.context.width
    }
    /// Returns the supplied storage branch, not a file flag; e.g. `let active = region.active()`.
    pub fn active(self) -> bool {
        self.context.active
    }
    /// Returns the body interval including the stored width, excluding version; e.g. `let interval = region.body_interval()`.
    pub fn body_interval(self) -> (usize, usize) {
        self.span.full
    }
    /// Borrows actual raw values, empty for inactive/zero-size regions; e.g. `let words = region.wide_words()`.
    pub fn wide_words(self) -> FlowTableWideArray<'a> {
        field_wide(&self.owner.wire, self.span.wide)
    }
}
