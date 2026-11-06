use super::*;

struct SelectedField {
    group: usize,
    member: Option<usize>,
    offset: usize,
    before: FlowGroupValue,
    after: FlowGroupValue,
}

impl FlowGroupedRecord {
    /// Replaces one existing wire field on a copy; e.g. `let edit = record.replace_field(0, FlowGroupFieldEdit::Word { member: 0, value: 7 })?`.
    pub fn replace_field(
        &self,
        group: usize,
        request: FlowGroupFieldEdit,
    ) -> Result<FlowGroupEdit> {
        let field = select_field(&self.0, group, request)?;
        let mut expected = self.0.decoded.to_vec();
        let value = fields::value_bytes(field.after);
        expected[field.offset..field.offset + value.len()].copy_from_slice(&value);
        let carrier = edited_carrier(self, &field, &expected)?;
        let result = result_record(self, &carrier)?;
        verify_result(&result, &expected)?;
        let history = field_history(self, &result, &field);
        Ok(FlowGroupEdit {
            source: self.clone(),
            result,
            carrier,
            history,
        })
    }
}

fn select_field(
    revision: &GroupRevision,
    group: usize,
    request: FlowGroupFieldEdit,
) -> Result<SelectedField> {
    let span = revision.spans.get(group).ok_or_else(|| {
        group_failure(
            "group index",
            FailureClass::Invalid,
            format!("group {group}; expected below {}", revision.spans.len()),
        )
    })?;
    let (member, after) = requested_value(request);
    let offset = selected_offset(revision.version, span, member, after)?;
    let before = fields::read_value(&revision.decoded, offset, after);
    Ok(SelectedField {
        group,
        member,
        offset,
        before,
        after,
    })
}

fn requested_value(request: FlowGroupFieldEdit) -> (Option<usize>, FlowGroupValue) {
    match request {
        FlowGroupFieldEdit::Word { member, value } => (Some(member), FlowGroupValue::Word(value)),
        FlowGroupFieldEdit::WideWord { member, value } => {
            (Some(member), FlowGroupValue::WideWord(value))
        }
        FlowGroupFieldEdit::Flag { member, value } => (Some(member), FlowGroupValue::Flag(value)),
        FlowGroupFieldEdit::Auxiliary(value) => (None, FlowGroupValue::Auxiliary(value)),
    }
}

fn selected_offset(
    version: FlowGroupVersion,
    span: &GroupSpan,
    member: Option<usize>,
    value: FlowGroupValue,
) -> Result<usize> {
    if matches!(value, FlowGroupValue::Auxiliary(_)) {
        return auxiliary_offset(version, span);
    }
    let member = member.ok_or_else(|| {
        group_failure(
            "member index",
            FailureClass::Invalid,
            format!("missing member for {value:?}; expected a member ordinal"),
        )
    })?;
    if member >= span.members {
        return Err(group_failure(
            "member index",
            FailureClass::Invalid,
            format!("member {member}; expected below {}", span.members),
        ));
    }
    let field = member_field(version, value)?;
    Ok(fields::member_offset(version, span, member, field))
}

fn auxiliary_offset(version: FlowGroupVersion, span: &GroupSpan) -> Result<usize> {
    if version == FlowGroupVersion::V4 {
        return Ok(span.start + 4);
    }
    Err(group_failure(
        "auxiliary field",
        FailureClass::Missing,
        format!(
            "version {}; expected version 4 with a stored auxiliary i32",
            version.wire_value()
        ),
    ))
}

fn member_field(version: FlowGroupVersion, value: FlowGroupValue) -> Result<fields::MemberField> {
    match value {
        FlowGroupValue::Word(_) => Ok(fields::MemberField::Word),
        FlowGroupValue::WideWord(_) => Ok(fields::MemberField::WideWord),
        FlowGroupValue::Flag(_) if fields::has_flags(version) => Ok(fields::MemberField::Flag),
        _ => Err(group_failure(
            "member field",
            FailureClass::Missing,
            format!(
                "{value:?} in version {}; expected an existing word/wide field or a version-5/6 wire flag",
                version.wire_value()
            ),
        )),
    }
}

fn edited_carrier(
    source: &FlowGroupedRecord,
    field: &SelectedField,
    decoded: &[u8],
) -> Result<FlowCarrierEdit> {
    if field.before == field.after {
        let payload = super::super::payload_codec::record_payload(
            source.source(),
            source.record(),
            404,
            "Flow record 404",
        )?;
        return source.source().replace_payload(source.record(), payload);
    }
    let payload = match source.compression() {
        FlowGroupCompression::Raw => decoded.to_vec(),
        FlowGroupCompression::Zlib => {
            super::super::payload_codec::encode_zlib(decoded, "Flow record 404")?
        }
    };
    source.source().replace_payload(source.record(), &payload)
}

fn result_record(
    source: &FlowGroupedRecord,
    carrier: &FlowCarrierEdit,
) -> Result<FlowGroupedRecord> {
    let ordinal = source.record().ordinal();
    let record = carrier.result().records().nth(ordinal).ok_or_else(|| {
        group_failure(
            "result occurrence",
            FailureClass::Integrity,
            format!("missing ordinal {ordinal}; expected unchanged record count"),
        )
    })?;
    carrier
        .result()
        .record_404_groups(&record.id, source.0.options)
}

fn verify_result(result: &FlowGroupedRecord, expected: &[u8]) -> Result<()> {
    if result.decoded_bytes() == expected {
        return Ok(());
    }
    Err(group_failure(
        "result bytes",
        FailureClass::Integrity,
        format!(
            "{} result bytes differ from {} expected bytes; expected only the selected field to change",
            result.decoded_bytes().len(),
            expected.len()
        ),
    ))
}

fn field_history(
    source: &FlowGroupedRecord,
    result: &FlowGroupedRecord,
    field: &SelectedField,
) -> FlowGroupFieldHistory {
    FlowGroupFieldHistory {
        source_record: source.record().clone(),
        result_record: result.record().clone(),
        group: field.group,
        member: field.member,
        before: field.before,
        after: field.after,
        relation: if field.before == field.after {
            FlowPayloadRelation::Copied
        } else {
            FlowPayloadRelation::Derived
        },
    }
}
