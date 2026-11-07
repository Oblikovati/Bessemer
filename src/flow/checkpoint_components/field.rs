use super::*;
// #248: identities retain complete buffers, so diagnostic formatting must stay bounded.
impl std::fmt::Debug for FlowCheckpointField {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FlowCheckpointField")
            .field("kind", &self.revision.kind)
            .field("start", &self.start)
            .field("shape", &self.shape)
            .field("rule", &self.rule)
            .finish()
    }
}
impl PartialEq for FlowCheckpointField {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.revision, &other.revision)
            && self.start == other.start
            && self.shape == other.shape
            && self.rule == other.rule
    }
}
impl Eq for FlowCheckpointField {}
impl FlowCheckpointField {
    /// Returns the exact half-open component interval; e.g. `let span = field.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (self.start, self.start + self.shape.width())
    }
    /// Reads the exact serialized value class; e.g. `let raw = field.value()?`.
    pub fn value(&self) -> Result<FlowCheckpointValue> {
        let (start, end) = self.payload_interval();
        let raw = self.revision.wire.get(start..end).ok_or_else(|| {
            invalid(
                "field interval",
                format!(
                    "interval {start}..{end}; expected bytes within component length {}",
                    self.revision.wire.len()
                ),
            )
        })?;
        let mut cursor = ArchiveCursor::new(raw);
        let value = match self.shape {
            ValueShape::Byte => cursor.u8().map(FlowCheckpointValue::Byte),
            ValueShape::Signed32 => cursor.i32().map(FlowCheckpointValue::Signed32),
            ValueShape::Pattern32 => cursor.u32().map(FlowCheckpointValue::Pattern32),
            ValueShape::Pattern64 => cursor.u64().map(FlowCheckpointValue::Pattern64),
        };
        value.map_err(boundary)
    }
    pub(super) fn encoded(&self, value: FlowCheckpointValue) -> Result<Vec<u8>> {
        if value.shape() != self.shape {
            return Err(invalid(
                "replacement field class",
                format!(
                    "value {value:?} at offset {}; expected serialized class {:?}",
                    self.start, self.shape
                ),
            ));
        }
        self.check_rule(value)?;
        Ok(value.encode())
    }
    fn check_rule(&self, value: FlowCheckpointValue) -> Result<()> {
        if self.rule == ValueRule::Any {
            return Ok(());
        }
        let (old, new) = self.byte_pair(value)?;
        if self.rule.same_class(old, new) {
            return Ok(());
        }
        Err(invalid(
            "replacement byte structure",
            format!(
                "byte {new} replacing {old} at offset {}; expected the same {:?} structural class",
                self.start, self.rule
            ),
        ))
    }
    fn byte_pair(&self, value: FlowCheckpointValue) -> Result<(u8, u8)> {
        match (self.value()?, value) {
            (FlowCheckpointValue::Byte(old), FlowCheckpointValue::Byte(new)) => Ok((old, new)),
            pair => Err(invalid(
                "byte field rule",
                format!("values {pair:?}; expected byte values for {:?}", self.rule),
            )),
        }
    }
}
impl FlowCheckpointValue {
    fn shape(self) -> ValueShape {
        match self {
            Self::Byte(_) => ValueShape::Byte,
            Self::Signed32(_) => ValueShape::Signed32,
            Self::Pattern32(_) => ValueShape::Pattern32,
            Self::Pattern64(_) => ValueShape::Pattern64,
        }
    }
    fn encode(self) -> Vec<u8> {
        match self {
            Self::Byte(value) => vec![value],
            Self::Signed32(value) => value.to_le_bytes().to_vec(),
            Self::Pattern32(value) => value.to_le_bytes().to_vec(),
            Self::Pattern64(value) => value.to_le_bytes().to_vec(),
        }
    }
}
impl ValueRule {
    fn same_class(self, old: u8, new: u8) -> bool {
        match self {
            Self::Any => true,
            Self::Presence => (old == 0) == (new == 0),
            Self::SlotTag => (old <= 1) == (new <= 1),
        }
    }
}
