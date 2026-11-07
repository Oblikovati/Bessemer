use bessemer::display_lists::{DisplayVersion, DisplayViewLimits};
#[path = "view_vectors.rs"]
mod framing;
pub const LIMITS: DisplayViewLimits = DisplayViewLimits {
    maximum_bytes: 1_000_000,
    maximum_views: 100,
    maximum_nested_entries: 1000,
};
const VECTORS: &[u8] = include_bytes!("../fixtures/display-lists/spatial-views.bin");
pub fn vectors() -> Vec<(DisplayVersion, u8, &'static [u8])> {
    framing::read(VECTORS, 51)
}
