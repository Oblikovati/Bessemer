use super::wire::{TransmitCursor, TransmitWriter};
use super::{
    BaseSchema, FieldDefinition, FieldExtent, FieldKind, LayoutDefinition, LayoutEdit, invalid,
};
use crate::Result;

pub(super) fn read(
    cursor: &mut TransmitCursor<'_>,
    code: u16,
    base: &BaseSchema,
) -> Result<LayoutDefinition> {
    let count = cursor.u8()?;
    if count == 255 {
        return Ok(LayoutDefinition::Base);
    }
    if base.node_types().contains_key(&code) {
        return read_edits(cursor, count, code, base);
    }
    read_defined(cursor, count)
}

fn read_defined(cursor: &mut TransmitCursor<'_>, count: u8) -> Result<LayoutDefinition> {
    let name = cursor.string8()?;
    let description = cursor.string8()?;
    let fields = (0..count)
        .map(|_| read_definition(cursor))
        .collect::<Result<Vec<_>>>()?;
    let variable = cursor.u8()?;
    Ok(LayoutDefinition::Defined {
        name,
        description,
        fields,
        variable,
    })
}

fn read_edits(
    cursor: &mut TransmitCursor<'_>,
    count: u8,
    code: u16,
    base: &BaseSchema,
) -> Result<LayoutDefinition> {
    let mut edits = Vec::new();
    loop {
        let operation = cursor.u8()?;
        if operation == b'Z' {
            break;
        }
        edits.push(read_edit(cursor, operation)?);
    }
    let layout = LayoutDefinition::Edited(edits);
    let actual = super::definitions::effective_layout(base, code, &layout)?.len();
    if actual != usize::from(count) {
        return Err(invalid(format!(
            "type {code} edited field count {actual}; expected {count}"
        )));
    }
    Ok(layout)
}

fn read_edit(cursor: &mut TransmitCursor<'_>, operation: u8) -> Result<LayoutEdit> {
    match operation {
        b'C' => Ok(LayoutEdit::Copy),
        b'D' => Ok(LayoutEdit::Drop),
        b'I' => Ok(LayoutEdit::Insert(read_definition(cursor)?)),
        b'A' => Ok(LayoutEdit::Append(read_definition(cursor)?)),
        other => Err(invalid(format!(
            "layout edit {other:#04x}; expected C,D,I,A,Z"
        ))),
    }
}

fn read_definition(cursor: &mut TransmitCursor<'_>) -> Result<FieldDefinition> {
    let name = cursor.string8()?;
    let pointer_class = cursor.u16()?;
    let extent = read_extent(cursor, &name)?;
    let kind = if pointer_class == 0 {
        FieldKind::parse(&cursor.string8()?)?
    } else {
        FieldKind::Pointer
    };
    Ok(FieldDefinition {
        name,
        kind,
        pointer_class,
        extent,
    })
}

fn read_extent(cursor: &mut TransmitCursor<'_>, name: &str) -> Result<FieldExtent> {
    match cursor.u16()? {
        0 => Err(invalid(format!(
            "field {name:?} stored extent 0; expected elements + 1 >= 1"
        ))),
        1 => Ok(FieldExtent::Scalar),
        2 => Ok(FieldExtent::Variable),
        n => Ok(FieldExtent::Fixed(u32::from(n - 1))),
    }
}

pub(super) fn write(
    writer: &mut TransmitWriter,
    layout: &LayoutDefinition,
    effective_count: usize,
) -> Result<()> {
    if matches!(layout, LayoutDefinition::Base) {
        writer.u8(255);
        return Ok(());
    }
    write_count(writer, effective_count)?;
    write_explicit(writer, layout)
}

fn write_count(writer: &mut TransmitWriter, effective_count: usize) -> Result<()> {
    let count = u8::try_from(effective_count)
        .ok()
        .filter(|count| *count < 255)
        .ok_or_else(|| {
            invalid(format!(
                "explicit field count {effective_count}; expected 0..=254"
            ))
        })?;
    writer.u8(count);
    Ok(())
}

fn write_explicit(writer: &mut TransmitWriter, layout: &LayoutDefinition) -> Result<()> {
    match layout {
        LayoutDefinition::Edited(edits) => write_edits(writer, edits),
        LayoutDefinition::Defined {
            name,
            description,
            fields,
            variable,
        } => write_defined(writer, name, description, fields, *variable),
        LayoutDefinition::Base => Err(invalid(
            "base layout in explicit writer; expected edited or defined layout",
        )),
    }
}

fn write_defined(
    writer: &mut TransmitWriter,
    name: &str,
    description: &str,
    fields: &[FieldDefinition],
    variable: u8,
) -> Result<()> {
    writer.string8(name)?;
    writer.string8(description)?;
    for field in fields {
        write_definition(writer, field)?;
    }
    writer.u8(variable);
    Ok(())
}

fn write_edits(writer: &mut TransmitWriter, edits: &[LayoutEdit]) -> Result<()> {
    for edit in edits {
        match edit {
            LayoutEdit::Copy => writer.u8(b'C'),
            LayoutEdit::Drop => writer.u8(b'D'),
            LayoutEdit::Insert(field) => {
                writer.u8(b'I');
                write_definition(writer, field)?;
            }
            LayoutEdit::Append(field) => {
                writer.u8(b'A');
                write_definition(writer, field)?;
            }
        }
    }
    writer.u8(b'Z');
    Ok(())
}

fn write_definition(writer: &mut TransmitWriter, field: &FieldDefinition) -> Result<()> {
    writer.string8(&field.name)?;
    writer.u16(field.pointer_class);
    writer.u16(stored_extent(field)?);
    if field.pointer_class == 0 {
        writer.string8(field.kind.letter())?;
    }
    Ok(())
}

fn stored_extent(field: &FieldDefinition) -> Result<u16> {
    let elements = match field.extent {
        FieldExtent::Scalar => 0,
        FieldExtent::Variable => 1,
        FieldExtent::Fixed(n) => n,
    };
    elements
        .checked_add(1)
        .and_then(|count| u16::try_from(count).ok())
        .ok_or_else(|| {
            invalid(format!(
                "field {:?} extent {elements}; expected 0..=65534",
                field.name
            ))
        })
}
