//! Edits a partition WORLD counter; archive checks do not establish kernel/model validity.
use bessemer::block_stream::{BlockStream, ChunkFraming};
use bessemer::parasolid::{
    BaseSchema, FieldKind, FieldValues, TransmitBody, TransmitFile, TransmitLimits,
};
use bessemer::{BessemerError, Document, FailureClass, Result};
use std::error::Error;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, schema_path, stream, highest_id, destination] = arguments.as_slice() else {
        return Err(format!(
            "{} arguments; expected SOURCE BASE_SCHEMA STREAM HIGHEST_ID DESTINATION",
            arguments.len()
        )
        .into());
    };
    let base = BaseSchema::from_bytes(&std::fs::read(schema_path)?)?;
    let mut document = Document::from_bytes(&std::fs::read(source)?)?;
    let carrier = document.read_stream_limited(stream, 128 * 1024 * 1024)?;
    document.replace_stream(
        stream,
        edited_carrier(&carrier, &base, highest_id.parse()?)?,
    )?;
    std::fs::write(destination, document.to_bytes()?)?;
    Ok(())
}

fn edited_carrier(bytes: &[u8], base: &BaseSchema, highest_id: i32) -> Result<Vec<u8>> {
    let mut carrier =
        BlockStream::from_bytes(bytes, ChunkFraming::LengthPrefixed, 256 * 1024 * 1024)?;
    let first = carrier.chunks().first().ok_or_else(|| {
        counter_error(
            FailureClass::Missing,
            "chunk count 0; expected a partition in chunk 0",
        )
    })?;
    let replacement = edited_world(first.payload(), base, highest_id)?;
    carrier.replace_chunk(0, replacement)?;
    carrier.to_bytes()
}

fn edited_world(bytes: &[u8], base: &BaseSchema, highest_id: i32) -> Result<Vec<u8>> {
    let limits = TransmitLimits {
        maximum_records: 1_000_000,
        maximum_field_elements: 10_000_000,
    };
    let source = TransmitFile::from_bytes(bytes, base, limits)?;
    let offset = source
        .layout_fields(101)?
        .iter()
        .position(|field| field.name == "highest_id" && field.kind == FieldKind::Integer)
        .ok_or_else(|| {
            counter_error(
                FailureClass::Missing,
                "WORLD integer field absent; expected highest_id",
            )
        })?;
    let mut contents = source.contents().clone();
    let TransmitBody::Partition(list) = &mut contents.body else {
        return Err(counter_error(
            FailureClass::Unsupported,
            "deltas body; expected partition",
        ));
    };
    let world = list
        .records
        .iter_mut()
        .find(|record| record.node_type == 101)
        .ok_or_else(|| {
            counter_error(
                FailureClass::Missing,
                "WORLD record absent; expected type 101",
            )
        })?;
    let field = world.fields.get_mut(offset).ok_or_else(|| {
        counter_error(
            FailureClass::Invalid,
            "WORLD field absent; expected validated highest_id slot",
        )
    })?;
    *field = FieldValues::Integers(vec![highest_id]);
    source.with_contents(contents)?.to_bytes()
}

fn counter_error(class: FailureClass, cause: &str) -> BessemerError {
    BessemerError {
        operation: "edit WORLD counter",
        entity: "partition WORLD".to_owned(),
        class,
        cause: cause.to_owned(),
    }
}
