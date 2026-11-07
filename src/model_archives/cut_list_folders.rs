use super::display_records::declared;
use super::{ModelDecoder, browser_folders, custom_properties};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

const BODY_FOLDERS: &[&str] = &[
    "moAtomBodyFolder_c",
    "moFeatSolidBodyFolder_c",
    "moFeatSurfaceBodyFolder_c",
];
const FEATURE_FOLDERS: &[&str] = &[
    "moRefAxisFtrFolder_c",
    "moCoordSysFtrFolder_c",
    "moCurveFtrFolder_c",
    "moMirroredCompFeatFtrFolder_c",
    "moProfileFtrFolder_c",
    "moRefPlaneFtrFolder_c",
    "moRefPointFtrFolder_c",
    "moStockFtrFolder_c",
];

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    if BODY_FOLDERS.contains(&name) {
        return Some(browser_folders::body_folder(reader, map));
    }
    if FEATURE_FOLDERS.contains(&name) {
        // #54 / C-204 binds all eight node hooks and their zero-valued +A40 selector.
        return Some(super::features::feature_group(reader, map, false));
    }
    let result = match name {
        "moSubAtomFolder_c" => sub_atom(reader, map),
        "moSubWeldFolder_c" => sub_weld(reader, map),
        "moCutListFolder_c" => read(reader, map),
        _ => return None,
    };
    Some(result)
}

fn sub_atom(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body_folder", |reader| {
        browser_folders::body_folder(reader, map)
    })?;
    reader.scope("sub_atom", |reader| {
        declared(reader, map, "list_3d8", &["suObList"])?;
        reader.i32("raw_3e0")?;
        declared(
            reader,
            map,
            "feature_reference_3e8",
            &["moFeatureRefWrapper_c"],
        )?;
        reader.i32s(&["raw_3f4", "raw_3e4"])
    })
}

fn sub_weld(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    sub_atom(reader, map)?;
    reader.scope("direct_properties_448", |reader| {
        custom_properties::list(reader, "properties")
    })
}

fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("sub_weld", |reader| sub_weld(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(
        reader,
        map,
        "feature_490",
        super::component_references::FEATURE_CLASSES,
    )?;
    reader.i32s(&["raw_49c", "raw_498"])?;
    reader.u8("raw_484")?;
    declared(reader, map, "plane_460", &["moPlaneRef_w"])?;
    reader.f64s(&["raw_468", "raw_470", "raw_478"])?;
    reader.i32s(&["raw_4a8", "raw_4ac"])
}
