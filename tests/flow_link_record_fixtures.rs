//! Independent reconstruction of every documented legacy record-402 field.
#[path = "flow_link_records/fixture.rs"]
mod fixture;
use fixture::*;
use std::io::Read;

#[test]
fn every_link_field_reconstructs_exact_wire_and_context_presence() {
    for case in cases() {
        let mut raw = case.version.to_le_bytes().to_vec();
        raw.extend_from_slice(&(case.entries.len() as i32).to_le_bytes());
        for (ordinal, entry) in case.entries.iter().enumerate() {
            assert_eq!(entry.ordinal, ordinal as u32);
            append_entry(&mut raw, entry, &case);
        }
        assert_eq!(
            case.variable_count as usize,
            case.entries.iter().map(|e| e.variable.len()).sum()
        );
        append_tail(&mut raw, &case);
        assert_eq!(raw, case.decoded, "{}", case.name);
        assert_eq!(case.checked, case.pointer_count.is_some());
        assert!(!case.runtime && !case.roles);
        check_transport(&case);
    }
}

fn append_entry(raw: &mut Vec<u8>, entry: &LinkEntry, case: &LinkCase) {
    assert_eq!(raw.len(), entry.start as usize);
    append_ints(raw, &entry.references);
    for bits in entry.wide {
        raw.extend_from_slice(&bits.to_le_bytes());
    }
    raw.push(entry.code);
    raw.push(u8::try_from(entry.variable.len()).unwrap());
    append_ints(raw, &entry.variable);
    check_entry_semantics(entry, case);
    assert_eq!(raw.len(), entry.end as usize);
}

fn check_entry_semantics(entry: &LinkEntry, case: &LinkCase) {
    let converted = if case.version == 0 {
        native_code(entry.code)
    } else {
        entry.code
    };
    assert_eq!(entry.native_code, converted);
    if let Some(count) = case.pointer_count {
        assert!(entry
            .references
            .iter()
            .all(|&v| v == -1 || (v >= 0 && v < count)));
    }
}

fn native_code(code: u8) -> u8 {
    let band = code / 10;
    if (3..=6).contains(&band) && code % 10 < 6 {
        return band * 10 + (code % 10) / 2;
    }
    code
}

fn append_tail(raw: &mut Vec<u8>, case: &LinkCase) {
    let tail = &case.tail;
    assert_eq!(raw.len(), tail.start as usize);
    append_ints(raw, &tail.members);
    append_tail_prefix(raw, tail, case.version);
    append_conditional(raw, tail);
    assert_eq!(tail.numeric.is_some(), case.version >= 6);
    if let Some(numeric) = &tail.numeric {
        append_numeric(raw, numeric, case.version);
    }
    assert_eq!(tail.final_tail.is_some(), case.version >= 7);
    if let Some(final_tail) = &tail.final_tail {
        append_final(raw, final_tail, case.version);
    }
}

fn append_tail_prefix(raw: &mut Vec<u8>, tail: &LinkTail, version: i32) {
    assert_eq!(
        tail.discarded.len(),
        match version {
            2 => 1,
            3 => 3,
            _ => 0,
        }
    );
    append_ints(raw, &tail.discarded);
    assert_eq!(tail.control.is_some(), version >= 5);
    if let Some(control) = tail.control {
        raw.extend_from_slice(&control.to_le_bytes());
    }
}

fn append_conditional(raw: &mut Vec<u8>, tail: &LinkTail) {
    assert_eq!(raw.len(), tail.conditional_start as usize);
    let count = if tail.control.is_some_and(|c| c > 0) {
        tail.members[6].max(0) as usize
    } else {
        0
    };
    assert_eq!(tail.conditional.len(), count);
    append_ints(raw, &tail.conditional);
    assert_eq!(raw.len(), tail.conditional_end as usize);
}

fn append_numeric(raw: &mut Vec<u8>, tail: &NumericTail, version: i32) {
    assert_eq!(raw.len(), tail.start as usize);
    raw.extend_from_slice(&(tail.wide.len() as i32).to_le_bytes());
    for bits in &tail.wide {
        raw.extend_from_slice(&bits.to_le_bytes());
    }
    assert_eq!(tail.indices.is_some(), version >= 8);
    assert_eq!(tail.runtime_default, (version < 8).then_some(-1));
    if let Some(indices) = &tail.indices {
        assert_eq!(indices.len(), tail.wide.len());
        append_ints(raw, indices);
    }
    assert_eq!(raw.len(), tail.end as usize);
}

fn append_final(raw: &mut Vec<u8>, tail: &FinalTail, version: i32) {
    assert_eq!(raw.len(), tail.start as usize);
    assert_eq!(tail.width, if version == 9 { 8 } else { 4 });
    raw.extend_from_slice(&(tail.values.len() as i32).to_le_bytes());
    for value in &tail.values {
        raw.extend_from_slice(&value.to_le_bytes()[..tail.width as usize]);
    }
    assert_eq!(raw.len(), tail.end as usize);
}

fn append_ints(raw: &mut Vec<u8>, values: &[i32]) {
    raw.extend(values.iter().flat_map(|v| v.to_le_bytes()));
}

fn check_transport(case: &LinkCase) {
    if !case.compressed {
        assert_eq!(case.stored, case.decoded);
        return;
    }
    let mut decoded = Vec::new();
    flate2::read::ZlibDecoder::new(case.stored.as_slice())
        .read_to_end(&mut decoded)
        .unwrap();
    assert_eq!(decoded, case.decoded);
}

#[test]
fn native_totals_and_all_byte_normalizations_have_owned_expectations() {
    let cases = cases();
    assert_eq!(cases.len(), 954);
    for case in &cases[..2] {
        assert_eq!(
            (case.version, case.entries.len(), case.decoded.len()),
            (6, 5532, 588028)
        );
        assert_eq!((case.variable_count, case.tail.start), (47602, 555528));
        assert_eq!(case.tail.numeric.as_ref().unwrap().wide.len(), 4056);
    }
    for code in 0..=255_u8 {
        assert!(cases.iter().any(|c| c.name == format!("v0-code-{code}")
            && c.entries[0].native_code == native_code(code)));
    }
}
