use crate::Result;
use crate::archive_codec::ArchiveString;

use super::{HeaderBounds, HeaderReference, SharedFileName, invalid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum WireKind {
    U8,
    U16,
    U32,
    I32,
    Text,
    Reference,
    References,
    Texts,
    I32x3,
    I32x6,
    Bounds,
    SharedName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FieldRole {
    Any,
    Strings,
    ObjectList,
    RevisionLogs,
    Log,
    Stamp,
    ExternalList,
    External,
    Handle,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Phase {
    Outer,
    Body,
}

pub(super) struct FieldRule {
    pub name: &'static str,
    pub kind: WireKind,
    pub role: FieldRole,
    pub phase: Phase,
    pub optional: bool,
    pub enabled: fn(u32) -> bool,
}

#[derive(Debug)]
pub(super) enum FieldValue {
    U8(u8),
    U16(u16),
    U32(u32),
    I32(i32),
    Text(ArchiveString),
    Reference(HeaderReference),
    References(Vec<HeaderReference>),
    Texts(Vec<ArchiveString>),
    I32x3([i32; 3]),
    I32x6([i32; 6]),
    Bounds(HeaderBounds),
    SharedName(SharedFileName),
}

#[derive(Clone, Copy)]
pub(super) enum FieldView<'a> {
    U8(u8),
    U16(u16),
    U32(u32),
    I32(i32),
    Text(&'a ArchiveString),
    Reference(HeaderReference),
    References(&'a [HeaderReference]),
    Texts(&'a [ArchiveString]),
    I32x3(&'a [i32; 3]),
    I32x6(&'a [i32; 6]),
    Bounds(&'a HeaderBounds),
    SharedName(&'a SharedFileName),
}

pub(super) trait FieldType: Sized {
    const KIND: WireKind;
    fn from_value(value: FieldValue) -> Result<Self>;
    fn view(&self) -> FieldView<'_>;
}

macro_rules! scalar_field {
    ($type:ty, $variant:ident) => {
        impl FieldType for $type {
            const KIND: WireKind = WireKind::$variant;
            fn from_value(value: FieldValue) -> Result<Self> {
                match value {
                    FieldValue::$variant(value) => Ok(value),
                    other => Err(mismatched(&other, Self::KIND)),
                }
            }
            fn view(&self) -> FieldView<'_> {
                FieldView::$variant(*self)
            }
        }
    };
}

macro_rules! borrowed_field {
    ($type:ty, $variant:ident) => {
        impl FieldType for $type {
            const KIND: WireKind = WireKind::$variant;
            fn from_value(value: FieldValue) -> Result<Self> {
                match value {
                    FieldValue::$variant(value) => Ok(value),
                    other => Err(mismatched(&other, Self::KIND)),
                }
            }
            fn view(&self) -> FieldView<'_> {
                FieldView::$variant(self)
            }
        }
    };
}

scalar_field!(u8, U8);
scalar_field!(u16, U16);
scalar_field!(u32, U32);
scalar_field!(i32, I32);
scalar_field!(HeaderReference, Reference);
borrowed_field!(ArchiveString, Text);
borrowed_field!(Vec<HeaderReference>, References);
borrowed_field!(Vec<ArchiveString>, Texts);
borrowed_field!([i32; 3], I32x3);
borrowed_field!([i32; 6], I32x6);
borrowed_field!(HeaderBounds, Bounds);
borrowed_field!(SharedFileName, SharedName);

impl FieldValue {
    fn kind(&self) -> WireKind {
        match self {
            Self::U8(_) => WireKind::U8,
            Self::U16(_) => WireKind::U16,
            Self::U32(_) => WireKind::U32,
            Self::I32(_) => WireKind::I32,
            Self::Text(_) => WireKind::Text,
            Self::Reference(_) => WireKind::Reference,
            Self::References(_) => WireKind::References,
            Self::Texts(_) => WireKind::Texts,
            Self::I32x3(_) => WireKind::I32x3,
            Self::I32x6(_) => WireKind::I32x6,
            Self::Bounds(_) => WireKind::Bounds,
            Self::SharedName(_) => WireKind::SharedName,
        }
    }
}

fn mismatched(actual: &FieldValue, expected: WireKind) -> crate::BessemerError {
    invalid(format!(
        "field kind {:?}; expected {expected:?}",
        actual.kind()
    ))
}

pub(super) fn required<T: FieldType>(
    fields: &mut impl Iterator<Item = Option<FieldValue>>,
    name: &str,
) -> Result<T> {
    let value = fields
        .next()
        .flatten()
        .ok_or_else(|| invalid(format!("field {name} absent; expected {:?}", T::KIND)))?;
    T::from_value(value)
}

pub(super) fn optional<T: FieldType>(
    fields: &mut impl Iterator<Item = Option<FieldValue>>,
    name: &str,
) -> Result<Option<T>> {
    fields
        .next()
        .ok_or_else(|| invalid(format!("field {name} missing; expected an optional slot")))?
        .map(T::from_value)
        .transpose()
}

macro_rules! field_type {
    (required, $type:ty) => { $type };
    (optional, $type:ty) => { Option<$type> };
}
macro_rules! field_optional {
    (required) => {
        false
    };
    (optional) => {
        true
    };
}
macro_rules! field_view {
    (required, $type:ty, $value:expr) => {
        Some(<$type as FieldType>::view(&$value))
    };
    (optional, $type:ty, $value:expr) => {
        $value.as_ref().map(<$type as FieldType>::view)
    };
}
macro_rules! record {
    ($(#[$doc:meta])* $name:ident { $($(#[$field_doc:meta])* $field:ident : $mode:ident $type:ty => ($phase:ident, $condition:expr, $role:ident);)* }) => {
        $(#[$doc])* #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name { $($(#[$field_doc])* pub $field: field_type!($mode, $type),)* }
        impl $name {
            pub(super) const RULES: &'static [FieldRule] = &[$(FieldRule {
                name: stringify!($field), kind: <$type as FieldType>::KIND, phase: Phase::$phase,
                optional: field_optional!($mode), enabled: $condition, role: FieldRole::$role,
            },)*];
            pub(super) fn from_fields(fields: Vec<Option<FieldValue>>) -> Result<Self> {
                if fields.len() != Self::RULES.len() { return Err(invalid(format!("{} field slots; expected {} for {}", fields.len(), Self::RULES.len(), stringify!($name)))); }
                let mut fields = fields.into_iter();
                Ok(Self { $($field: $mode::<$type>(&mut fields, stringify!($field))?,)* })
            }
            pub(super) fn views(&self) -> Vec<Option<FieldView<'_>>> { vec![$(field_view!($mode, $type, self.$field),)*] }
        }
    };
}

pub(super) use {field_optional, field_type, field_view, record};
