use super::*;

pub(super) fn encode_fields(
    fields: &[ComposerSceneField],
    limits: ComposerSceneLimits,
) -> Result<Vec<u8>> {
    budget("stored fields", fields.len(), limits.maximum_fields)?;
    let total = fields.iter().try_fold(0usize, |total, field| {
        total.checked_add(value_size(&field.value)?).ok_or_else(|| {
            invalid(format!(
                "field bytes after {total}; expected representable complete stream size"
            ))
        })
    })?;
    budget("decoded bytes", total, limits.maximum_decoded_bytes)?;
    let mut bytes = Vec::with_capacity(total);
    for field in fields {
        append_value(&mut bytes, &field.value);
    }
    Ok(bytes)
}

pub(super) fn value_size(value: &ComposerSceneValue) -> Result<usize> {
    match value {
        ComposerSceneValue::Byte(_) => Ok(1),
        ComposerSceneValue::Word(_) => Ok(4),
        ComposerSceneValue::Wide(_) | ComposerSceneValue::Double(_) => Ok(8),
        ComposerSceneValue::Text(text) => checked_size(text.units.len(), 2, 2),
        ComposerSceneValue::Words(words) => checked_size(words.len(), 4, 0),
        ComposerSceneValue::Vector(block) => Ok(block.coverage().wire_bytes),
        ComposerSceneValue::Indices(block) => Ok(block.coverage().wire_bytes),
        ComposerSceneValue::SizedExtension(bytes) => Ok(bytes.len()),
    }
}

fn checked_size(count: usize, width: usize, extra: usize) -> Result<usize> {
    count
        .checked_mul(width)
        .and_then(|size| size.checked_add(extra))
        .ok_or_else(|| {
            invalid(format!(
                "field size {count} * {width} + {extra}; expected representable wire length"
            ))
        })
}

fn append_value(bytes: &mut Vec<u8>, value: &ComposerSceneValue) {
    match value {
        ComposerSceneValue::Byte(word) => bytes.push(*word),
        ComposerSceneValue::Word(word) => bytes.extend_from_slice(&word.to_le_bytes()),
        ComposerSceneValue::Wide(word) => bytes.extend_from_slice(&word.to_le_bytes()),
        ComposerSceneValue::Double(word) => bytes.extend_from_slice(&word.bits().to_le_bytes()),
        ComposerSceneValue::Text(text) => append_text(bytes, text),
        ComposerSceneValue::Words(words) => {
            for word in words.iter() {
                bytes.extend_from_slice(&word.to_le_bytes());
            }
        }
        ComposerSceneValue::Vector(block) => bytes.extend_from_slice(&block.to_bytes()),
        ComposerSceneValue::Indices(block) => bytes.extend_from_slice(&block.to_bytes()),
        ComposerSceneValue::SizedExtension(extension) => bytes.extend_from_slice(extension),
    }
}

fn append_text(bytes: &mut Vec<u8>, text: &ComposerSceneText) {
    for unit in text.units.iter() {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.extend_from_slice(&text.terminator.word().to_le_bytes());
}

pub(super) fn derive_sizes(fields: &mut [ComposerSceneField], rules: &[SizeRule]) -> Result<()> {
    for rule in rules {
        let length = rule_size(
            &fields[rule.start_field..rule.end_field],
            rule.omit_text_terminators,
        )?;
        let word = u32::try_from(length)
            .map_err(|_| invalid(format!("bounded size {length}; expected u32")))?;
        fields[rule.size_field].value = ComposerSceneValue::Word(word);
    }
    Ok(())
}

fn rule_size(fields: &[ComposerSceneField], omit_terminators: bool) -> Result<usize> {
    fields.iter().try_fold(0usize, |total, field| {
        let omitted =
            usize::from(omit_terminators && matches!(field.value, ComposerSceneValue::Text(_))) * 2;
        let length = value_size(&field.value)? - omitted;
        total.checked_add(length).ok_or_else(|| {
            invalid(format!(
                "bounded span {total} + {length}; expected representable size"
            ))
        })
    })
}
