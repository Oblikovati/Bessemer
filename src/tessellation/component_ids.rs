use super::TessellationDirectory;
use std::collections::BTreeSet;

impl TessellationDirectory {
    /// Collects literal component declarations once, deduplicating text without changing stored records.
    ///
    /// ```no_run
    /// # fn example(directory: &bessemer::tessellation::TessellationDirectory) {
    /// let component_ids = directory.component_ids();
    /// # }
    /// ```
    pub fn component_ids(&self) -> BTreeSet<String> {
        self.contents()
            .entries
            .iter()
            .map(|entry| entry.component_id.text().to_owned())
            .collect()
    }
}
