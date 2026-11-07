use super::FlowMediumKindTwoWideField as WideField;
use super::*;
pub(super) type Wide = (FlowMediumKindTwoWideField, usize, Vec<u64>);
pub(crate) fn fields(case: &KindTwoCase) -> Vec<Wide> {
    let mut result = Vec::new();
    let mut shared = Vec::new();
    for (ordinal, curve) in case.prefix.curves.iter().enumerate() {
        crate::api::curve_fields(&mut shared, ordinal, curve);
    }
    let end = crate::api::shared_fields(&mut shared, &case.prefix);
    remap(&mut result, shared, FlowMediumCurveScope::Body);
    assert_eq!(end + 12, case.model.start as usize);
    result.push((WideField::BodyScalar, end, vec![case.scalar]));
    model_fields(&mut result, &case.model);
    result
}
fn remap(
    result: &mut Vec<Wide>,
    fields: Vec<(FlowMediumWideField, usize, Vec<u64>)>,
    scope: FlowMediumCurveScope,
) {
    for (field, offset, words) in fields {
        let field = match field {
            FlowMediumWideField::CurveScalar { curve } => WideField::CurveScalar { scope, curve },
            FlowMediumWideField::CurveArray { curve, array } => WideField::CurveArray {
                scope,
                curve,
                array,
            },
            FlowMediumWideField::SharedArray { array } => WideField::SharedArray { array },
            _ => panic!("unexpected common fixture role"),
        };
        result.push((field, offset, words));
    }
}
fn model_fields(result: &mut Vec<Wide>, model: &ModelCase) {
    let start = model.start as usize + 1;
    match &model.payload {
        ModelPayload::Curves(value) => model_curves(result, value),
        ModelPayload::Scalars(value) => push(result, WideField::ModelScalars, start, value),
        ModelPayload::Switched(value) => model_switched(result, start, value),
        ModelPayload::Table(value) => table_fields(result, start, value),
    }
}
fn push(result: &mut Vec<Wide>, field: FlowMediumKindTwoWideField, offset: usize, words: &[u64]) {
    result.push((field, offset, words.to_vec()));
}
fn table_fields(result: &mut Vec<Wide>, start: usize, table: &TableModel) {
    let mut offset = first_table_fields(result, start, table);
    append_next(
        result,
        WideField::ModelSecondArray,
        &mut offset,
        &table.second,
    );
    offset += table.word_patterns.len() * 4 + 4;
    append_next(
        result,
        WideField::ModelThirdArray,
        &mut offset,
        &table.third,
    );
    offset += 4;
    append_next(result, WideField::ModelScalars, &mut offset, &table.scalars);
}
fn append_next(
    result: &mut Vec<Wide>,
    field: FlowMediumKindTwoWideField,
    offset: &mut usize,
    words: &[u64],
) {
    push(result, field, *offset, words);
    *offset += words.len() * 8;
}
pub(super) fn states(model: &ModelCase) -> (usize, Vec<u8>) {
    match &model.payload {
        ModelPayload::Curves(value) => (
            model.end as usize - value.states.len(),
            value.states.clone(),
        ),
        ModelPayload::Scalars(_) => (model.end as usize, Vec::new()),
        ModelPayload::Switched(value) => (model.start as usize + 49, vec![value.state]),
        ModelPayload::Table(value) => (
            model.end as usize - value.states.len(),
            value.states.clone(),
        ),
    }
}
pub(super) fn word_span(model: &ModelCase, table: &TableModel) -> usize {
    model.start as usize + 9 + table.first_count as usize * 16 + table.second_count as usize * 8
}

fn model_curves(result: &mut Vec<Wide>, value: &CurveModel) {
    let mut curves = Vec::new();
    for (ordinal, curve) in value.curves.iter().enumerate() {
        crate::api::curve_fields(&mut curves, ordinal, curve);
    }
    remap(result, curves, FlowMediumCurveScope::Model);
    push(
        result,
        WideField::ModelScalars,
        value.curves.last().unwrap().end as usize,
        &value.scalars,
    );
}

fn model_switched(result: &mut Vec<Wide>, start: usize, value: &SwitchedModel) {
    push(result, WideField::ModelFirstScalars, start, &value.first);
    push(
        result,
        WideField::ModelLastScalar,
        start + 49,
        &[value.last],
    );
}

fn first_table_fields(result: &mut Vec<Wide>, start: usize, table: &TableModel) -> usize {
    let mut offset = start + 8;
    for (array, values) in table.first.iter().enumerate() {
        append_next(
            result,
            WideField::ModelFirstArray { array },
            &mut offset,
            values,
        );
    }
    offset
}
