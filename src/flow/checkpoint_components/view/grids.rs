use super::*;
/// Borrowed complete cyclic grid; dimensions are structural, signed members remain editable.
#[derive(Clone, Copy, Debug)]
pub struct FlowCheckpointGrid<'a> {
    owner: &'a FlowCheckpointComponent,
    layout: &'a GridLayout,
}
/// Borrowed group with every stored presence position, including absent rows.
#[derive(Clone, Copy, Debug)]
pub struct FlowCheckpointGroup<'a> {
    owner: &'a FlowCheckpointComponent,
    layout: &'a GroupLayout,
    axis: usize,
}
/// Borrowed present row and its optional graph-two slot stream.
#[derive(Clone, Copy, Debug)]
pub struct FlowCheckpointRow<'a> {
    owner: &'a FlowCheckpointComponent,
    layout: &'a RowLayout,
    ordinal: usize,
}
/// Borrowed slot stream; an empty stored stream is distinct from an auxiliary row with no stream.
#[derive(Clone, Copy, Debug)]
pub struct FlowCheckpointSlots<'a> {
    owner: &'a FlowCheckpointComponent,
    slots: &'a [SlotLayout],
    interval: (usize, usize),
}
/// Borrowed raw tag and optional complete numeric container.
#[derive(Clone, Copy, Debug)]
pub struct FlowCheckpointSlot<'a> {
    owner: &'a FlowCheckpointComponent,
    layout: &'a SlotLayout,
    ordinal: usize,
}
impl FlowCheckpointComponent {
    /// Borrows a graph-two or auxiliary grid; e.g. `let group = view.grid()?.group(0)?`.
    pub fn grid(&self) -> Result<FlowCheckpointGrid<'_>> {
        let Layout::Grid(layout) = &self.revision.layout else {
            return Err(wrong_profile(self, "Graph2 or Auxiliary"));
        };
        Ok(FlowCheckpointGrid {
            owner: self,
            layout,
        })
    }
}
impl<'a> FlowCheckpointGrid<'a> {
    /// Returns parsed nonnegative dimensions; e.g. `let dimensions = grid.dimensions()`.
    pub fn dimensions(self) -> [i32; 3] {
        self.layout.dimensions
    }
    /// Returns a checked signed member field; e.g. `let value = grid.member(0)?.value()?`.
    pub fn member(self, axis: usize) -> Result<FlowCheckpointField> {
        let axis = checked_index(axis, 3, "grid member")?;
        Ok(self
            .owner
            .field(12 + axis * 4, ValueShape::Signed32, ValueRule::Any))
    }
    /// Borrows one cyclic axis group; e.g. `let group = grid.group(2)?`.
    pub fn group(self, axis: usize) -> Result<FlowCheckpointGroup<'a>> {
        let axis = checked_index(axis, 3, "grid group")?;
        Ok(FlowCheckpointGroup {
            owner: self.owner,
            layout: &self.layout.groups[axis],
            axis,
        })
    }
}
impl<'a> FlowCheckpointGroup<'a> {
    /// Returns the axis ordinal; e.g. `let axis = group.axis()`.
    pub fn axis(self) -> usize {
        self.axis
    }
    /// Returns dimension-derived positions, including absent rows; e.g. `let count = group.len()`.
    pub fn len(self) -> usize {
        self.layout.rows.len()
    }
    /// Distinguishes an empty complete group; e.g. `let empty = group.is_empty()`.
    pub fn is_empty(self) -> bool {
        self.layout.rows.is_empty()
    }
    /// Returns the raw presence interval before all row bodies; e.g. `let span = group.presence_interval()`.
    pub fn presence_interval(self) -> (usize, usize) {
        (self.layout.presence.start, self.layout.presence.end())
    }
    /// Borrows unchanged presence bytes; e.g. `let flags = group.presence_bytes()`.
    pub fn presence_bytes(self) -> &'a [u8] {
        &self.owner.as_bytes()[self.layout.presence.start..self.layout.presence.end()]
    }
    /// Returns a presence field whose zero/nonzero class is protected; e.g. `let field = group.presence(0)?`.
    pub fn presence(self, index: usize) -> Result<FlowCheckpointField> {
        let index = checked_index(index, self.len(), "row presence")?;
        Ok(self.owner.field(
            self.layout.presence.start + index,
            ValueShape::Byte,
            ValueRule::Presence,
        ))
    }
    /// Returns a checked position, preserving absent rows as None; e.g. `let row = group.row(0)?`.
    pub fn row(self, index: usize) -> Result<Option<FlowCheckpointRow<'a>>> {
        let ordinal = checked_index(index, self.len(), "grid row")?;
        Ok(self.layout.rows[ordinal]
            .as_ref()
            .map(|layout| FlowCheckpointRow {
                owner: self.owner,
                layout,
                ordinal,
            }))
    }
}
impl<'a> FlowCheckpointRow<'a> {
    /// Returns the original group position; e.g. `let ordinal = row.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the count and complete body interval; e.g. `let span = row.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.layout.start, self.layout.end)
    }
    /// Returns the nonnegative stored index count; e.g. `let count = row.stored_count()`.
    pub fn stored_count(self) -> usize {
        self.layout.indices.count
    }
    /// Borrows the raw index array; e.g. `let value = row.indices().value(0)?`.
    pub fn indices(self) -> FlowCheckpointPatternArray<'a> {
        array(self.owner, self.layout.indices)
    }
    /// Distinguishes a stored slot stream from an auxiliary row; e.g. `let slots = row.slots()`.
    pub fn slots(self) -> Option<FlowCheckpointSlots<'a>> {
        self.layout.slots.as_ref().map(|slots| FlowCheckpointSlots {
            owner: self.owner,
            slots,
            interval: (self.layout.indices.end(), self.layout.end),
        })
    }
}
impl<'a> FlowCheckpointSlots<'a> {
    /// Returns one slot per stored row index; e.g. `let count = slots.len()`.
    pub fn len(self) -> usize {
        self.slots.len()
    }
    /// Distinguishes a complete empty slot stream; e.g. `let empty = slots.is_empty()`.
    pub fn is_empty(self) -> bool {
        self.slots.is_empty()
    }
    /// Returns all raw slot bytes and bodies; e.g. `let span = slots.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.interval
    }
    /// Borrows a checked slot; e.g. `let slot = slots.slot(0)?`.
    pub fn slot(self, index: usize) -> Result<FlowCheckpointSlot<'a>> {
        let ordinal = checked_index(index, self.slots.len(), "row slot")?;
        Ok(FlowCheckpointSlot {
            owner: self.owner,
            layout: &self.slots[ordinal],
            ordinal,
        })
    }
}
impl<'a> FlowCheckpointSlot<'a> {
    /// Returns stored slot order; e.g. `let ordinal = slot.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the tag and optional body interval; e.g. `let span = slot.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (
            self.layout.start,
            self.layout
                .container
                .as_ref()
                .map_or(self.layout.start + 1, |value| value.end),
        )
    }
    /// Returns the raw tag field with body-presence class protected; e.g. `let raw = slot.tag().value()?`.
    pub fn tag(self) -> FlowCheckpointField {
        self.owner
            .field(self.layout.start, ValueShape::Byte, ValueRule::SlotTag)
    }
    /// Reports writer-tag canonicality separately from reader acceptance; e.g. `let canonical = slot.canonical_writer_tag()?`.
    pub fn canonical_writer_tag(self) -> Result<bool> {
        match self.tag().value()? {
            FlowCheckpointValue::Byte(value) => Ok(value <= 2),
            value => Err(invalid(
                "slot tag",
                format!("value {value:?}; expected a byte tag"),
            )),
        }
    }
    /// Borrows a present container for every tag above one; e.g. `let body = slot.container()`.
    pub fn container(self) -> Option<FlowCheckpointContainer<'a>> {
        self.layout
            .container
            .as_ref()
            .map(|layout| containers::container(self.owner, layout))
    }
}
