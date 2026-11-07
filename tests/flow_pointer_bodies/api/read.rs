use super::*;

#[test]
fn every_frozen_pool_body_interval_context_and_pattern_matches_independent_oracle() {
    for case in cases() {
        let bodies = read_case(&case);
        assert_eq!(bodies.version(), case.version, "{}", case.frame.name);
        assert_eq!(bodies.decoded_bytes(), case.frame.decoded);
        check_context(&bodies, &case.frame);
        assert_eq!(bodies.pools().len(), case.pools.len());
        for pool in &case.pools {
            check_pool(bodies.pool(pool.ordinal as usize).unwrap(), pool);
        }
        for group in &case.groups {
            check_group(&bodies, group);
        }
        check_coverage(&bodies, &case);
        assert!(bodies.pool(case.pools.len()).is_none());
        assert!(bodies.entry(3, 0).is_none());
        assert!(bodies.group_interval(3).is_none());
        assert!(bodies.entry_count(3).is_none());
    }
}

fn check_context(bodies: &FlowPointerBodies, frame: &PointerFrame) {
    assert_eq!(bodies.options().leaf_count.value(), frame.leaf);
    assert_eq!(bodies.options().group_counts.values(), frame.counts);
    assert_eq!(
        bodies.options().entry_codes.as_deref(),
        frame.codes.as_deref()
    );
}

fn check_pool(actual: FlowPointerBodyPool<'_>, expected: &PointerPool) {
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(actual.triplet_count() as u32, expected.count);
    check_wide(actual.wide_words(), &expected.bits);
}

fn check_wide(actual: FlowTableWideArray<'_>, expected: &[u64]) {
    assert_eq!(actual.word_count(), expected.len());
    for (index, bits) in expected.iter().enumerate() {
        assert_eq!(actual.raw_bits(index), Some(*bits));
    }
    assert_eq!(actual.raw_bits(expected.len()), None);
    assert_eq!(actual.as_bytes().len(), expected.len() * 8);
}

fn check_group(bodies: &FlowPointerBodies, group: &PointerGroup) {
    let ordinal = group.ordinal as usize;
    assert_eq!(
        bodies.group_interval(ordinal),
        Some((group.start as usize, group.end as usize))
    );
    assert_eq!(bodies.entry_count(ordinal), Some(group.count as usize));
    for entry in &group.entries {
        let actual = bodies.entry(ordinal, entry.ordinal as usize).unwrap();
        assert_eq!(actual.path(), (ordinal, entry.ordinal as usize));
        assert_eq!(
            actual.payload_interval(),
            (entry.start as usize, entry.end as usize)
        );
        assert_eq!(actual.runtime_code(), entry.code);
        check_body(actual, &entry.body);
    }
    assert!(bodies.entry(ordinal, group.count as usize).is_none());
}

fn check_body(actual: FlowPointerBodyEntry<'_>, body: &PointerEntryBody) {
    check_body_branch(actual, body);
    if !matches!(body, PointerEntryBody::Tagged(_)) {
        assert_eq!(actual.tag_byte(), None);
        assert_eq!(actual.packed_payload_decoded(), None);
        assert_eq!(actual.raw_payload(), None);
    }
}

fn check_body_branch(actual: FlowPointerBodyEntry<'_>, body: &PointerEntryBody) {
    match body {
        PointerEntryBody::Single(bytes) => {
            check_vectors(actual, &[bytes], FlowPointerBodyKind::SingleVector)
        }
        PointerEntryBody::Wide { count, bits } => {
            assert_eq!(actual.kind(), FlowPointerBodyKind::WideTriplets);
            assert_eq!(actual.stored_count(), Some(*count));
            check_wide(actual.wide_words().unwrap(), bits);
        }
        PointerEntryBody::Multiple(vectors) => {
            check_vectors(
                actual,
                &vectors.iter().collect::<Vec<_>>(),
                FlowPointerBodyKind::MultipleVectors,
            );
            assert_eq!(actual.stored_count(), Some(vectors.len() as u32));
        }
        PointerEntryBody::Tagged(tagged) => check_tagged(actual, tagged),
    }
}

fn check_vectors(
    actual: FlowPointerBodyEntry<'_>,
    vectors: &[&Vec<u8>],
    kind: FlowPointerBodyKind,
) {
    assert_eq!(actual.kind(), kind);
    assert_eq!(actual.vector_count(), vectors.len());
    assert!(actual.wide_words().is_none());
    for (index, bytes) in vectors.iter().enumerate() {
        assert_eq!(actual.vector(index), Some(bytes.as_slice()));
        let (start, end) = actual.vector_interval(index).unwrap();
        assert_eq!(end - start, bytes.len());
    }
    assert!(actual.vector(vectors.len()).is_none());
}

fn check_tagged(actual: FlowPointerBodyEntry<'_>, tagged: &TaggedPointerBody) {
    assert_eq!(actual.tag_byte(), Some(tagged.tag));
    assert_eq!(actual.stored_count(), tagged.count);
    assert_eq!(actual.packed_payload_decoded(), tagged.packed_decoded);
    assert_eq!(actual.raw_payload(), tagged.raw.as_deref());
    assert_eq!(actual.vector_count(), 0);
    match &tagged.bits {
        Some(bits) => {
            assert_eq!(actual.kind(), FlowPointerBodyKind::TaggedWide);
            check_wide(actual.wide_words().unwrap(), bits);
        }
        None => check_tagged_byte_kind(actual, tagged),
    }
    if let Some(raw) = &tagged.raw {
        let (start, end) = actual.raw_interval().unwrap();
        assert_eq!(end - start, raw.len());
    }
}

fn check_tagged_byte_kind(actual: FlowPointerBodyEntry<'_>, tagged: &TaggedPointerBody) {
    let expected = if tagged.raw.is_some() {
        FlowPointerBodyKind::TaggedRaw
    } else {
        FlowPointerBodyKind::UnknownTag
    };
    assert_eq!(actual.kind(), expected);
    assert!(actual.wide_words().is_none());
}

fn check_coverage(actual: &FlowPointerBodies, case: &PointerCase) {
    let coverage = actual.coverage();
    let totals = expected_totals(case);
    assert_eq!(coverage.pools, totals.0);
    assert_eq!(coverage.entries, totals.1);
    assert_eq!(coverage.vectors, totals.2);
    assert_eq!(coverage.wide_words, totals.3);
    assert_eq!(coverage.raw_bytes, totals.4);
    assert_eq!(
        (coverage.stored_bytes, coverage.decoded_bytes),
        (case.frame.stored.len(), case.frame.decoded.len())
    );
    assert_eq!(coverage.achieved_error, 0);
    check_transport_mode(coverage.mode, case.frame.compressed);
    check_exclusions(actual);
}

fn check_transport_mode(actual: FlowLegacyTableCheckMode, compressed: bool) {
    assert_eq!(
        actual,
        if compressed {
            FlowLegacyTableCheckMode::ExactZlibTable
        } else {
            FlowLegacyTableCheckMode::ExactRawTable
        }
    );
}

fn check_exclusions(actual: &FlowPointerBodies) {
    assert_eq!(actual.diagnostics().len(), 7);
    assert!(
        actual
            .diagnostics()
            .contains(&FlowPointerBodyDiagnostic::OrientationPostprocessingNotReproduced)
    );
    assert_eq!(actual.record().ordinal(), 1);
    assert_eq!(actual.source().records().len(), 3);
}

fn expected_totals(case: &PointerCase) -> (usize, usize, usize, usize, usize) {
    let mut result = (
        case.pools.len(),
        0,
        0,
        case.pools.iter().map(|pool| pool.bits.len()).sum(),
        0,
    );
    for entry in case.groups.iter().flat_map(|group| &group.entries) {
        result.1 += 1;
        let counts = body_counts(&entry.body);
        result.2 += counts.0;
        result.3 += counts.1;
        result.4 += counts.2;
    }
    result
}

fn body_counts(body: &PointerEntryBody) -> (usize, usize, usize) {
    match body {
        PointerEntryBody::Single(bytes) => (1, 0, bytes.len()),
        PointerEntryBody::Wide { bits, .. } => (0, bits.len(), 0),
        PointerEntryBody::Multiple(vectors) => {
            (vectors.len(), 0, vectors.iter().map(Vec::len).sum())
        }
        PointerEntryBody::Tagged(tagged) => (
            0,
            tagged.bits.as_ref().map_or(0, Vec::len),
            tagged.raw.as_ref().map_or(0, Vec::len),
        ),
    }
}
