//! Edits one archived display colour; archive checks do not certify geometry or native acceptance.
use bessemer::tessellation::*;
use bessemer::{BessemerError, Document, FailureClass};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [source, version, stream, component, face, color, destination] = arguments.as_slice()
    else {
        return Err(format!("{} arguments; expected SOURCE VERSION STREAM COMPONENT_INDEX FACE_INDEX COLOR DESTINATION", arguments.len()).into());
    };
    let mut document = Document::from_bytes(&std::fs::read(source)?)?;
    let bytes = document.read_stream_limited(stream, 256 * 1024 * 1024)?;
    let context = TessellationContext {
        archive_version: ArchiveVersion::new(version.parse()?),
        mode: FacetArchiveMode::Full,
    };
    let block = FaceTessellationBlock::from_bytes(&bytes, context, limits())?;
    let replacement = recolored(&block, component.parse()?, face.parse()?, color.parse()?)?;
    document.replace_stream(stream, replacement)?;
    std::fs::write(destination, document.to_bytes()?)?;
    Ok(())
}

fn recolored(
    block: &FaceTessellationBlock,
    component: i32,
    index: usize,
    color: u32,
) -> bessemer::Result<Vec<u8>> {
    let mut contents = block.contents().clone();
    let entry = contents
        .entries
        .iter_mut()
        .find(|entry| entry.index_in_block == component)
        .ok_or_else(|| {
            edit_error(format!(
                "component index {component}; expected a stored block index"
            ))
        })?;
    recolor_face(entry, index, color)?;
    block.with_contents(contents)?.to_bytes()
}

fn recolor_face(entry: &mut FacetBlockComponent, index: usize, color: u32) -> bessemer::Result<()> {
    let component = entry.index_in_block;
    let face = entry.faces.get_mut(index).ok_or_else(|| {
        edit_error(format!(
            "face index {index}; expected an existing face in component {component}"
        ))
    })?;
    let FacetVertexLayout::Styled { style, .. } = &mut face.vertex_layout else {
        return Err(edit_error(format!(
            "face index {index} has no stored style; expected styled vertex layout"
        )));
    };
    style.face_color = color;
    Ok(())
}

fn limits() -> TessellationLimits {
    TessellationLimits {
        maximum_components: 1_000_000,
        maximum_faces: 1_000_000,
        maximum_array_elements: 100_000_000,
    }
}

fn edit_error(cause: String) -> BessemerError {
    BessemerError {
        operation: "edit facet colour",
        entity: "block face style".into(),
        class: FailureClass::Invalid,
        cause,
    }
}
