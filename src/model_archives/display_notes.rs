use super::{ModelDecoder, ModelObjectRole, display_item, display_object, raw_fields};
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord, vectors::double_vector};
use crate::{BessemerError, FailureClass, Result};
use raw_fields::RawScalarKind::{Double, Signed, Text, Unsigned, Word};

/// Retain the complete display-item base before the versioned note suffix.
/// Example: `display_notes::read(reader, map)?` at the exact class dispatch.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    display_object::read(reader, map)?;
    reader.scope("item", |reader| display_item::read(reader, map))?;
    reader.scope("note", |reader| suffix(reader, map))
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    raw_fields::fields(reader, Word, "644 648 64c")?;
    double_vector(reader, "vector_628")?;
    reader.f64("raw_630")?;
    reader.i32s(&["raw_690", "raw_694"])?;
    pointer(reader, map, "hyperlink_6a0", "suHyperlink_c")?;
    raw_fields::fields(reader, Text, "658 660 668")?;
    pointer(reader, map, "strings_6b8", "su_CMapStringToString")?;
    pointer(reader, map, "stacked_data_590", "moStackedNoteData_c")?;
    let count = reader.signed_count("flag_chunks_768.count", 4)?;
    for index in 0..count {
        reader.u32(&format!("flag_chunks_768[{index}]"))?;
    }
    middle(reader, map)?;
    tail(reader, map)
}

fn middle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    pointer(reader, map, "chain_5a0", "moChainComponent_c")?;
    reader.f64("raw_698")?;
    reader.text("string_6d8")?;
    reader.object("object_6e0", map, ModelObjectRole::Any)?;
    reader.u16("raw_640")?;
    pointer(reader, map, "bom_5b0", "moBomFeat_c")?;
    raw_fields::groups(
        reader,
        &[
            (Signed, "5b8 5bc"),
            (Double, "710"),
            (Signed, "5c0"),
            (Unsigned, "770"),
            (Signed, "784"),
        ],
    )
}

fn tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x2bbb {
        reader.i32("raw_788")?;
    }
    if reader.version() > 0x2f77 {
        reader.i32("raw_78c")?;
    }
    if reader.version() > 0x2f9a {
        pointer(reader, map, "thread_790", "moCompCThread_c")?;
    }
    if reader.version() > 0x2fb7 {
        pointer(reader, map, "integers_6c0", "su_CMapStringToInt")?;
    }
    Ok(())
}

fn pointer(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    class: &str,
) -> Result<()> {
    reader.object(label, map, ModelObjectRole::Declared(&[class]))
}

/// Select only the independently proven null-component note branch.
/// Example: `display_notes::component(reader, map)?` for `moCompNote_c`.
pub(super) fn component(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let component = reader.scope("component_reference", |reader| {
        super::component_references::base_record(reader, map)
    })?;
    require_null_component(reader, &component)?;
    for label in ["detail_e8", "detail_f8"] {
        reader.object(label, map, ModelObjectRole::Declared(&["moDetailItem_c"]))?;
    }
    reader.i32("raw_100")?;
    Ok(())
}

fn require_null_component(reader: &mut ModelDecoder<'_>, component: &ObjectRecord) -> Result<()> {
    if !matches!(component, ObjectRecord::Null) {
        return reader.checked(Err(BessemerError::new(
            "note component",
            "component reference",
            FailureClass::Unsupported,
            format!("component {component:?}; expected null for the verified internal note branch"),
        )));
    }
    Ok(())
}

/// Compose note data from the shared detail base and its tagged text pointer.
/// Example: `display_notes::note_data(reader, map)?`.
pub(super) fn note_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("detail", |reader| {
        super::component_detail_references::detail_item(reader, map)
    })?;
    reader.object(
        "text_90",
        map,
        ModelObjectRole::Declared(&["moDimText_c", "moGenericText_c"]),
    )
}
