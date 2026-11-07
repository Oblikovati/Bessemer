mod checkpoints;
mod items;
use super::*;

impl FlowFieldValues {
    /// Borrows the legacy width table as exact signed-word patterns; e.g. `let table = fields.width_table()`.
    pub fn width_table(&self) -> Option<FlowBlockWordArray<'_>> {
        self.legacy_layout().map(|_| FlowBlockWordArray {
            bytes: &self.wire.decoded[..self.table_words * 4],
            start: 0,
        })
    }
    /// Reads one signed stored table word, without interpreting unchecked negatives; e.g. `let width = fields.width_table_word(0)`.
    pub fn width_table_word(&self, index: usize) -> Option<i32> {
        self.width_table()?.raw_word(index).map(|word| word as i32)
    }
    /// Enumerates global-object positions, retaining zero-width items; e.g. `let count = fields.objects().len()`.
    pub fn objects(&self) -> impl ExactSizeIterator<Item = FlowFieldObject<'_>> {
        let widths = self
            .legacy_layout()
            .map_or(&[][..], FlowLegacyFieldLayout::object_widths);
        self.spans
            .iter()
            .zip(widths)
            .enumerate()
            .map(|(ordinal, (span, width))| FlowFieldObject {
                owner: self,
                ordinal,
                width: *width,
                span,
            })
    }
    /// Borrows one global-object position; e.g. `let object = fields.object(0)`.
    pub fn object(&self, ordinal: usize) -> Option<FlowFieldObject<'_>> {
        self.objects().nth(ordinal)
    }
    /// Enumerates region positions, retaining inactive/empty regions; e.g. `let count = fields.regions().len()`.
    pub fn regions(&self) -> impl ExactSizeIterator<Item = FlowFieldRegion<'_>> {
        let regions = self
            .region_layout()
            .map_or(&[][..], FlowRegionFieldLayout::regions);
        self.spans
            .iter()
            .zip(regions)
            .enumerate()
            .map(|(ordinal, (span, context))| FlowFieldRegion {
                owner: self,
                ordinal,
                context: *context,
                span,
            })
    }
    /// Borrows one region position; e.g. `let region = fields.region(0)`.
    pub fn region(&self, ordinal: usize) -> Option<FlowFieldRegion<'_>> {
        self.regions().nth(ordinal)
    }
    /// Reports complete byte counts and actual value coverage; e.g. `let coverage = fields.coverage()`.
    pub fn coverage(&self) -> FlowFieldValueCoverage {
        let legacy = self.legacy_layout();
        FlowFieldValueCoverage {
            mode: self.check_mode(),
            stored_version_bytes: 4,
            stored_body_bytes: self.wire.stored_bytes - 4,
            decoded_body_bytes: self.wire.decoded.len(),
            decoded_payload_bytes: self.wire.decoded.len() + 4,
            width_table_words: self.table_words,
            objects: self.objects().len(),
            regions: self.regions().len(),
            wide_words: self.wide_words,
            expected_width_table_checked: legacy
                .map(|layout| layout.expected_width_table().is_some()),
            expected_region_count_checked: legacy.is_none(),
            expected_region_widths_checked: legacy.is_none(),
            achieved_error: 0,
        }
    }
    /// Borrows native and engineering exclusions; e.g. `let exclusions = fields.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowFieldValueDiagnostic] {
        use FlowFieldValueDiagnostic::*;
        &[
            LayoutContextAssumed,
            NativeContextNotExecuted,
            NativeStorageNotReproduced,
            EngineeringColumnsNotResolved,
            SelectedNumericBindingNotEstablished,
            NativeObjectFormsNotValidated,
            OwnerCompressionContextNotReconstructed,
            NativeApplicationNotChecked,
        ]
    }
    fn legacy_layout(&self) -> Option<&FlowLegacyFieldLayout> {
        match &self.options.layout {
            FlowFieldLayout::Legacy(layout) => Some(layout),
            _ => None,
        }
    }
    fn region_layout(&self) -> Option<&FlowRegionFieldLayout> {
        match &self.options.layout {
            FlowFieldLayout::Regions(layout) => Some(layout),
            _ => None,
        }
    }
    fn check_mode(&self) -> FlowFieldValueCheckMode {
        match self.options.body_compression {
            FlowLegacyTableTransport::Raw => FlowFieldValueCheckMode::ExactRawVersionedBody,
            FlowLegacyTableTransport::Zlib => FlowFieldValueCheckMode::ExactZlibVersionedBody,
        }
    }
}

fn field_wide(wire: &TableWire, span: (usize, usize)) -> FlowTableWideArray<'_> {
    FlowTableWideArray {
        bytes: &wire.decoded[span.0..span.1],
        start: span.0,
    }
}
