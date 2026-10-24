use super::{FacetContainer, TessellationContext, TessellationLimits, invalid};
use super::{codec::FacetCodec, context::FacetState, output::FacetOutput};
use crate::{Result, archive_codec::ArchiveCursor};

pub(crate) fn read_embedded(
    cursor: &mut ArchiveCursor<'_>,
    context: TessellationContext,
    maximum_entries: usize,
) -> Result<(FacetContainer, usize)> {
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: maximum_entries,
    };
    let mut state = FacetState::with_element_budget(context, limits, maximum_entries);
    let container = FacetContainer::read(cursor, &mut state)?;
    Ok((container, state.element_count()))
}

pub(crate) fn write_embedded(
    container: &FacetContainer,
    context: TessellationContext,
    maximum_bytes: usize,
) -> Result<Vec<u8>> {
    let mut sizing = FacetOutput::validation();
    container.write(&mut sizing, &mut FacetState::writing(context))?;
    let size = sizing.encoded_size()?;
    if size > maximum_bytes {
        return Err(invalid(format!(
            "encoded facet bytes {size}; expected at most {maximum_bytes}"
        )));
    }
    let mut output = FacetOutput::encoded();
    container.write(&mut output, &mut FacetState::writing(context))?;
    Ok(output.finish())
}
