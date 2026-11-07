use crate::{Result, simulation::invalid};

/// Separate complete stored bodies; child payloads never become inline group bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationBeamGroupKind {
    /// Group header and direct reference container.
    JointGroup,
    /// Individual joint with historical inclusive reference gates.
    Joint,
    /// Ordered adjacency pairs with an optional independently enumerated child count.
    JointList {
        /// Independently enumerated direct child count; None requests no agreement check.
        expected_joint_count: Option<usize>,
    },
    /// Word/text header only; no native fixture or complete storage-tree acceptance.
    BeamFolder,
}

impl SimulationBeamGroupKind {
    pub(super) fn expected_count(self) -> Option<usize> {
        match self {
            Self::JointList {
                expected_joint_count,
            } => expected_joint_count,
            _ => None,
        }
    }

    pub(super) fn normalized(self) -> Result<Self> {
        if self.expected_count().is_some_and(|count| count > 10000) {
            return Err(invalid(
                "beam JointList expected child count",
                format!("{:?}; expected None or 0..=10000", self.expected_count()),
            ));
        }
        Ok(self)
    }
}

/// Exact stored-profile scope states whether independent child-count agreement was checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationBeamGroupCheckMode {
    /// Complete bytes, shape and explicit optional child-count agreement.
    ExactStoredProfile {
        /// Independently supplied count that matched, or None when not requested.
        expected_joint_count: Option<usize>,
    },
    /// Complete result plus exact isolated preservation, with the same count scope.
    ExactStoredProfileAndPreservation {
        /// Original independent count scope retained through isolated editing.
        expected_joint_count: Option<usize>,
    },
}
