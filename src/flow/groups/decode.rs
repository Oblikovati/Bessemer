use super::*;

struct ParsedGroups {
    version: FlowGroupVersion,
    spans: Vec<GroupSpan>,
    members: usize,
}

fn version_count(bytes: &[u8]) -> Result<(FlowGroupVersion, i32)> {
    if bytes.len() < 8 {
        return Err(group_failure(
            "version/group count",
            FailureClass::Invalid,
            format!("{} bytes; expected at least eight", bytes.len()),
        ));
    }
    let version = parse_version(fields::word(bytes, 0) as i32)?;
    Ok((version, fields::word(bytes, 4) as i32))
}

fn require_end(position: usize, length: usize) -> Result<()> {
    if position == length {
        return Ok(());
    }
    Err(group_failure(
        "decoded EOF",
        FailureClass::Invalid,
        format!("group end {position}, {length} bytes; expected exact complete payload EOF"),
    ))
}

fn group_count(bytes: &[u8], start: usize, version: FlowGroupVersion) -> Result<(usize, i32)> {
    let header = if version == FlowGroupVersion::V4 {
        8
    } else {
        4
    };
    if bytes.len() - start < header {
        return Err(group_failure(
            "group header",
            FailureClass::Invalid,
            format!(
                "{} bytes at {start}; expected {header}-byte group header",
                bytes.len() - start
            ),
        ));
    }
    Ok((start + header, fields::word(bytes, start) as i32))
}

fn check_member_extent(
    count: i32,
    available: usize,
    maximum: usize,
    width: usize,
    start: usize,
) -> Result<()> {
    if count < 0 || count as usize > maximum || count as usize > available / width {
        return Err(group_failure(
            "member count",
            FailureClass::Invalid,
            format!(
                "count {count} at {start}; expected nonnegative count <= {maximum} and complete {width}-byte members"
            ),
        ));
    }
    Ok(())
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowGroupOptions,
) -> Result<FlowGroupedRecord> {
    validate_options(options)?;
    let payload = super::super::payload_codec::record_payload(source, id, 404, "Flow record 404")?;
    let decoded = expand(payload, options)?;
    let (version, count) = header(&decoded, options.maximum_groups)?;
    let parsed = scan_groups(&decoded, version, count, options.maximum_members)?;
    Ok(published(
        source,
        id,
        options,
        payload.len(),
        decoded,
        parsed,
    ))
}

fn published(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowGroupOptions,
    stored_bytes: usize,
    decoded: Vec<u8>,
    parsed: ParsedGroups,
) -> FlowGroupedRecord {
    FlowGroupedRecord(Arc::new(GroupRevision {
        source: source.clone(),
        record: id.clone(),
        decoded: Arc::from(decoded),
        spans: parsed.spans,
        version: parsed.version,
        options,
        stored_bytes,
        members: parsed.members,
    }))
}

fn validate_options(options: FlowGroupOptions) -> Result<()> {
    if options.maximum_decoded_bytes < 8
        || options.maximum_groups == 0
        || options.maximum_members == 0
    {
        return Err(group_failure(
            "options",
            FailureClass::Invalid,
            format!("{options:?}; expected decoded bound >=8 and positive group/member bounds"),
        ));
    }
    Ok(())
}

fn expand(payload: &[u8], options: FlowGroupOptions) -> Result<Vec<u8>> {
    if options.compression == FlowGroupCompression::Zlib {
        return super::super::payload_codec::inflate_zlib(
            payload,
            options.maximum_decoded_bytes,
            "Flow record 404",
        );
    }
    if payload.len() > options.maximum_decoded_bytes {
        return Err(group_failure(
            "raw payload",
            FailureClass::Invalid,
            format!(
                "{} bytes; expected at most {}",
                payload.len(),
                options.maximum_decoded_bytes
            ),
        ));
    }
    Ok(payload.to_vec())
}

fn header(bytes: &[u8], maximum: usize) -> Result<(FlowGroupVersion, usize)> {
    let (version, count) = version_count(bytes)?;
    let minimum = if version == FlowGroupVersion::V4 {
        8
    } else {
        4
    };
    if count < 0 || count as usize > maximum || count as usize > (bytes.len() - 8) / minimum {
        return Err(group_failure(
            "group count",
            FailureClass::Invalid,
            format!(
                "count {count}, {} bytes; expected nonnegative count <= {maximum} with {minimum} bytes per group",
                bytes.len()
            ),
        ));
    }
    Ok((version, count as usize))
}

fn parse_version(version: i32) -> Result<FlowGroupVersion> {
    match version {
        3 => Ok(FlowGroupVersion::V3),
        4 => Ok(FlowGroupVersion::V4),
        5 => Ok(FlowGroupVersion::V5),
        6 => Ok(FlowGroupVersion::V6),
        _ => Err(group_failure(
            "payload version",
            FailureClass::Unsupported,
            format!("version {version}; expected scoped versions 3..6, without delegated layouts"),
        )),
    }
}

fn scan_groups(
    bytes: &[u8],
    version: FlowGroupVersion,
    count: usize,
    maximum_members: usize,
) -> Result<ParsedGroups> {
    let mut spans = Vec::with_capacity(count);
    let mut position = 8;
    let mut members = 0;
    for _ in 0..count {
        let span = next_span(bytes, position, version, maximum_members - members)?;
        position = span.end;
        members += span.members;
        spans.push(span);
    }
    require_end(position, bytes.len())?;
    Ok(ParsedGroups {
        version,
        spans,
        members,
    })
}

fn next_span(
    bytes: &[u8],
    start: usize,
    version: FlowGroupVersion,
    remaining_members: usize,
) -> Result<GroupSpan> {
    let (values, count) = group_count(bytes, start, version)?;
    let width = if fields::has_flags(version) { 13 } else { 12 };
    check_member_extent(count, bytes.len() - values, remaining_members, width, start)?;
    Ok(GroupSpan {
        start,
        end: values + count as usize * width,
        values,
        members: count as usize,
    })
}
