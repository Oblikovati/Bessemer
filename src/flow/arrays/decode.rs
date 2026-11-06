use super::*;

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowArrayOptions,
) -> Result<FlowMapArrays> {
    validate_options(options)?;
    let compressed = record_payload(source, id)?;
    let decoded = super::super::payload_codec::inflate_zlib(
        compressed,
        options.maximum_decoded_bytes,
        "Flow record 433",
    )?;
    let (selector, count) = array_shape(&decoded, options)?;
    let (first, second) = split_arrays(&decoded, selector, count);
    Ok(FlowMapArrays {
        source: source.clone(),
        record: id.clone(),
        selector,
        first,
        second,
        decoded: Arc::from(decoded),
        compressed_bytes: compressed.len(),
        options,
    })
}

pub(super) fn record_payload<'a>(source: &'a FlowCarrier, id: &FlowRecordId) -> Result<&'a [u8]> {
    super::super::payload_codec::record_payload(source, id, 433, "Flow record 433")
}

fn validate_options(options: FlowArrayOptions) -> Result<()> {
    let invalid_count = options
        .expected_count
        .is_some_and(|count| count > i32::MAX as u32);
    if options.maximum_decoded_bytes < 8 || options.maximum_elements == 0 || invalid_count {
        return Err(array_failure(
            "options",
            FailureClass::Invalid,
            format!(
                "{options:?}; expected decoded bound >=8, positive element bound and optional count <=i32::MAX"
            ),
        ));
    }
    Ok(())
}

fn array_shape(bytes: &[u8], options: FlowArrayOptions) -> Result<(i32, usize)> {
    let (selector, count) = selector_count(bytes)?;
    check_counts(selector, count, options)?;
    let count = count as usize;
    let width = if selector == 0 { 8 } else { 4 };
    if count > (bytes.len() - 8) / width || bytes.len() != 8 + count * width {
        return Err(array_failure(
            "array frames",
            FailureClass::Invalid,
            format!(
                "selector {selector}, count {count}, {} bytes; expected exactly eight plus count times {width}",
                bytes.len()
            ),
        ));
    }
    Ok((selector, count))
}

fn selector_count(bytes: &[u8]) -> Result<(i32, i32)> {
    if bytes.len() < 8 {
        return Err(array_failure(
            "selector/count",
            FailureClass::Invalid,
            format!("{} decoded bytes; expected at least eight", bytes.len()),
        ));
    }
    let selector = signed_word(&bytes[..4]);
    let count = signed_word(&bytes[4..8]);
    Ok((selector, count))
}

fn check_counts(selector: i32, count: i32, options: FlowArrayOptions) -> Result<()> {
    if selector < 0 || count < 0 || count as usize > options.maximum_elements {
        return Err(array_failure(
            "selector/count",
            FailureClass::Invalid,
            format!(
                "selector {selector}, count {count}; expected nonnegative values and count <={}",
                options.maximum_elements
            ),
        ));
    }
    check_runtime_count(count, options.expected_count)
}

fn check_runtime_count(count: i32, expected_count: Option<u32>) -> Result<()> {
    if expected_count.is_some_and(|expected| expected != count as u32) {
        return Err(array_failure(
            "runtime count",
            FailureClass::Invalid,
            format!(
                "stored count {count}, supplied {:?}; expected exact equality",
                expected_count
            ),
        ));
    }
    Ok(())
}

type SignedArrays = (Option<Arc<[i32]>>, Arc<[i32]>);

fn split_arrays(bytes: &[u8], selector: i32, count: usize) -> SignedArrays {
    let (first, position) = if selector == 0 {
        (Some(read_array(&bytes[8..8 + count * 4])), 8 + count * 4)
    } else {
        (None, 8)
    };
    (first, read_array(&bytes[position..]))
}

fn read_array(bytes: &[u8]) -> Arc<[i32]> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .copied()
        .map(i32::from_le_bytes)
        .collect()
}

fn signed_word(bytes: &[u8]) -> i32 {
    i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
