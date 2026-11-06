use super::*;

impl FlowContextCounts {
    /// Derives only the explicitly initialized-absent-map count; e.g. `let binding = counts.initialize_absent_map(FlowAbsentMapInitialization::InitializeAbsentMap)?`.
    pub fn initialize_absent_map(
        &self,
        assumption: FlowAbsentMapInitialization,
    ) -> Result<FlowInitializedCountBinding> {
        let total = self.mathematical_total();
        if total > i32::MAX as u64 {
            return Err(count_failure(
                FailureClass::Invalid,
                format!(
                    "mathematical count {total}, native wrapped count {}; expected initialized map count 0 through {}",
                    self.native_total(),
                    i32::MAX
                ),
            ));
        }
        Ok(FlowInitializedCountBinding {
            counts: self.clone(),
            count: total as u32,
            assumption,
        })
    }
}

impl FlowInitializedCountBinding {
    /// Reads a selected record 433 under this explicit conditional count; e.g. `let bound = binding.read_record_433(&carrier, &id, limits)?`.
    pub fn read_record_433(
        &self,
        source: &FlowCarrier,
        id: &FlowRecordId,
        limits: FlowBoundArrayLimits,
    ) -> Result<FlowBoundMapArrays> {
        let options = FlowArrayOptions {
            maximum_decoded_bytes: limits.maximum_decoded_bytes,
            maximum_elements: limits.maximum_elements,
            expected_count: Some(self.count),
        };
        let arrays = source.record_433_arrays(id, options)?;
        Ok(FlowBoundMapArrays {
            binding: self.clone(),
            arrays,
        })
    }
}
