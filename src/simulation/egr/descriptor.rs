use super::SimulationEgrArchive;
use std::sync::Arc;

/// Assigned descriptor bytes only, without an in-place edit or complete native save claim.
#[derive(Debug, Clone)]
pub struct SimulationEgrDescriptorProjection {
    /// Complete immutable original image and revision.
    pub source: SimulationEgrArchive,
    /// Original tag/width pairs, without rejecting noncanonical tags.
    pub original_descriptor_bytes: Arc<[u8]>,
    /// Writer-assigned pairs: tag one plus each original positive width.
    pub projected_descriptor_bytes: Arc<[u8]>,
    /// Every original alignment byte after the descriptors.
    pub alignment_bytes: Arc<[u8]>,
    /// Whether assigned descriptor bytes differ, without mutating the original table.
    pub descriptor_bytes_changed: bool,
    /// Always false: header reuse, directory writes and native error handling are not projected.
    pub complete_native_save: bool,
}

impl SimulationEgrArchive {
    /// Project only the observed descriptor assignments, e.g. `table.descriptor_writer_projection()`; the input is unchanged.
    pub fn descriptor_writer_projection(&self) -> SimulationEgrDescriptorProjection {
        let original = &self.bytes[128..128 + self.layout.fields.len() * 8];
        let projected: Vec<_> = self
            .layout
            .fields
            .iter()
            .flat_map(|field| [1u32, field.width as u32])
            .flat_map(u32::to_le_bytes)
            .collect();
        SimulationEgrDescriptorProjection {
            source: self.clone(),
            descriptor_bytes_changed: projected != original,
            original_descriptor_bytes: original.into(),
            projected_descriptor_bytes: projected.into(),
            alignment_bytes: self.field_alignment().into(),
            complete_native_save: false,
        }
    }
}
