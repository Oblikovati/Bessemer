use super::{ModelNodeStoredFlag, ModelNodeStoredState};
use crate::Result;

impl ModelNodeStoredState {
    /// Returns the complete signed word without clearing coexisting states.
    /// ```no_run
    /// # fn example(node: &bessemer::model_archives::ModelNodeStoredState) {
    /// let signed = node.update_state();
    /// # }
    /// ```
    pub fn update_state(&self) -> i32 {
        self.update_state
    }

    /// Returns every bit of the first stored flags word.
    /// ```no_run
    /// # fn example(node: &bessemer::model_archives::ModelNodeStoredState) {
    /// let raw = node.flags_28();
    /// # }
    /// ```
    pub fn flags_28(&self) -> u32 {
        self.flags_28
    }

    /// Preserves the distinction between an absent word and a stored zero.
    /// ```no_run
    /// # fn example(node: &bessemer::model_archives::ModelNodeStoredState) {
    /// let optional = node.flags_2c();
    /// # }
    /// ```
    pub fn flags_2c(&self) -> Option<u32> {
        self.flags_2c
    }

    /// Identifies the exact supported parameter class, excluding generic nodes.
    /// ```no_run
    /// # fn example(node: &bessemer::model_archives::ModelNodeStoredState) {
    /// let scope = node.parameter_class();
    /// # }
    /// ```
    pub fn parameter_class(&self) -> Option<&'static str> {
        self.parameter_class
    }

    /// Returns update-state bits without documented roles.
    /// ```no_run
    /// # fn example(node: &bessemer::model_archives::ModelNodeStoredState) {
    /// let unexplained = node.update_state_unresolved_bits();
    /// # }
    /// ```
    pub fn update_state_unresolved_bits(&self) -> u32 {
        self.update_state as u32 & 0xfffd3ef8
    }

    /// Keeps every first-word bit outside this node's verified class scope.
    /// ```no_run
    /// # fn example(node: &bessemer::model_archives::ModelNodeStoredState) {
    /// let unexplained = node.flags_28_unresolved_bits();
    /// # }
    /// ```
    pub fn flags_28_unresolved_bits(&self) -> u32 {
        let known = if self.parameter_class.is_some() {
            0xf0000000
        } else {
            0x40000000
        };
        self.flags_28 & !known
    }

    /// Preserves absence and unresolved second-word bits without generic parameter meanings.
    /// ```no_run
    /// # fn example(node: &bessemer::model_archives::ModelNodeStoredState) {
    /// let unexplained = node.flags_2c_unresolved_bits();
    /// # }
    /// ```
    pub fn flags_2c_unresolved_bits(&self) -> Option<u32> {
        let known = u32::from(self.parameter_class.is_some());
        self.flags_2c.map(|word| word & !known)
    }

    /// Reads a stored bit, rejecting parameter-only requests outside their verified scope.
    /// ```no_run
    /// # fn example(node: &bessemer::model_archives::ModelNodeStoredState) -> bessemer::Result<()> {
    /// use bessemer::model_archives::ModelNodeStoredFlag;
    /// let stored = node.stored_flag(ModelNodeStoredFlag::NeedsUpdate)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn stored_flag(&self, flag: ModelNodeStoredFlag) -> Result<bool> {
        let (word, _, mask) = self.selected_word(flag)?;
        Ok(word & mask != 0)
    }
}
