use super::{
    BaseSchema, FieldDefinition, FieldExtent, FieldKind, LayoutDefinition, LayoutEdit, invalid,
    unsupported,
};
use crate::Result;

impl FieldKind {
    pub(super) fn parse(text: &str) -> Result<Self> {
        match text {
            "d" => Ok(Self::Integer),
            "p" => Ok(Self::Pointer),
            "u" => Ok(Self::UnsignedByte),
            "c" => Ok(Self::Character),
            "l" => Ok(Self::Logical),
            "n" => Ok(Self::Short),
            "w" => Ok(Self::WideUnit),
            "f" => Ok(Self::Real),
            "i" => Ok(Self::Interval),
            "v" => Ok(Self::Vector),
            "h" => Ok(Self::HVector),
            "b" => Ok(Self::Box),
            "q" => Ok(Self::SchemaOnlyQ),
            "t" => Ok(Self::SchemaOnlyT),
            other => Err(unsupported(format!(
                "transmitted field kind {other:?}; expected d,p,u,c,l,n,w,f,i,v,h,b"
            ))),
        }
    }

    pub(super) fn letter(self) -> &'static str {
        match self {
            Self::Integer => "d",
            Self::Pointer => "p",
            Self::UnsignedByte => "u",
            Self::Character => "c",
            Self::Logical => "l",
            Self::Short => "n",
            Self::WideUnit => "w",
            Self::Real => "f",
            Self::Interval => "i",
            Self::Vector => "v",
            Self::HVector => "h",
            Self::Box => "b",
            Self::SchemaOnlyQ => "q",
            Self::SchemaOnlyT => "t",
        }
    }

    pub(super) fn minimum_bytes(self, delta: bool) -> Result<usize> {
        let size = match self {
            Self::Integer => 4,
            Self::Pointer => 2 + usize::from(delta),
            Self::UnsignedByte | Self::Character | Self::Logical => 1,
            Self::Short | Self::WideUnit => 2,
            Self::Real => 8,
            Self::Interval => 16,
            Self::Vector => 24,
            Self::HVector => 24 + 64 * usize::from(delta),
            Self::Box => 48,
            Self::SchemaOnlyQ | Self::SchemaOnlyT => {
                return Err(unsupported(format!(
                    "transmitted field kind {:?}; expected a documented wire kind instead of schema-only q/t",
                    self.letter()
                )));
            }
        };
        Ok(size)
    }
}

pub(super) fn latin1_length(text: &str, maximum: usize) -> Result<usize> {
    if let Some(character) = text.chars().find(|character| u32::from(*character) > 255) {
        return Err(invalid(format!(
            "text character {character:?}; expected Latin-1 code point <= 255"
        )));
    }
    let count = text.chars().count();
    if count > maximum {
        return Err(invalid(format!(
            "text length {count}; expected at most {maximum} Latin-1 bytes"
        )));
    }
    Ok(count)
}

pub(super) fn validate_fields(fields: &[FieldDefinition], descriptor: bool) -> Result<()> {
    if descriptor && fields.len() > 254 {
        return Err(invalid(format!(
            "layout field count {}; expected 0..=254 for explicit descriptor",
            fields.len()
        )));
    }
    for (index, field) in fields.iter().enumerate() {
        validate_definition(field, descriptor)?;
        if field.extent == FieldExtent::Variable && index + 1 != fields.len() {
            return Err(invalid(format!(
                "variable field {:?} at {index}; expected final field of {}",
                field.name,
                fields.len()
            )));
        }
    }
    Ok(())
}

fn validate_definition(field: &FieldDefinition, descriptor: bool) -> Result<()> {
    latin1_length(
        &field.name,
        if descriptor { 255 } else { u32::MAX as usize },
    )?;
    if field.kind != FieldKind::Pointer && field.pointer_class != 0 {
        return Err(invalid(format!(
            "field {:?} class {}; expected zero for {:?}",
            field.name, field.pointer_class, field.kind
        )));
    }
    validate_extent(field, descriptor)
}

fn validate_extent(field: &FieldDefinition, descriptor: bool) -> Result<()> {
    let FieldExtent::Fixed(count) = field.extent else {
        return Ok(());
    };
    let maximum = if descriptor { 65_534 } else { u32::MAX };
    if !(2..=maximum).contains(&count) {
        return Err(invalid(format!(
            "field {:?} fixed extent {count}; expected 2..={maximum}",
            field.name
        )));
    }
    Ok(())
}

pub(super) fn effective_layout(
    base: &BaseSchema,
    code: u16,
    layout: &LayoutDefinition,
) -> Result<Vec<FieldDefinition>> {
    let fields = resolve_fields(base, code, layout)?;
    validate_fields(&fields, !matches!(layout, LayoutDefinition::Base))?;
    for field in &fields {
        field.kind.minimum_bytes(false)?;
    }
    Ok(fields)
}

fn resolve_fields(
    base: &BaseSchema,
    code: u16,
    layout: &LayoutDefinition,
) -> Result<Vec<FieldDefinition>> {
    let source = base.node_types().get(&code);
    match (source, layout) {
        (Some(node), LayoutDefinition::Base) => Ok(node.fields.clone()),
        (Some(node), LayoutDefinition::Edited(edits)) => edit_layout(&node.fields, edits),
        (
            None,
            LayoutDefinition::Defined {
                name,
                description,
                fields,
                ..
            },
        ) => defined_fields(name, description, fields),
        _ => Err(invalid(format!(
            "node type {code} descriptor {}; expected base/edited for known type or full definition for new type",
            descriptor_label(layout)
        ))),
    }
}

fn defined_fields(
    name: &str,
    description: &str,
    fields: &[FieldDefinition],
) -> Result<Vec<FieldDefinition>> {
    latin1_length(name, 255)?;
    latin1_length(description, 255)?;
    Ok(fields.to_vec())
}

fn descriptor_label(layout: &LayoutDefinition) -> &'static str {
    match layout {
        LayoutDefinition::Base => "Base",
        LayoutDefinition::Edited(_) => "Edited",
        LayoutDefinition::Defined { .. } => "Defined",
    }
}

fn edit_layout(source: &[FieldDefinition], edits: &[LayoutEdit]) -> Result<Vec<FieldDefinition>> {
    let mut remaining = source.iter();
    let mut fields = Vec::new();
    for edit in edits {
        match edit {
            LayoutEdit::Copy => fields.push(next_base_field(&mut remaining)?.clone()),
            LayoutEdit::Drop => {
                next_base_field(&mut remaining)?;
            }
            LayoutEdit::Insert(field) | LayoutEdit::Append(field) => {
                validate_definition(field, true)?;
                fields.push(field.clone());
            }
        }
    }
    Ok(fields)
}

fn next_base_field<'a>(
    remaining: &mut impl Iterator<Item = &'a FieldDefinition>,
) -> Result<&'a FieldDefinition> {
    remaining
        .next()
        .ok_or_else(|| invalid("base layout exhausted; expected a remaining field for C/D edit"))
}
