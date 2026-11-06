use super::*;

pub(super) fn offset(
    owner: &FlowLinkRecords,
    field: FlowLinkRecordField,
) -> Result<(usize, usize)> {
    use FlowLinkRecordField::*;
    match field {
        EntryReference { entry, component } => at(entry_span(owner, entry)?.start, component, 2, 4),
        EntryWide { entry, index } => at(entry_span(owner, entry)?.start + 8, index, 7, 8),
        EntryCode { entry } => Ok((entry_span(owner, entry)?.start + 64, 1)),
        EntryInteger { entry, index } => entry_integer(owner, entry, index),
        TailMember { index } => at(owner.tail.start, index, 11, 4),
        Discarded { index } => integer_interval(owner.tail.discarded, index),
        Control => owner
            .tail
            .control
            .map(|o| (o, 4))
            .ok_or_else(|| missing(field)),
        Conditional { index } => integer_interval(owner.tail.conditional, index),
        NumericWide { index } => numeric(owner, index, false),
        NumericIndex { index } => numeric(owner, index, true),
        FinalValue { index } => final_value(owner, index),
    }
}

fn entry_span(owner: &FlowLinkRecords, entry: usize) -> Result<&EntrySpan> {
    owner
        .entries
        .get(entry)
        .ok_or_else(|| table_index_error(402, "entry", entry, owner.entries.len()))
}

fn entry_integer(owner: &FlowLinkRecords, entry: usize, index: usize) -> Result<(usize, usize)> {
    let span = entry_span(owner, entry)?;
    at(span.start + 66, index, span.variable_count, 4)
}

fn integer_interval((start, end): (usize, usize), index: usize) -> Result<(usize, usize)> {
    at(start, index, (end - start) / 4, 4)
}

fn at(start: usize, index: usize, count: usize, width: usize) -> Result<(usize, usize)> {
    if index >= count {
        return Err(table_index_error(402, "field component", index, count));
    }
    Ok((start + index * width, width))
}

fn numeric(owner: &FlowLinkRecords, index: usize, indices: bool) -> Result<(usize, usize)> {
    let field = if indices {
        FlowLinkRecordField::NumericIndex { index }
    } else {
        FlowLinkRecordField::NumericWide { index }
    };
    let span = owner.tail.numeric.as_ref().ok_or_else(|| missing(field))?;
    if indices {
        return at(
            span.indices.ok_or_else(|| missing(field))?,
            index,
            span.count,
            4,
        );
    }
    at(span.start + 4, index, span.count, 8)
}

fn final_value(owner: &FlowLinkRecords, index: usize) -> Result<(usize, usize)> {
    let field = FlowLinkRecordField::FinalValue { index };
    let span = owner
        .tail
        .final_tail
        .as_ref()
        .ok_or_else(|| missing(field))?;
    at(span.start + 4, index, span.count, span.width)
}

fn missing(field: FlowLinkRecordField) -> BessemerError {
    table_failure(
        402,
        FailureClass::Missing,
        format!("field {field:?} absent; expected an existing serialized field"),
    )
}
