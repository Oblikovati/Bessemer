use super::{
    BaseSchema, DeltaSection, FieldDefinition, FieldExtent, LayoutDefinition, NodeRecord,
    RecordList, TransmitBody, TransmitContents, invalid,
};
use crate::Result;
use std::collections::{BTreeMap, BTreeSet};

pub(super) type EffectiveLayouts = BTreeMap<u16, Vec<FieldDefinition>>;

pub(super) fn contents(
    base: &BaseSchema,
    contents: &TransmitContents,
    context: super::title_context::TitleContext,
) -> Result<EffectiveLayouts> {
    require_header(base, contents, context)?;
    let layouts = resolve_layouts(base, &contents.layouts)?;
    let mut used = BTreeSet::new();
    validate_body(&contents.body, &layouts, &mut used)?;
    if used.len() != layouts.len() {
        return Err(invalid(format!(
            "{} used of {} layout definitions; expected no orphan layouts",
            used.len(),
            layouts.len()
        )));
    }
    Ok(layouts)
}

fn require_header(
    base: &BaseSchema,
    contents: &TransmitContents,
    context: super::title_context::TitleContext,
) -> Result<()> {
    super::decode::require_schema(&contents.schema_key, base)?;
    super::definitions::latin1_length(&contents.title, u32::MAX as usize)?;
    let delta = context.is_delta(&contents.title)?;
    if delta != matches!(contents.body, TransmitBody::Deltas(_)) {
        return Err(invalid(format!(
            "title {:?} and delta-body presence {}; expected matching file kind",
            contents.title,
            matches!(contents.body, TransmitBody::Deltas(_))
        )));
    }
    Ok(())
}

fn resolve_layouts(
    base: &BaseSchema,
    layouts: &BTreeMap<u16, LayoutDefinition>,
) -> Result<EffectiveLayouts> {
    layouts
        .iter()
        .map(|(code, layout)| {
            super::definitions::effective_layout(base, *code, layout).map(|fields| (*code, fields))
        })
        .collect()
}

pub(super) fn is_delta_title(title: &str) -> Result<bool> {
    match (title.contains("(partition)"), title.contains("(deltas)")) {
        (true, false) => Ok(false),
        (false, true) => Ok(true),
        _ => Err(invalid(format!(
            "transmit title {title:?}; expected exactly one (partition) or (deltas) marker"
        ))),
    }
}

fn validate_body(
    body: &TransmitBody,
    layouts: &EffectiveLayouts,
    used: &mut BTreeSet<u16>,
) -> Result<()> {
    match body {
        TransmitBody::Partition(records) => validate_list(records, layouts, used, false, false),
        TransmitBody::Deltas(sections) => validate_sections(sections, layouts, used),
    }
}

fn validate_sections(
    sections: &[DeltaSection],
    layouts: &EffectiveLayouts,
    used: &mut BTreeSet<u16>,
) -> Result<()> {
    if sections.is_empty() {
        return Err(invalid(
            "delta section count 0; expected at least one final section",
        ));
    }
    for (index, section) in sections.iter().enumerate() {
        require_section_shape(section, index, sections.len())?;
        validate_list(&section.records, layouts, used, true, false)?;
        if let Some(changes) = &section.changes {
            validate_list(&changes.modified, layouts, used, true, true)?;
        }
    }
    Ok(())
}

fn require_section_shape(section: &DeltaSection, index: usize, count: usize) -> Result<()> {
    if section.changes.is_none() == (index + 1 == count) {
        return Ok(());
    }
    Err(invalid(format!(
        "delta section {index} changes presence {}; expected absent only in final section",
        section.changes.is_some()
    )))
}

fn validate_list(
    list: &RecordList,
    layouts: &EffectiveLayouts,
    used: &mut BTreeSet<u16>,
    deltas: bool,
    modified: bool,
) -> Result<()> {
    for record in &list.records {
        let fields = record_fields(record.node_type, layouts)?;
        validate_record(
            record,
            fields,
            modified || (deltas && !matches!(record.node_type, 3 | 4)),
        )?;
        used.insert(record.node_type);
    }
    Ok(())
}

fn record_fields(code: u16, layouts: &EffectiveLayouts) -> Result<&[FieldDefinition]> {
    if code == 1 {
        return Err(invalid(
            "record type 1; expected terminator outside record collection",
        ));
    }
    layouts
        .get(&code)
        .map(Vec::as_slice)
        .ok_or_else(|| invalid(format!("record type {code}; expected a declared layout")))
}

fn validate_record(record: &NodeRecord, fields: &[FieldDefinition], delta: bool) -> Result<()> {
    require_record_shape(record, fields)?;
    for (field, values) in fields.iter().zip(&record.fields) {
        super::field_validate::validate(field, values, record.length, delta)?;
    }
    Ok(())
}

fn require_record_shape(record: &NodeRecord, fields: &[FieldDefinition]) -> Result<()> {
    if record.fields.len() != fields.len() {
        return Err(invalid(format!(
            "type {} field count {}; expected {}",
            record.node_type,
            record.fields.len(),
            fields.len()
        )));
    }
    let variable = fields
        .iter()
        .any(|field| field.extent == FieldExtent::Variable);
    if record.length.is_some() != variable {
        return Err(invalid(format!(
            "type {} length {:?}; expected length presence {variable}",
            record.node_type, record.length
        )));
    }
    Ok(())
}
