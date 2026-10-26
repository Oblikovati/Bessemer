use super::{ModelDecoder, ModelFieldValue, invalid};
use crate::archive_codec::MaterialRead;
use crate::block_stream::{BlockStream, ChunkFraming};
use crate::parasolid::{
    BaseSchema, LayoutDefinition, TransmitBody, TransmitContents, TransmitFile, TransmitFormat,
    TransmitLimits,
};
use crate::{BessemerError, FailureClass, Result};

#[derive(Clone, Debug)]
pub(super) struct CompressedKernelContext {
    pub base: BaseSchema,
    pub maximum_expanded: usize,
    pub transmit_limits: TransmitLimits,
}

#[derive(Clone, Debug)]
pub(super) struct KernelFrame {
    base: BaseSchema,
    carrier: BlockStream,
    transmit: TransmitFile,
}

impl PartialEq for KernelFrame {
    fn eq(&self, other: &Self) -> bool {
        // Reparse changes compression provenance; compare the typed contents and emitted frame.
        let (Ok(left), Ok(right)) = (self.to_bytes(), other.to_bytes()) else {
            return false;
        };
        self.base == other.base
            && self.transmit.contents() == other.transmit.contents()
            && left == right
    }
}

impl Eq for KernelFrame {}

pub(super) fn read_body(reader: &mut ModelDecoder<'_>) -> Result<()> {
    read_body_state(reader).map(|_| ())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum KernelBodyState {
    Suppressed,
    Serialized,
}

pub(super) fn read_body_state(reader: &mut ModelDecoder<'_>) -> Result<KernelBodyState> {
    reader.require_ordinary_part()?;
    if reader.u8("suppressed")? != 0 {
        return Ok(KernelBodyState::Suppressed);
    }
    let context = constrained_context(reader)?;
    let frame = reader.scalar(|cursor| {
        let length = cursor.i32()?;
        decode_frame(length, cursor, &context)
    })?;
    reader.charge_kernel(frame.expanded_bytes(), frame.record_count())?;
    reader.field("frame", ModelFieldValue::Kernel(Box::new(frame)))?;
    Ok(KernelBodyState::Serialized)
}

fn constrained_context(reader: &mut ModelDecoder<'_>) -> Result<CompressedKernelContext> {
    let context = reader.kernel_context().cloned().ok_or_else(|| {
        BessemerError::new(
            "model archive",
            "embedded kernel context",
            FailureClass::Missing,
            "context None; expected explicit framed compressed Parasolid schema and limits",
        )
    });
    let mut context = reader.checked(context)?;
    context.maximum_expanded = reader.kernel_allowance();
    context.transmit_limits.maximum_records = context
        .transmit_limits
        .maximum_records
        .min(reader.remaining_entries());
    Ok(context)
}

fn decode_frame(
    length: i32,
    cursor: &mut crate::archive_codec::ArchiveCursor<'_>,
    context: &CompressedKernelContext,
) -> Result<KernelFrame> {
    if length < 29 {
        return Err(invalid(format!(
            "kernel frame length {length}; expected >= 29 including its type byte"
        )));
    }
    let kind = cursor.u8()?;
    if kind != 0 {
        return Err(BessemerError::new(
            "model archive",
            "kernel type",
            FailureClass::Unsupported,
            format!("kernel type {kind}; expected neutral Parasolid type 0"),
        ));
    }
    KernelFrame::from_payload(cursor.take(length as usize - 1)?, context)
}

impl KernelFrame {
    pub fn expanded_bytes(&self) -> usize {
        self.carrier
            .chunks()
            .iter()
            .map(|chunk| chunk.payload().len())
            .sum()
    }

    pub fn record_count(&self) -> usize {
        match &self.transmit.contents().body {
            TransmitBody::Partition(list) => list.records.len(),
            TransmitBody::Deltas(_) => 0,
        }
    }
    pub fn from_payload(bytes: &[u8], context: &CompressedKernelContext) -> Result<Self> {
        let carrier = single_carrier(bytes, context.maximum_expanded)?;
        let chunk = carrier
            .chunks()
            .first()
            .ok_or_else(|| invalid("kernel chunks empty; expected one"))?;
        let transmit = TransmitFile::from_embedded_partition(
            chunk.payload(),
            &context.base,
            context.transmit_limits,
        )?;
        validate_body(&context.base, transmit.contents())?;
        let frame = Self {
            base: context.base.clone(),
            carrier,
            transmit,
        };
        frame.to_bytes()?;
        Ok(frame)
    }

    pub fn contents(&self) -> &TransmitContents {
        self.transmit.contents()
    }

    pub fn layout_fields(&self, code: u16) -> Result<&[crate::parasolid::FieldDefinition]> {
        self.transmit.layout_fields(code)
    }

    pub fn with_contents(&self, contents: TransmitContents) -> Result<Self> {
        let transmit = TransmitFile::new_embedded_partition(&self.base, contents)?;
        validate_body(&self.base, transmit.contents())?;
        let mut carrier = self.carrier.clone();
        carrier.replace_chunk(0, transmit.to_bytes()?)?;
        let frame = Self {
            base: self.base.clone(),
            carrier,
            transmit,
        };
        frame.to_bytes()?;
        Ok(frame)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let typed =
            TransmitFile::new_embedded_partition(&self.base, self.transmit.contents().clone())?
                .to_bytes()?;
        let mut carrier = self.carrier.clone();
        carrier.replace_chunk(0, typed)?;
        let payload = carrier.to_bytes()?;
        let length = i32::try_from(payload.len().saturating_add(1)).map_err(|_| {
            invalid(format!(
                "kernel payload bytes {}; expected length including type within i32",
                payload.len()
            ))
        })?;
        let mut output = length.to_le_bytes().to_vec();
        output.push(0);
        output.extend_from_slice(&payload);
        Ok(output)
    }
}

fn single_carrier(bytes: &[u8], maximum_expanded: usize) -> Result<BlockStream> {
    let carrier = BlockStream::from_bytes(bytes, ChunkFraming::LengthPrefixed, maximum_expanded)?;
    if carrier.chunks().len() != 1 {
        return Err(invalid(format!(
            "kernel chunks {}; expected exactly one",
            carrier.chunks().len()
        )));
    }
    Ok(carrier)
}

fn validate_body(base: &BaseSchema, contents: &TransmitContents) -> Result<()> {
    let TransmitBody::Partition(list) = &contents.body else {
        return Err(invalid("kernel body deltas; expected a complete partition"));
    };
    let count = list
        .records
        .iter()
        .filter(|record| is_body_type(base, contents, record.node_type))
        .count();
    if contents.format != TransmitFormat::Neutral || count != 1 {
        return Err(invalid(format!(
            "kernel format {:?}, BODY count {count}; expected neutral partition with exactly one BODY",
            contents.format
        )));
    }
    Ok(())
}

fn is_body_type(base: &BaseSchema, contents: &TransmitContents, code: u16) -> bool {
    match contents.layouts.get(&code) {
        Some(LayoutDefinition::Defined { name, .. }) => name == "BODY",
        _ => base
            .node_types()
            .get(&code)
            .is_some_and(|node| node.name == "BODY"),
    }
}
