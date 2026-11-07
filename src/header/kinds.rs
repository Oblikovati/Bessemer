use crate::Result;
use crate::archive_codec::ArchiveClass;

use super::fields::{FieldRule, FieldValue, FieldView};
use super::{
    ExternalFileReference, HeaderObject, HeaderRecord, LogRecord, ObjectList, StringArray, invalid,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ClassKind {
    Header,
    Strings,
    List,
    ExternalList,
    Log,
    Stamp,
    Handle,
    External,
}

impl ClassKind {
    pub fn parse(class: &ArchiveClass) -> Result<Self> {
        match class.name.as_str() {
            "moHeader_c" => Ok(Self::Header),
            "su_CStringArray" => Ok(Self::Strings),
            "suObList" => Ok(Self::List),
            "moExtObjectList_c" => Ok(Self::ExternalList),
            "moLogs_c" => Ok(Self::Log),
            "moStamp_c" => Ok(Self::Stamp),
            "moCStringHandle_c" => Ok(Self::Handle),
            "moExtObject_c" => Ok(Self::External),
            other => Err(invalid(format!(
                "class {other:?}; expected a documented header-family class"
            ))),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Header => "moHeader_c",
            Self::Strings => "su_CStringArray",
            Self::List => "suObList",
            Self::ExternalList => "moExtObjectList_c",
            Self::Log => "moLogs_c",
            Self::Stamp => "moStamp_c",
            Self::Handle => "moCStringHandle_c",
            Self::External => "moExtObject_c",
        }
    }

    pub fn rules(self) -> &'static [FieldRule] {
        match self {
            Self::Header => HeaderRecord::RULES,
            Self::Strings => StringArray::RULES,
            Self::List | Self::ExternalList => ObjectList::RULES,
            Self::Log => LogRecord::RULES,
            Self::External => ExternalFileReference::RULES,
            Self::Stamp | Self::Handle => &[],
        }
    }

    pub fn assemble(self, fields: Vec<Option<FieldValue>>) -> Result<HeaderObject> {
        match self {
            Self::Header => Ok(HeaderObject::Header(Box::new(HeaderRecord::from_fields(
                fields,
            )?))),
            Self::Strings => Ok(HeaderObject::Strings(StringArray::from_fields(fields)?)),
            Self::List => Ok(HeaderObject::List(ObjectList::from_fields(fields)?)),
            Self::ExternalList => Ok(HeaderObject::ExternalList(ObjectList::from_fields(fields)?)),
            Self::Log => Ok(HeaderObject::Log(LogRecord::from_fields(fields)?)),
            Self::External => Ok(HeaderObject::External(Box::new(
                ExternalFileReference::from_fields(fields)?,
            ))),
            other => Err(invalid(format!(
                "class {other:?}; expected field-based body"
            ))),
        }
    }
}

impl HeaderObject {
    pub(super) fn kind(&self) -> ClassKind {
        match self {
            Self::Header(_) => ClassKind::Header,
            Self::Strings(_) => ClassKind::Strings,
            Self::List(_) => ClassKind::List,
            Self::ExternalList(_) => ClassKind::ExternalList,
            Self::Log(_) => ClassKind::Log,
            Self::Stamp(_) => ClassKind::Stamp,
            Self::StringHandle(_) => ClassKind::Handle,
            Self::External(_) => ClassKind::External,
        }
    }

    pub(super) fn views(&self) -> Vec<Option<FieldView<'_>>> {
        match self {
            Self::Header(record) => record.views(),
            Self::Strings(record) => record.views(),
            Self::List(record) | Self::ExternalList(record) => record.views(),
            Self::Log(record) => record.views(),
            Self::External(record) => record.views(),
            Self::Stamp(_) | Self::StringHandle(_) => Vec::new(),
        }
    }
}
