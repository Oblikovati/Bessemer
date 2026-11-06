use super::types::*;
pub fn body(case: &BodyCase) -> Vec<u8> {
    let mut raw = prefix(&case.prefix);
    assert_eq!(case.footer.scalars.len(), 4);
    append_words(&mut raw, &case.footer.scalars);
    raw.push(case.footer.state);
    assert_eq!(raw.len(), case.footer.end as usize);
    raw.extend_from_slice(&case.footer.tail);
    check_footer(&case.footer);
    raw
}
fn append_text(raw: &mut Vec<u8>, text: &BodyText) {
    assert_eq!(raw.len(), text.start as usize);
    raw.extend_from_slice(&text.units.to_le_bytes());
    assert_eq!(raw.len(), text.text_start as usize);
    assert_eq!(text.raw.len(), text.units as usize * 2);
    if text.units > 0 {
        assert_eq!(&text.raw[text.raw.len() - 2..], &[0, 0]);
    }
    let units = text.raw.as_chunks::<2>().0;
    let first_nul = units
        .iter()
        .position(|unit| *unit == [0, 0])
        .unwrap_or(units.len());
    assert_eq!(text.native, &text.raw[..first_nul * 2]);
    raw.extend_from_slice(&text.raw);
    assert_eq!(raw.len(), text.end as usize);
}
fn append_curve(raw: &mut Vec<u8>, curve: &BodyCurve) {
    assert_eq!(raw.len(), curve.start as usize);
    assert_eq!(curve.version, 0);
    raw.push(curve.version);
    raw.extend_from_slice(&curve.scalar.to_le_bytes());
    raw.extend_from_slice(&curve.count.to_le_bytes());
    assert_eq!(curve.first.len(), curve.count as usize);
    assert_eq!(curve.second.len(), curve.count as usize);
    append_words(raw, &curve.first);
    append_words(raw, &curve.second);
    raw.push(curve.flag);
    check_extra_lengths(curve);
    append_words(raw, &curve.extra_first);
    append_words(raw, &curve.extra_second);
    assert_eq!(raw.len(), curve.end as usize);
}
fn append_arrays(raw: &mut Vec<u8>, count: u32, arrays: &[Vec<u64>], arity: usize) {
    raw.extend_from_slice(&count.to_le_bytes());
    assert_eq!(arrays.len(), arity);
    for values in arrays {
        assert_eq!(values.len(), count as usize);
        append_words(raw, values);
    }
}
fn append_words(raw: &mut Vec<u8>, values: &[u64]) {
    for value in values {
        raw.extend_from_slice(&value.to_le_bytes());
    }
}

fn check_footer(footer: &BodyFooter) {
    assert_eq!(footer.loaded_state, 1);
    assert!(footer.complete);
    assert!(!footer.runtime);
    assert!(!footer.engineering);
}

fn check_extra_lengths(curve: &BodyCurve) {
    assert_eq!(curve.extra_count, (curve.flag != 0).then_some(curve.count));
    let extra = if curve.flag == 0 {
        0
    } else {
        curve.count as usize
    };
    assert_eq!(curve.extra_first.len(), extra);
    assert_eq!(curve.extra_second.len(), extra);
}

pub(super) fn prefix(prefix: &BodyPrefix) -> Vec<u8> {
    assert_eq!(prefix.version, 0);
    assert!(!prefix.base_dispatch);
    assert_eq!(prefix.key.len(), 16);
    let mut raw = vec![prefix.version, prefix.base];
    raw.extend_from_slice(&prefix.key);
    append_text(&mut raw, &prefix.text);
    assert_eq!(prefix.curves.len(), 7);
    for curve in &prefix.curves {
        append_curve(&mut raw, curve);
    }
    append_arrays(&mut raw, prefix.first_count, &prefix.first, 2);
    append_arrays(&mut raw, prefix.second_count, &prefix.second, 4);
    raw
}
