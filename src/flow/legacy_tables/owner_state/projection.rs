use super::*;
impl FlowOwnerMember {
    /// Returns the documented native byte offset; e.g. `assert_eq!(FlowOwnerMember::Member20.native_offset(), 0x20)`.
    pub fn native_offset(self) -> u32 {
        8 + (self as u32) * 4
    }
}
impl FlowOwnerUpdates {
    /// Returns all seven signed updates in native offset order; e.g. `let members = updates.members()`.
    pub fn members(self) -> [i32; 7] {
        self.members
    }
    /// Reads a typed native member projection; e.g. `let state = updates.member(FlowOwnerMember::Member20)`.
    pub fn member(self, member: FlowOwnerMember) -> i32 {
        self.members[member as usize]
    }
}
impl FlowOwnerState {
    /// Reports the exact legacy all-ones gate, or unconditional modern updates; e.g. `let enabled = state.updates_enabled()`.
    pub fn updates_enabled(&self) -> bool {
        self.kind == FlowOwnerStateKind::Modern434
            || wire::unsigned_at(&self.wire.decoded, 0) != u32::MAX
    }
    /// Projects native updates without executing them, preserving disabled absence; e.g. `let updates = state.updates()`.
    pub fn updates(&self) -> Option<FlowOwnerUpdates> {
        if !self.updates_enabled() {
            return None;
        }
        let members = match self.kind {
            FlowOwnerStateKind::Legacy122 => self.legacy_updates(),
            FlowOwnerStateKind::Modern434 => std::array::from_fn(|index| {
                if index == 6 {
                    return self.wire.decoded[28] as i32;
                }
                wire::signed_at(&self.wire.decoded, 4 + index * 4)
            }),
        };
        Some(FlowOwnerUpdates { members })
    }
    fn legacy_updates(&self) -> [i32; 7] {
        let raw = &self.wire.decoded;
        [
            wire::signed_at(raw, 20),
            wire::signed_at(raw, 28),
            wire::signed_at(raw, 4),
            wire::signed_at(raw, 32),
            wire::signed_at(raw, 8),
            -1,
            0,
        ]
    }
}
