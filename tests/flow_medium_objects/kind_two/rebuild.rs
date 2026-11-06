use super::*;
pub fn body(case: &KindTwoCase) -> Vec<u8> {
    let mut raw = rebuild::prefix(&case.prefix);
    raw.extend_from_slice(&case.scalar.to_le_bytes());
    raw.extend_from_slice(&case.model.kind.to_le_bytes());
    append_model(&mut raw, &case.model);
    raw.push(case.footer.state);
    assert_eq!(raw.len(), case.footer.end as usize);
    raw.extend_from_slice(&case.footer.tail);
    assert_eq!(case.footer.loaded, 1);
    assert!(case.footer.complete);
    assert!(!case.footer.installed);
    assert!(!case.footer.runtime);
    raw
}
fn append_model(raw: &mut Vec<u8>, model: &ModelCase) {
    assert_eq!(raw.len(), model.start as usize);
    assert_eq!(model.version, 0);
    raw.push(model.version);
    match &model.payload {
        ModelPayload::Curves(value) => append_curves(raw, model.kind, value),
        ModelPayload::Scalars(value) => {
            assert_eq!(model.kind, 3);
            assert_eq!(value.len(), 7);
            rebuild::append_words(raw, value);
        }
        ModelPayload::Switched(value) => append_switched(raw, model.kind, value),
        ModelPayload::Table(value) => {
            assert_eq!(model.kind, 5);
            append_table(raw, value);
        }
    }
    assert_eq!(raw.len(), model.end as usize);
}
fn append_curves(raw: &mut Vec<u8>, kind: i32, model: &CurveModel) {
    assert!((0..=2).contains(&kind));
    assert_eq!(model.curves.len(), if kind == 0 { 2 } else { 3 });
    for curve in &model.curves {
        rebuild::append_curve(raw, curve);
    }
    assert_eq!(model.scalars.len(), 1);
    rebuild::append_words(raw, &model.scalars);
    assert_eq!(model.states.len(), if kind == 1 { 2 } else { 0 });
    raw.extend_from_slice(&model.states);
}
fn append_switched(raw: &mut Vec<u8>, kind: i32, model: &SwitchedModel) {
    assert_eq!(kind, 4);
    assert_eq!(model.first.len(), 6);
    rebuild::append_words(raw, &model.first);
    raw.push(model.state);
    raw.extend_from_slice(&model.last.to_le_bytes());
}
fn append_table(raw: &mut Vec<u8>, model: &TableModel) {
    raw.extend_from_slice(&model.first_count.to_le_bytes());
    raw.extend_from_slice(&model.second_count.to_le_bytes());
    assert_eq!(model.first.len(), 2);
    for values in &model.first {
        assert_eq!(values.len(), model.first_count as usize);
        rebuild::append_words(raw, values);
    }
    assert_eq!(model.second.len(), model.second_count as usize);
    rebuild::append_words(raw, &model.second);
    assert_eq!(model.word_patterns.len(), model.second_count as usize);
    for value in &model.word_patterns {
        raw.extend_from_slice(&value.to_le_bytes());
    }
    append_table_tail(raw, model);
}
fn append_table_tail(raw: &mut Vec<u8>, model: &TableModel) {
    raw.extend_from_slice(&model.third_count.to_le_bytes());
    assert_eq!(model.third.len(), model.third_count as usize);
    rebuild::append_words(raw, &model.third);
    raw.extend_from_slice(&model.member.to_le_bytes());
    assert_eq!(model.scalars.len(), 2);
    rebuild::append_words(raw, &model.scalars);
    assert_eq!(model.states.len(), 2);
    raw.extend_from_slice(&model.states);
}
