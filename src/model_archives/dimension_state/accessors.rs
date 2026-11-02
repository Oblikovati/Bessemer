use super::ModelDimensionStoredState;

impl ModelDimensionStoredState {
    /// Retains the complete signed stored mask, including every unrelated bit.
    /// ```no_run
    /// # fn example(dimension: &bessemer::model_archives::ModelDimensionStoredState) {
    /// let raw = dimension.mask();
    /// # }
    /// ```
    pub fn mask(&self) -> i32 {
        self.mask
    }

    /// Reads the stored flag without applying native angle-range checks.
    /// ```no_run
    /// # fn example(dimension: &bessemer::model_archives::ModelDimensionStoredState) {
    /// assert_eq!(dimension.enforce_bend_angle_range(), dimension.mask() & 2 != 0);
    /// # }
    /// ```
    pub fn enforce_bend_angle_range(&self) -> bool {
        self.mask & 2 != 0
    }

    /// Reads the three stored bits, retaining zero without advancing the native counter.
    /// ```no_run
    /// # fn example(dimension: &bessemer::model_archives::ModelDimensionStoredState) {
    /// assert!(dimension.fillet_ref_point_change_number() <= 7);
    /// # }
    /// ```
    pub fn fillet_ref_point_change_number(&self) -> u8 {
        ((self.mask as u32 >> 2) & 7) as u8
    }

    /// Preserves every bit outside the generic dimension roles exposed by this view.
    /// ```no_run
    /// # fn example(dimension: &bessemer::model_archives::ModelDimensionStoredState) {
    /// assert_eq!(dimension.mask_unresolved_bits(), dimension.mask() as u32 & 0xffffffe1);
    /// # }
    /// ```
    pub fn mask_unresolved_bits(&self) -> u32 {
        self.mask as u32 & 0xffffffe1
    }

    /// Reads the signed stored identifier without imposing index or uniqueness constraints.
    /// ```no_run
    /// # fn example(dimension: &bessemer::model_archives::ModelDimensionStoredState) {
    /// let signed_identifier = dimension.redundant_id();
    /// # }
    /// ```
    pub fn redundant_id(&self) -> i32 {
        self.redundant_id
    }
}
