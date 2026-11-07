use super::*;

impl FlowFeatureIdentities {
    /// Edits one existing object on a copy; e.g. `let edit = list.replace_object_field(0, FlowFeatureFieldEdit::State(255))?`.
    pub fn replace_object_field(
        &self,
        index: usize,
        edit: FlowFeatureFieldEdit,
    ) -> Result<FlowFeatureEdit> {
        let object = self
            .objects
            .get(index)
            .ok_or_else(|| index_error("object", index, self.objects.len()))?;
        let payload = object_payload(self, object, edit)?;
        publish_edit(self, &payload)
    }
    /// Edits one physically stored tail value; e.g. `let edit = list.replace_tail_value(0, -1)?`.
    pub fn replace_tail_value(&self, index: usize, value: i32) -> Result<FlowFeatureEdit> {
        let FlowFeatureTail::Stored(values) = &self.tail else {
            return Err(feature_failure(
                FailureClass::Missing,
                format!(
                    "tail index {index} in legacy version {}; expected a physically stored tail",
                    self.version
                ),
            ));
        };
        if index >= values.len() {
            return Err(index_error("tail", index, values.len()));
        }
        let offset = self.payload.len() - values.len() * 4 + index * 4;
        publish_edit(
            self,
            &scalar_payload(&self.payload, offset, &value.to_le_bytes()),
        )
    }
}

fn object_payload(
    list: &FlowFeatureIdentities,
    object: &FlowFeatureIdentity,
    edit: FlowFeatureFieldEdit,
) -> Result<Vec<u8>> {
    Ok(match edit {
        FlowFeatureFieldEdit::HeaderVersion(word) => {
            scalar_payload(&list.payload, object.start, &word.to_le_bytes())
        }
        FlowFeatureFieldEdit::TypeCode(word) => {
            scalar_payload(&list.payload, object.start + 4, &word.to_le_bytes())
        }
        FlowFeatureFieldEdit::BodyVersion(word) => {
            scalar_payload(&list.payload, object.start + 8, &word.to_le_bytes())
        }
        FlowFeatureFieldEdit::State(byte) => scalar_payload(&list.payload, object.end - 1, &[byte]),
        FlowFeatureFieldEdit::Text(units) => super::text_edit::payload(list, object, &units)?,
    })
}

fn scalar_payload(original: &[u8], offset: usize, replacement: &[u8]) -> Vec<u8> {
    let mut payload = original.to_vec();
    payload[offset..offset + replacement.len()].copy_from_slice(replacement);
    payload
}

fn publish_edit(source: &FlowFeatureIdentities, payload: &[u8]) -> Result<FlowFeatureEdit> {
    let carrier_edit = source.source.replace_payload(&source.record, payload)?;
    let result = checked_result(&carrier_edit, source.options, payload)?;
    Ok(FlowFeatureEdit {
        source: source.clone(),
        result,
        carrier_edit,
    })
}

fn index_error(label: &str, index: usize, count: usize) -> BessemerError {
    feature_failure(
        FailureClass::Invalid,
        format!("{label} index {index}; expected below {count} in this selected immutable list"),
    )
}

fn checked_result(
    carrier_edit: &FlowCarrierEdit,
    options: FlowFeatureOptions,
    payload: &[u8],
) -> Result<FlowFeatureIdentities> {
    let result = carrier_edit
        .result()
        .feature_identities(options)?
        .ok_or_else(|| {
            feature_failure(
                FailureClass::Missing,
                "edited record 143 absent; expected selected complete identity list".to_owned(),
            )
        })?;
    if result.payload() != payload {
        return Err(feature_failure(
            FailureClass::Invalid,
            format!(
                "reopened payload differs at length {}; expected complete patched {} bytes",
                result.payload().len(),
                payload.len()
            ),
        ));
    }
    Ok(result)
}
