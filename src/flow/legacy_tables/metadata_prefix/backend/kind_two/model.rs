use super::*;
pub(super) fn read(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    kind: FlowMediumModelKind,
    remaining: &mut usize,
    maximum_words: usize,
) -> Result<ModelSpan> {
    let start = length - cursor.remaining();
    medium_body::zero(cursor, "kind-two model")?;
    let payload = match kind {
        FlowMediumModelKind::TwoCurves
        | FlowMediumModelKind::ThreeCurves
        | FlowMediumModelKind::ThreeCurvesWithState => curves(cursor, length, kind, remaining)?,
        FlowMediumModelKind::SevenScalars => {
            ModelPayload::Scalars(medium_body::words(cursor, length, 7, remaining)?)
        }
        FlowMediumModelKind::ScalarsWithState => switched(cursor, length, remaining)?,
        FlowMediumModelKind::Tables => {
            ModelPayload::Tables(table(cursor, length, remaining, maximum_words)?)
        }
    };
    Ok(ModelSpan {
        kind,
        start,
        end: length - cursor.remaining(),
        payload,
    })
}
fn curves(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    kind: FlowMediumModelKind,
    remaining: &mut usize,
) -> Result<ModelPayload> {
    let count = if kind == FlowMediumModelKind::TwoCurves {
        2
    } else {
        3
    };
    let curves = medium_body::curves(cursor, length, count, remaining, "kind-two model curve")?;
    let scalars = medium_body::words(cursor, length, 1, remaining)?;
    let count = if kind == FlowMediumModelKind::ThreeCurvesWithState {
        2
    } else {
        0
    };
    let states = bytes(cursor, length, count)?;
    Ok(ModelPayload::Curves {
        curves,
        scalars,
        states,
    })
}
fn switched(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    remaining: &mut usize,
) -> Result<ModelPayload> {
    let first = medium_body::words(cursor, length, 6, remaining)?;
    let state = medium_body::byte_position(cursor, length)?;
    let last = medium_body::words(cursor, length, 1, remaining)?;
    Ok(ModelPayload::Switched { first, state, last })
}
fn table(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    remaining: &mut usize,
    maximum_words: usize,
) -> Result<TableModelSpan> {
    let head = table_head(cursor, length, remaining, maximum_words)?;
    let c = native_count(cursor, "model third array")?;
    let third = medium_body::words(cursor, length, c, remaining)?;
    let member = length - cursor.remaining();
    cursor.i32()?;
    let scalars = medium_body::words(cursor, length, 2, remaining)?;
    let states = bytes(cursor, length, 2)?;
    Ok(TableModelSpan {
        counts: [head.counts[0], head.counts[1], c],
        first: head.first,
        second: head.second,
        words: head.words,
        third,
        member,
        scalars,
        states,
    })
}
fn native_count(cursor: &mut ArchiveCursor<'_>, label: &str) -> Result<usize> {
    let count = cursor.i32()?;
    if count < 0 {
        return Err(metadata_invalid(format!(
            "{label} count {count}; expected 0..=2147483647"
        )));
    }
    Ok(count as usize)
}
fn word_patterns(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    count: usize,
    maximum: usize,
) -> Result<(usize, usize)> {
    if count > maximum || count > cursor.remaining() / 4 {
        return Err(metadata_invalid(format!(
            "model word count {count}; expected at most {maximum} budgeted and {} physical words",
            cursor.remaining() / 4
        )));
    }
    bytes(cursor, length, count * 4)
}
fn bytes(cursor: &mut ArchiveCursor<'_>, length: usize, count: usize) -> Result<(usize, usize)> {
    let start = length - cursor.remaining();
    cursor.take(count)?;
    Ok((start, length - cursor.remaining()))
}

struct TableModelHead {
    counts: [usize; 2],
    first: [(usize, usize); 2],
    second: (usize, usize),
    words: (usize, usize),
}
fn table_head(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    remaining: &mut usize,
    maximum_words: usize,
) -> Result<TableModelHead> {
    let a = native_count(cursor, "model first arrays")?;
    let b = native_count(cursor, "model second arrays")?;
    let first = [
        medium_body::words(cursor, length, a, remaining)?,
        medium_body::words(cursor, length, a, remaining)?,
    ];
    let second = medium_body::words(cursor, length, b, remaining)?;
    let words = word_patterns(cursor, length, b, maximum_words)?;
    Ok(TableModelHead {
        counts: [a, b],
        first,
        second,
        words,
    })
}
