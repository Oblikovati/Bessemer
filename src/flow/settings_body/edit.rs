use super::*;

impl FlowSettingsBody {
    /// Edits one present same-shape member on a copy; e.g. `let edit = body.replace_member(member, value)?`.
    pub fn replace_member(
        &self,
        member: FlowSettingsBodyMember,
        value: FlowSettingsBodyValue,
    ) -> Result<FlowCarrierEdit> {
        let field = self.field(member).ok_or_else(|| missing_member(member))?;
        if std::mem::discriminant(&field.value) != std::mem::discriminant(&value) {
            return Err(body_failure(
                FailureClass::Invalid,
                format!(
                    "member {member:?} value {value:?}; expected representation like {:?}",
                    field.value
                ),
            ));
        }
        let id = self.record.as_ref().ok_or_else(|| missing_member(member))?;
        let mut payload = self
            .payload()
            .ok_or_else(|| missing_member(member))?
            .to_vec();
        let bytes = wire::value_bytes(value);
        payload[field.offset..field.offset + bytes.len()].copy_from_slice(&bytes);
        let edit = self.source.replace_payload(id, &payload)?;
        verify_edit(&edit, self.runtime_integer_slot_88, member, value, &payload)?;
        Ok(edit)
    }
}

fn verify_edit(
    edit: &FlowCarrierEdit,
    context: Option<i32>,
    member: FlowSettingsBodyMember,
    value: FlowSettingsBodyValue,
    payload: &[u8],
) -> Result<()> {
    let reopened = edit.result.settings_body(context)?;
    if reopened.payload() == Some(payload)
        && reopened
            .field(member)
            .is_some_and(|field| field.value == value)
    {
        return Ok(());
    }
    Err(body_failure(
        FailureClass::Invalid,
        format!(
            "reopened member {member:?} disagrees; expected {value:?} and complete patched payload"
        ),
    ))
}

fn missing_member(member: FlowSettingsBodyMember) -> BessemerError {
    body_failure(
        FailureClass::Missing,
        format!(
            "member {member:?} absent; expected a physically stored member in the selected versioned record 511"
        ),
    )
}
