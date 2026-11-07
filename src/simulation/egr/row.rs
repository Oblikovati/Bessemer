use super::{
    Result, SimulationEgrArchive, SimulationEgrCheckMode, SimulationEgrRowId,
    SimulationSolverElementCode, SimulationSolverNameState, require, wire,
};
use std::sync::Arc;

/// Loaded-width evidence normalized once for the exact conditional 32-by-four-byte row interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationEgrRowProfile(());

impl SimulationEgrRowProfile {
    /// Establish the explicit loaded-width profile, e.g. `SimulationEgrRowProfile::from_loaded_widths(&[4; 32])?`.
    pub fn from_loaded_widths(widths: &[usize]) -> Result<Self> {
        require(
            widths.len() == 32,
            "EGR conditional row profile",
            format!(
                "{} fields; expected exactly 32 explicitly loaded widths",
                widths.len()
            ),
        )?;
        require(
            widths.iter().all(|width| *width == 4),
            "EGR conditional row widths",
            format!("{widths:?}; expected exactly 32 four-byte fields"),
        )?;
        Ok(Self(()))
    }
}

/// Complete conditional row image, optionally retaining its real complete file and live-row parent.
#[derive(Debug, Clone)]
pub struct SimulationEgrRowImage {
    bytes: Arc<[u8; 128]>,
    profile: SimulationEgrRowProfile,
    parent: Option<(SimulationEgrArchive, SimulationEgrRowId)>,
}

/// Attributed integer values and exact retained row tail; physical parameter meanings remain open.
#[derive(Debug, Clone)]
pub struct SimulationEgrRowView {
    /// Complete original row and optional real file parent.
    pub source: SimulationEgrRowImage,
    /// Signed first stored word, distinct from the supplied logical ordinal.
    pub group_identifier: i32,
    /// Signed second stored word under the established four-byte profile.
    pub solver_type_code: SimulationSolverElementCode,
    /// Complete known/null/unknown lookup state; not a GEN field assignment.
    pub name_state: SimulationSolverNameState,
    /// Eight signed words in original stored order, without floating-point conversion.
    pub parameter_words: [i32; 8],
    /// Remaining 88 bytes retained verbatim without added meanings.
    pub unassigned_tail: [u8; 88],
    /// Exact conditional word extraction and raw-tail preservation scope.
    pub check_mode: SimulationEgrCheckMode,
    /// Always false: complete meanings for all fields and values remain unassigned.
    pub semantics_complete: bool,
}

impl SimulationEgrRowImage {
    /// Capture a complete row only with explicit normalized widths, e.g. `SimulationEgrRowImage::from_bytes(raw, profile)?`.
    pub fn from_bytes(bytes: &[u8], profile: SimulationEgrRowProfile) -> Result<Self> {
        Ok(Self {
            bytes: Arc::new(wire::fixed::<128>(bytes, "EGR conditional row")?),
            profile,
            parent: None,
        })
    }
    /// Complete original row bytes, e.g. `row.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Normalized explicit loaded-width evidence, e.g. `row.profile()`.
    pub fn profile(&self) -> SimulationEgrRowProfile {
        self.profile
    }
    /// Real retained file parent when available, e.g. `row.source_table()`; standalone inputs do not invent one.
    pub fn source_table(&self) -> Option<&SimulationEgrArchive> {
        self.parent.as_ref().map(|(source, _)| source)
    }
    /// Real typed live-row parent, e.g. `row.source_row_id()`.
    pub fn source_row_id(&self) -> Option<&SimulationEgrRowId> {
        self.parent.as_ref().map(|(_, id)| id)
    }
    /// Inspect only the assigned integer fields, e.g. `row.view()`; the full row remains retained.
    pub fn view(&self) -> SimulationEgrRowView {
        let words = first_words(&self.bytes);
        let code = SimulationSolverElementCode::from_signed(words[1]);
        SimulationEgrRowView {
            source: self.clone(),
            group_identifier: words[0],
            solver_type_code: code,
            name_state: code.lookup(),
            parameter_words: std::array::from_fn(|index| words[index + 2]),
            unassigned_tail: std::array::from_fn(|index| self.bytes[index + 40]),
            check_mode: SimulationEgrCheckMode::ExactConditionalFourByteRowValues,
            semantics_complete: false,
        }
    }
}

impl SimulationEgrArchive {
    /// Capture a real live row only when actual loaded widths establish its interpretation, e.g. `table.conditional_row(&id)?`.
    pub fn conditional_row(&self, id: &SimulationEgrRowId) -> Result<SimulationEgrRowImage> {
        let bytes = self.live_row(id)?;
        let widths: Vec<_> = self.layout.fields.iter().map(|field| field.width).collect();
        let profile =
            SimulationEgrRowProfile::from_loaded_widths(&widths).map_err(|mut error| {
                error.class = crate::FailureClass::Unsupported;
                error
            })?;
        let mut row = SimulationEgrRowImage::from_bytes(bytes, profile)?;
        row.parent = Some((self.clone(), id.clone()));
        Ok(row)
    }
}

fn first_words(bytes: &[u8; 128]) -> [i32; 10] {
    std::array::from_fn(|index| {
        let start = index * 4;
        i32::from_le_bytes([
            bytes[start],
            bytes[start + 1],
            bytes[start + 2],
            bytes[start + 3],
        ])
    })
}
