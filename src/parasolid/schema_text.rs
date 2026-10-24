use super::{BaseNodeType, FieldDefinition, FieldExtent, FieldKind, SchemaNumber, invalid};
use crate::Result;
use std::collections::BTreeMap;

struct PendingType {
    code: u16,
    node: BaseNodeType,
    declared: usize,
    consumed: usize,
}

struct SchemaRow<'a> {
    label: &'a str,
    middle: &'a str,
    flags: [u32; 3],
}

pub(super) fn parse(text: &str) -> Result<(SchemaNumber, BTreeMap<u16, BaseNodeType>)> {
    let (header, body) = text
        .split_once("**END_OF_HEADER")
        .ok_or_else(|| invalid("schema header missing; expected **END_OF_HEADER"))?;
    let number = schema_identity(header)?;
    let mut lines = schema_lines(body)?;
    let count_line = lines
        .next()
        .ok_or_else(|| invalid("schema count line absent; expected four unsigned counts"))?;
    let (expected_types, expected_fields) = parse_count_line(count_line)?;
    let (types, fields) = collect_types(lines)?;
    if types.len() != expected_types || fields != expected_fields {
        return Err(invalid(format!(
            "schema counts {} types / {fields} fields; expected {expected_types} / {expected_fields}",
            types.len()
        )));
    }
    Ok((number, types))
}

fn schema_lines(body: &str) -> Result<std::str::Lines<'_>> {
    let mut lines = body.lines();
    lines.next();
    for expected in ["T", "1"] {
        let actual = lines.next().map(str::trim);
        if actual != Some(expected) {
            return Err(invalid(format!(
                "schema header line {actual:?}; expected {expected:?}"
            )));
        }
    }
    let title = lines.next();
    if !title.is_some_and(|line| line.starts_with(": SCHEMA FILE")) {
        return Err(invalid(format!(
            "schema title {title:?}; expected : SCHEMA FILE..."
        )));
    }
    Ok(lines)
}

fn collect_types(lines: std::str::Lines<'_>) -> Result<(BTreeMap<u16, BaseNodeType>, usize)> {
    let mut types = BTreeMap::new();
    let mut pending = None;
    let mut fields = 0;
    for line in lines
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('*'))
    {
        fields += parse_line(line, &mut pending, &mut types)?;
    }
    fields += finish_type(pending, &mut types)?;
    Ok((types, fields))
}

fn schema_identity(header: &str) -> Result<SchemaNumber> {
    let tail = header
        .split_once("KEY=SCH_")
        .map(|(_, tail)| tail)
        .ok_or_else(|| invalid("schema KEY absent; expected KEY=SCH_<number>;"))?;
    let (token, _) = tail.split_once(';').ok_or_else(|| {
        invalid(format!(
            "schema KEY {tail:?}; expected a semicolon after the number"
        ))
    })?;
    SchemaNumber::parse(token)
}

fn parse_count_line(line: &str) -> Result<(usize, usize)> {
    let counts = unsigned_words(line)?;
    let [_, types, fields, _] = counts.as_slice() else {
        return Err(invalid(format!(
            "schema counts {line:?}; expected four unsigned integers"
        )));
    };
    Ok((*types as usize, *fields as usize))
}

fn unsigned_words(text: &str) -> Result<Vec<u32>> {
    text.split_whitespace()
        .map(|word| SchemaNumber::parse(word).map(SchemaNumber::value))
        .collect()
}

fn parse_line(
    line: &str,
    pending: &mut Option<PendingType>,
    types: &mut BTreeMap<u16, BaseNodeType>,
) -> Result<usize> {
    let row = parse_columns(line)?;
    if let Some((code, name)) = row.label.split_once(' ') {
        let fields = finish_type(pending.take(), types)?;
        *pending = Some(start_type(code, name, &row)?);
        return Ok(fields);
    }
    add_field(pending, &row)?;
    Ok(0)
}

fn parse_columns(line: &str) -> Result<SchemaRow<'_>> {
    let (label, rest) = line.split_once(';').ok_or_else(|| {
        invalid(format!(
            "schema line {line:?}; expected label; description/type; flags"
        ))
    })?;
    let (middle, flags) = rest
        .rsplit_once(';')
        .ok_or_else(|| invalid(format!("schema line {line:?}; expected two semicolons")))?;
    let words: [u32; 3] = unsigned_words(flags)?.try_into().map_err(|_| {
        invalid(format!(
            "schema flags {flags:?}; expected three unsigned integers"
        ))
    })?;
    Ok(SchemaRow {
        label,
        middle: middle.trim(),
        flags: words,
    })
}

fn start_type(code: &str, name: &str, row: &SchemaRow<'_>) -> Result<PendingType> {
    let [_, declared, variable] = row.flags;
    let numeric = SchemaNumber::parse(code)?.value();
    let code = u16::try_from(numeric)
        .map_err(|_| invalid(format!("node type {numeric}; expected u16")))?;
    let node = BaseNodeType {
        name: name.trim().to_owned(),
        description: row.middle.to_owned(),
        variable,
        fields: Vec::new(),
    };
    Ok(PendingType {
        code,
        node,
        declared: declared as usize,
        consumed: 0,
    })
}

fn add_field(pending: &mut Option<PendingType>, row: &SchemaRow<'_>) -> Result<()> {
    let pending = pending.as_mut().ok_or_else(|| {
        invalid(format!(
            "field {:?}; expected a preceding node type",
            row.label
        ))
    })?;
    pending.consumed += 1;
    if row.flags[0] == 0 {
        return Ok(());
    }
    pending.node.fields.push(schema_field(row)?);
    Ok(())
}

fn schema_field(row: &SchemaRow<'_>) -> Result<FieldDefinition> {
    let [_, pointer_class, elements] = row.flags;
    let pointer_class = u16::try_from(pointer_class).map_err(|_| {
        invalid(format!(
            "field {:?} pointer class {pointer_class}; expected u16",
            row.label
        ))
    })?;
    let extent = match elements {
        0 => FieldExtent::Scalar,
        1 => FieldExtent::Variable,
        n => FieldExtent::Fixed(n),
    };
    Ok(FieldDefinition {
        name: row.label.to_owned(),
        kind: FieldKind::parse(row.middle)?,
        pointer_class,
        extent,
    })
}

fn finish_type(
    pending: Option<PendingType>,
    types: &mut BTreeMap<u16, BaseNodeType>,
) -> Result<usize> {
    let Some(pending) = pending else {
        return Ok(0);
    };
    if pending.declared != pending.consumed {
        return Err(invalid(format!(
            "node type {} fields {}; expected {} declared fields",
            pending.code, pending.consumed, pending.declared
        )));
    }
    if types.insert(pending.code, pending.node).is_some() {
        return Err(invalid(format!(
            "duplicate node type {}; expected unique codes",
            pending.code
        )));
    }
    Ok(pending.consumed)
}
