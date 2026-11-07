use super::*;
impl FlowLegacyWordFields {
    /// Projects only record105's completed scalar branches; e.g. `let decision = fields.queries_105()?.slot7`.
    pub fn queries_105(&self) -> Result<Flow105ScalarQueries> {
        if self.profile != FlowLegacyWordProfile::Fixed105 {
            return Err(table_failure(
                self.profile.identifier(),
                FailureClass::Unsupported,
                format!(
                    "profile {:?}; expected unversioned record105 with exactly 320 patterns",
                    self.profile
                ),
            ));
        }
        Ok(Flow105ScalarQueries {
            slot10_nonzero: self.pattern(10)? & 0x7fffffffffffffff != 0,
            slot7: slot7_decision(self.pattern(7)?),
            native_fp_faults_reproduced: false,
            native_virtual_result_reproduced: false,
        })
    }
}
// Completed-branch modeling avoids imposing the process's floating-point exception or denormal state.
fn slot7_decision(bits: u64) -> Flow105Slot7Decision {
    let magnitude = bits & 0x7fffffffffffffff;
    if magnitude & 0x7ff0000000000000 == 0x7ff0000000000000 && magnitude & 0x000fffffffffffff != 0 {
        return Flow105Slot7Decision::VirtualMethod78;
    }
    if bits >> 63 != 0 || bits <= 0x406fa00000000000 {
        return Flow105Slot7Decision::AtOrBelow253;
    }
    if bits > 0x4081a80000000000 {
        return Flow105Slot7Decision::Above565;
    }
    Flow105Slot7Decision::VirtualMethod78
}
impl Flow105Slot7Decision {
    /// Returns only a modeled precheck rejection, never a native method result; e.g. `assert_eq!(Flow105Slot7Decision::Above565.precheck_status(), Some(-1))`.
    pub fn precheck_status(self) -> Option<i32> {
        match self {
            Self::VirtualMethod78 => None,
            Self::Above565 => Some(-1),
            Self::AtOrBelow253 => Some(-2),
        }
    }
    /// Reports which completed branch would call virtual slot0x78; e.g. `let selected = decision.would_call_virtual_78()`.
    pub fn would_call_virtual_78(self) -> bool {
        self == Self::VirtualMethod78
    }
}
