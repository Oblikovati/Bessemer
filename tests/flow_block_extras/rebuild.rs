use super::*;

pub fn decoded(case: &ExtraCase) -> Vec<u8> {
    let mut raw = case.version.to_le_bytes().to_vec();
    for (ordinal, block) in case.blocks.iter().enumerate() {
        assert_eq!(block.ordinal, ordinal as u32);
        assert_eq!(raw.len(), block.start as usize);
        raw.extend_from_slice(&block.token.to_le_bytes());
        append_optional(&mut raw, block.flag, case.version >= 3);
        assert_eq!(
            block.body.is_some(),
            block.token == -1 || block.flag.is_some_and(|f| f != 0)
        );
        if let Some(body) = &block.body {
            append_body(&mut raw, body, case.version);
        }
        assert_eq!(raw.len(), block.end as usize);
    }
    raw
}

fn append_body(raw: &mut Vec<u8>, body: &ExtraBody, version: i32) {
    assert_eq!(raw.len(), body.start as usize);
    raw.push(u8::try_from(body.first.len()).unwrap());
    for item in &body.first {
        append_first(raw, item, version);
    }
    raw.push(u8::try_from(body.second.len()).unwrap());
    for item in &body.second {
        append_second(raw, item);
    }
    raw.push(u8::try_from(body.third.len()).unwrap());
    for item in &body.third {
        append_third(raw, item);
    }
    raw.push(u8::try_from(body.fourth_bytes.len()).unwrap());
    assert_eq!(body.fourth_wide.len(), 3 * body.fourth_bytes.len());
    append_wide(raw, &body.fourth_wide);
    raw.extend_from_slice(&body.fourth_bytes);
    append_body_tail(raw, body, version);
    assert_eq!(raw.len(), body.end as usize);
}

fn append_first(raw: &mut Vec<u8>, item: &ExtraFirst, version: i32) {
    assert_eq!(raw.len(), item.start as usize);
    raw.extend_from_slice(&item.pair);
    append_integers(raw, &item.words);
    append_mixed(raw, &item.mixed, version);
    append_references(raw, item.references, version);
    append_bytes(raw, &item.bytes);
    append_additional(raw, item, version);
    assert_eq!(raw.len(), item.end as usize);
}

fn append_references(raw: &mut Vec<u8>, references: [u64; 2], version: i32) {
    for value in references {
        if version < 5 {
            raw.extend_from_slice(&u32::try_from(value).unwrap().to_le_bytes());
        } else {
            raw.extend_from_slice(&value.to_le_bytes());
        }
    }
}

fn append_mixed(raw: &mut Vec<u8>, mixed: &ExtraMixed, version: i32) {
    assert_eq!(raw.len(), mixed.start as usize);
    assert_eq!(mixed.interleaved, version == 8);
    assert_eq!(mixed.words.len(), mixed.wide.len());
    raw.extend_from_slice(&i32::try_from(mixed.words.len()).unwrap().to_le_bytes());
    if !mixed.interleaved {
        append_integers(raw, &mixed.words);
        append_wide(raw, &mixed.wide);
    } else {
        for (word, wide) in mixed.words.iter().zip(&mixed.wide) {
            raw.extend_from_slice(&word.to_le_bytes());
            raw.extend_from_slice(&wide.to_le_bytes());
        }
    }
    assert_eq!(raw.len(), mixed.end as usize);
}

fn append_second(raw: &mut Vec<u8>, item: &ExtraSecond) {
    assert_eq!(raw.len(), item.start as usize);
    raw.extend_from_slice(&item.pair);
    raw.push(item.tag);
    append_bytes(raw, &item.bytes);
    assert_eq!(raw.len(), item.end as usize);
}

fn append_third(raw: &mut Vec<u8>, item: &ExtraThird) {
    assert_eq!(raw.len(), item.start as usize);
    raw.extend_from_slice(&item.word.to_le_bytes());
    append_wide(raw, &item.wide);
    assert_eq!(raw.len(), item.end as usize);
}

fn append_body_tail(raw: &mut Vec<u8>, body: &ExtraBody, version: i32) {
    raw.push(body.outer);
    append_optional(raw, body.nested_flag, body.outer == 1);
    assert_eq!(body.nested.is_some(), body.nested_flag == Some(1));
    if let Some(rows) = &body.nested {
        assert_eq!(rows.len(), body.fourth_bytes.len());
        for row in rows {
            append_nested_row(raw, row, version);
        }
    }
    append_optional(raw, body.final_flag, body.outer == 1);
    assert_eq!(body.final_values.is_some(), body.final_flag == Some(1));
    if let Some(final_values) = &body.final_values {
        append_final(raw, final_values);
    }
    append_extra_tail(raw, body, version);
}

fn append_nested_row(raw: &mut Vec<u8>, row: &[ExtraNested], version: i32) {
    if version < 5 {
        raw.push(u8::try_from(row.len()).unwrap());
    } else {
        raw.extend_from_slice(&i32::try_from(row.len()).unwrap().to_le_bytes());
    }
    for (ordinal, item) in row.iter().enumerate() {
        append_nested(raw, item, ordinal, version);
    }
}

fn append_nested(raw: &mut Vec<u8>, item: &ExtraNested, ordinal: usize, version: i32) {
    assert_eq!(raw.len(), item.start as usize);
    raw.push(u8::try_from(item.words.len()).unwrap());
    assert_eq!(item.stored_index.is_some(), version >= 6);
    assert_eq!(
        item.native_index,
        item.stored_index.unwrap_or(ordinal as i32)
    );
    if let Some(index) = item.stored_index {
        raw.extend_from_slice(&index.to_le_bytes());
    }
    append_integers(raw, &item.words);
    assert_eq!(item.wide.len(), 3 * item.words.len());
    append_wide(raw, &item.wide);
    assert_eq!(raw.len(), item.end as usize);
}

fn append_final(raw: &mut Vec<u8>, item: &ExtraFinal) {
    raw.extend_from_slice(&item.word.to_le_bytes());
    raw.extend_from_slice(&i32::try_from(item.values.len()).unwrap().to_le_bytes());
    append_integers(raw, &item.values);
}

fn append_optional(raw: &mut Vec<u8>, value: Option<u8>, present: bool) {
    assert_eq!(value.is_some(), present);
    if let Some(value) = value {
        raw.push(value);
    }
}

fn append_bytes(raw: &mut Vec<u8>, bytes: &[u8]) {
    raw.push(u8::try_from(bytes.len()).unwrap());
    raw.extend_from_slice(bytes);
}

fn append_integers(raw: &mut Vec<u8>, values: &[i32]) {
    raw.extend(values.iter().flat_map(|v| v.to_le_bytes()));
}

fn append_wide(raw: &mut Vec<u8>, values: &[u64]) {
    raw.extend(values.iter().flat_map(|v| v.to_le_bytes()));
}

fn append_additional(raw: &mut Vec<u8>, item: &ExtraFirst, version: i32) {
    append_optional(
        raw,
        item.additional_flag,
        version >= 7 && !item.bytes.is_empty(),
    );
    assert_eq!(
        item.additional.len(),
        if item.additional_flag.is_some_and(|v| v != 0) {
            3 * item.bytes.len()
        } else {
            0
        }
    );
    append_integers(raw, &item.additional);
}

fn append_extra_tail(raw: &mut Vec<u8>, body: &ExtraBody, version: i32) {
    append_optional(raw, body.extra_flag, version >= 3);
    assert_eq!(
        body.extra.is_some(),
        body.extra_flag.is_some_and(|flag| flag != 0)
    );
    if let Some(extra) = &body.extra {
        raw.extend_from_slice(&extra.word.to_le_bytes());
        append_mixed(raw, &extra.mixed, version);
    }
    append_optional(raw, body.last, version >= 4);
}
