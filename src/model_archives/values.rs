use std::sync::Arc;

use crate::Result;
use crate::archive_codec::{
    ArchiveString, CompactTransform, FramedObjectRecord, StoredDouble, StoredFloat,
    TransformContents, transform_codec,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ModelFieldValue {
    U8(u8),
    U16(u16),
    I16(i16),
    I32(i32),
    U32(u32),
    U64(u64),
    F32(StoredFloat),
    F64(StoredDouble),
    Text(ArchiveString),
    ObjectTag(Box<FramedObjectRecord>),
    Transform(Box<CompactTransform>),
    Kernel(Box<super::kernel_frames::KernelFrame>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ModelField {
    pub path: Arc<str>,
    pub value: ModelFieldValue,
}

impl ModelFieldValue {
    fn encoded_size(&self) -> Result<usize> {
        Ok(match self {
            Self::U8(_) => 1,
            Self::I16(_) | Self::U16(_) => 2,
            Self::I32(_) | Self::U32(_) | Self::F32(_) => 4,
            Self::F64(_) | Self::U64(_) => 8,
            Self::Text(value) => return value.encoded_size(),
            Self::ObjectTag(value) => return super::objects::tag_size(value),
            Self::Transform(value) => return transform_size(value),
            Self::Kernel(value) => {
                return Ok(value.to_bytes().map_err(super::model_failure)?.len());
            }
        })
    }

    fn encode(&self, output: &mut Vec<u8>) -> Result<()> {
        match self {
            Self::U8(value) => output.push(*value),
            Self::U16(value) => output.extend_from_slice(&value.to_le_bytes()),
            Self::I16(value) => output.extend_from_slice(&value.to_le_bytes()),
            Self::I32(value) => output.extend_from_slice(&value.to_le_bytes()),
            Self::U32(value) => output.extend_from_slice(&value.to_le_bytes()),
            Self::U64(value) => output.extend_from_slice(&value.to_le_bytes()),
            Self::F32(value) => output.extend_from_slice(&value.bits().to_le_bytes()),
            Self::F64(value) => output.extend_from_slice(&value.bits().to_le_bytes()),
            Self::Text(value) => return value.encode(output),
            Self::ObjectTag(value) => return super::objects::encode_tag(value, output),
            Self::Transform(value) => transform_codec::write_compact(value, output),
            Self::Kernel(value) => {
                output.extend_from_slice(&value.to_bytes().map_err(super::model_failure)?)
            }
        }
        Ok(())
    }
}

fn transform_size(value: &CompactTransform) -> Result<usize> {
    transform_codec::validate_compact(value).map_err(super::model_failure)?;
    Ok(transform_codec::encode(&TransformContents::Compact(value.clone())).len())
}

pub(super) fn encode_fields(fields: &[ModelField], maximum_bytes: usize) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    for field in fields {
        let available = maximum_bytes.saturating_sub(output.len());
        let required = field.value.encoded_size()?;
        if required > available {
            return Err(super::invalid(format!(
                "{} encoded bytes {required}; expected <= {available} remaining output bytes",
                field.path
            )));
        }
        field.value.encode(&mut output)?;
    }
    Ok(output)
}
