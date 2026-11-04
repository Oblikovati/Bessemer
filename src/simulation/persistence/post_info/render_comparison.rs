use super::SimulationPostInfo;
use crate::Result;

impl SimulationPostInfo {
    /// Reproduce only the named four-word comparison, not all rendering dependencies; e.g. `info.diff_ogl_render(&other)?`.
    pub fn diff_ogl_render(&self, other: &Self) -> Result<bool> {
        let left = self.header_view()?.raw_words;
        let right = other.header_view()?.raw_words;
        Ok([1, 2, 4, 3]
            .into_iter()
            .any(|index| left[index] != right[index]))
    }
}
