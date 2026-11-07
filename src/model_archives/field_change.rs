use super::{ModelFieldChange, ModelFieldHandle, ModelFieldRelation};

impl ModelFieldChange {
    /// Returns the parser-derived path; handles retain the exact physical occurrence.
    ///
    /// ```no_run
    /// # fn example(change: &bessemer::model_archives::ModelFieldChange) {
    /// let path = change.path();
    /// # }
    /// ```
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns structural creation, deletion or derivation without inferring geometric history.
    ///
    /// ```no_run
    /// # fn example(change: &bessemer::model_archives::ModelFieldChange) {
    /// let relation = change.relation();
    /// # }
    /// ```
    pub fn relation(&self) -> ModelFieldRelation {
        self.relation
    }

    /// Borrows every source parent; created occurrences have none.
    ///
    /// ```no_run
    /// # fn example(change: &bessemer::model_archives::ModelFieldChange) {
    /// let parents = change.parents();
    /// # }
    /// ```
    pub fn parents(&self) -> &[ModelFieldHandle] {
        &self.parents
    }

    /// Borrows the result occurrence; deleted fields have none.
    ///
    /// ```no_run
    /// # fn example(change: &bessemer::model_archives::ModelFieldChange) {
    /// let result = change.result();
    /// # }
    /// ```
    pub fn result(&self) -> Option<&ModelFieldHandle> {
        self.result.as_ref()
    }
}
