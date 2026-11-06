use super::*;
use std::sync::Arc;

/// A width or row count admitted once to the native nonnegative signed domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFieldCount(u32);

impl TryFrom<i64> for FlowFieldCount {
    type Error = BessemerError;
    fn try_from(value: i64) -> Result<Self> {
        if (0..=i64::from(i32::MAX)).contains(&value) {
            return Ok(Self(value as u32));
        }
        Err(field_invalid(format!(
            "field count {value}; expected zero through {}",
            i32::MAX
        )))
    }
}

impl FlowFieldCount {
    /// Returns the admitted count; e.g. `let width = count.value()`.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Immutable external legacy layout; expected table presence is distinct from wildcard entries.
#[derive(Clone, Debug)]
pub struct FlowLegacyFieldLayout {
    table_count: FlowFieldCount,
    widths: Arc<[FlowFieldCount]>,
    expected: Option<Arc<[Option<FlowFieldCount>]>>,
}

impl FlowLegacyFieldLayout {
    /// Admits matching optional checks without inferring object order; e.g. `let layout = FlowLegacyFieldLayout::new(count, widths, None)?`.
    pub fn new(
        table_count: FlowFieldCount,
        widths: Vec<FlowFieldCount>,
        expected: Option<Vec<Option<FlowFieldCount>>>,
    ) -> Result<Self> {
        if let Some(expected) = &expected
            && expected.len() != table_count.value() as usize
        {
            return Err(field_invalid(format!(
                "expected width-table length {}; expected {} entries",
                expected.len(),
                table_count.value()
            )));
        }
        Ok(Self {
            table_count,
            widths: widths.into(),
            expected: expected.map(Arc::from),
        })
    }
    /// Returns the externally counted stored table length; e.g. `let count = layout.width_table_count()`.
    pub fn width_table_count(&self) -> FlowFieldCount {
        self.table_count
    }
    /// Borrows widths in supplied global-object order; e.g. `let widths = layout.object_widths()`.
    pub fn object_widths(&self) -> &[FlowFieldCount] {
        &self.widths
    }
    /// Retains absent checks, present empty checks and wildcard entries; e.g. `let expected = layout.expected_width_table()`.
    pub fn expected_width_table(&self) -> Option<&[Option<FlowFieldCount>]> {
        self.expected.as_deref()
    }
}

/// One region's explicit runtime storage context; these flags and rows are not file fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFieldRegionContext {
    /// Declared runtime rows, including large inactive counts.
    pub row_count: FlowFieldCount,
    /// Expected stored width.
    pub width: FlowFieldCount,
    /// Explicit compound native storage branch.
    pub active: bool,
}

/// Immutable ordered region context, with a native signed region-count boundary.
#[derive(Clone, Debug)]
pub struct FlowRegionFieldLayout(Arc<[FlowFieldRegionContext]>);

impl FlowRegionFieldLayout {
    /// Admits the ordered region descriptors once; e.g. `let layout = FlowRegionFieldLayout::new(regions)?`.
    pub fn new(regions: Vec<FlowFieldRegionContext>) -> Result<Self> {
        if regions.len() > i32::MAX as usize {
            return Err(field_invalid(format!(
                "region context length {}; expected at most {}",
                regions.len(),
                i32::MAX
            )));
        }
        Ok(Self(regions.into()))
    }
    /// Borrows complete immutable descriptors; e.g. `let regions = layout.regions()`.
    pub fn regions(&self) -> &[FlowFieldRegionContext] {
        &self.0
    }
}

/// Required layout family, independently of version and transport.
#[derive(Clone, Debug)]
pub enum FlowFieldLayout {
    /// Versions five and six: externally counted table and global-object order.
    Legacy(FlowLegacyFieldLayout),
    /// Versions seven and later: matching stored region counts/widths and runtime row/storage context.
    Regions(FlowRegionFieldLayout),
}

/// Independent body transport, compatibility gate, layout and allocation budgets.
#[derive(Clone, Debug)]
pub struct FlowFieldValueOptions {
    /// Raw/zlib applies only after the four raw version bytes.
    pub body_compression: FlowLegacyTableTransport,
    /// Native restriction rejects versions above seven before body decoding.
    pub compatibility_restriction: bool,
    /// Explicit immutable runtime layout.
    pub layout: FlowFieldLayout,
    /// Complete decoded-body bound; zero is valid for an empty legacy body.
    pub maximum_decoded_body_bytes: usize,
    /// Positive maximum objects/regions, including zero-width/inactive positions.
    pub maximum_items: usize,
    /// Positive maximum physically stored legacy width-table words.
    pub maximum_table_words: usize,
    /// Positive maximum actual raw wide patterns.
    pub maximum_wide_words: usize,
}
