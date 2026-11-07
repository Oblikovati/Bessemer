use super::*;
#[derive(Clone, Copy)]
enum Cell {
    FirstKey(usize),
    SecondKey(usize),
    State(usize),
    Reference(usize, usize),
    Outer(usize),
    Attribute(usize),
}
#[test]
fn every_reference_key_state_and_attribute_has_noop_checked_patch_history_and_fresh_reverse() {
    for case in profile_fixture::cases() {
        let profile = open(&case.raw, options()).unwrap();
        for object in profile.first_objects() {
            check_edit(&profile, Cell::FirstKey(object.ordinal()));
        }
        for object in profile.reference_objects() {
            check_edit(&profile, Cell::SecondKey(object.ordinal()));
            check_edit(&profile, Cell::State(object.ordinal()));
            for index in 0..object.references().len() {
                check_edit(&profile, Cell::Reference(object.ordinal(), index));
            }
        }
        for index in 0..profile.outer_references().len() {
            check_edit(&profile, Cell::Outer(index));
            check_edit(&profile, Cell::Attribute(index));
        }
    }
}
impl Cell {
    fn original(self, profile: &FlowMediumKindOneProfile) -> (usize, Vec<u8>) {
        let (start, len) = match self {
            Self::FirstKey(n) => (
                profile.first_object(n).unwrap().payload_interval().0 + 4,
                16,
            ),
            Self::SecondKey(n) => (reference_start(profile, n) + 6, 16),
            Self::State(n) => (reference_start(profile, n) + 22, 1),
            Self::Reference(n, i) => (
                profile.reference_object(n).unwrap().references_interval().0 + i * 16,
                16,
            ),
            Self::Outer(i) => (profile.outer_references_interval().0 + i * 16, 16),
            Self::Attribute(i) => (profile.outer_attributes_interval().0 + i * 4, 4),
        };
        (start, profile.payload_bytes()[start..start + len].to_vec())
    }
    fn apply(
        self,
        profile: &FlowMediumKindOneProfile,
        raw: &[u8],
    ) -> bessemer::Result<FlowMediumProfileEdit> {
        match self {
            Self::FirstKey(n) => profile.replace_first_key(n, guid(raw)),
            Self::SecondKey(n) => profile.replace_reference_key(n, guid(raw)),
            Self::State(n) => profile.replace_reference_state(n, raw[0]),
            Self::Reference(n, i) => profile.replace_reference(n, i, guid(raw)),
            Self::Outer(i) => profile.replace_outer_reference(i, guid(raw)),
            Self::Attribute(i) => {
                profile.replace_outer_attribute(i, i32::from_le_bytes(raw.try_into().unwrap()))
            }
        }
    }
    fn accepts(self, profile: &FlowMediumKindOneProfile, raw: &[u8]) -> bool {
        match self {
            Self::FirstKey(ordinal) => first_membership(profile, ordinal, guid(raw)),
            Self::SecondKey(ordinal) => second_membership(profile, ordinal, guid(raw)),
            _ => true,
        }
    }
    fn changed(self, profile: &FlowMediumKindOneProfile, old: &[u8]) -> Vec<u8> {
        match self {
            Self::Reference(..) => profile
                .first_objects()
                .last()
                .unwrap()
                .key()
                .as_bytes()
                .to_vec(),
            Self::Outer(_) => profile
                .reference_objects()
                .last()
                .unwrap()
                .key()
                .as_bytes()
                .to_vec(),
            _ => old.iter().map(|byte| byte ^ 255).collect(),
        }
    }
}
fn check_edit(profile: &FlowMediumKindOneProfile, cell: Cell) {
    let (offset, old) = cell.original(profile);
    check_noop(profile, cell, &old);
    let changed = cell.changed(profile, &old);
    let snapshot = profile.source().as_bytes().to_vec();
    let attempted = cell.apply(profile, &changed);
    assert_eq!(profile.source().as_bytes(), snapshot);
    let valid = cell.accepts(profile, &changed);
    assert_eq!(attempted.is_ok(), valid);
    if !valid {
        assert_eq!(
            attempted.unwrap_err().class,
            bessemer::FailureClass::Invalid
        );
        return;
    }
    check_patch(profile, cell, offset, &old, &changed, attempted.unwrap());
}
fn check_history(edit: &FlowMediumProfileEdit) {
    let history = edit.prefix_edit().carrier_edit();
    assert_eq!(history.history().len(), 3);
    let changed = history.source().as_bytes() != history.result().as_bytes();
    for ((before, after), entry) in history
        .source()
        .records()
        .zip(history.result().records())
        .zip(history.history())
    {
        assert_eq!(entry.source, before.id);
        assert_eq!(entry.result, after.id);
        let expected = relation(before.id.ordinal(), changed);
        assert_eq!(entry.relation, expected);
        if before.id.ordinal() != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}
fn relation(ordinal: usize, changed: bool) -> FlowPayloadRelation {
    if ordinal == 1 && changed {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    }
}
#[test]
fn every_physical_body_accepts_nested_edits_at_its_own_offset_with_complete_record_revalidation() {
    for case in profile_fixture::cases() {
        let profile = open(&case.raw, options()).unwrap();
        for object in profile.first_objects() {
            check_body_edit(&profile, object);
        }
    }
}
fn check_body_edit(profile: &FlowMediumKindOneProfile, object: FlowMediumProfileObject<'_>) {
    let body = object.body();
    let old = body.state_byte();
    let ordinal = object.ordinal();
    let local = body.replace_state_byte(old ^ 255).unwrap();
    let edit = profile.apply_body_edit(ordinal, &local).unwrap();
    let mut expected = profile.payload_bytes().to_vec();
    expected[body.payload_interval().1 - 1] = old ^ 255;
    assert_eq!(edit.result().payload_bytes(), expected);
    check_history(&edit);
    let after = edit.result().first_object(ordinal).unwrap();
    let local = after.body().replace_state_byte(old).unwrap();
    let reverse = edit.result().apply_body_edit(ordinal, &local).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        profile.source().as_bytes()
    );
    check_body_noop(profile, object);
}

fn first_membership(profile: &FlowMediumKindOneProfile, ordinal: usize, key: FlowGuid) -> bool {
    let keys: Vec<_> = profile
        .first_objects()
        .map(|object| {
            if object.ordinal() == ordinal {
                key
            } else {
                object.key()
            }
        })
        .collect();
    profile.reference_objects().all(|object| {
        object
            .references()
            .iter()
            .all(|reference| keys.contains(reference))
    })
}
fn second_membership(profile: &FlowMediumKindOneProfile, ordinal: usize, key: FlowGuid) -> bool {
    let keys: Vec<_> = profile
        .reference_objects()
        .map(|object| {
            if object.ordinal() == ordinal {
                key
            } else {
                object.key()
            }
        })
        .collect();
    profile
        .outer_references()
        .iter()
        .all(|reference| keys.contains(reference))
}

fn check_noop(profile: &FlowMediumKindOneProfile, cell: Cell, old: &[u8]) {
    let noop = cell.apply(profile, old).unwrap();
    assert_eq!(noop.result().record(), profile.record());
    assert_eq!(
        noop.result().source().as_bytes(),
        profile.source().as_bytes()
    );
    check_history(&noop);
}
fn reference_start(profile: &FlowMediumKindOneProfile, ordinal: usize) -> usize {
    profile
        .reference_object(ordinal)
        .unwrap()
        .payload_interval()
        .0
}

fn check_patch(
    profile: &FlowMediumKindOneProfile,
    cell: Cell,
    offset: usize,
    old: &[u8],
    changed: &[u8],
    edit: FlowMediumProfileEdit,
) {
    let mut expected = profile.payload_bytes().to_vec();
    expected[offset..offset + changed.len()].copy_from_slice(changed);
    assert_eq!(edit.result().payload_bytes(), expected);
    assert_eq!(
        edit.source().source().as_bytes(),
        profile.source().as_bytes()
    );
    check_history(&edit);
    let reverse = cell.apply(edit.result(), old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        profile.source().as_bytes()
    );
}

fn check_body_noop(profile: &FlowMediumKindOneProfile, object: FlowMediumProfileObject<'_>) {
    let body = object.body();
    let ordinal = object.ordinal();
    let noop = body.replace_base_byte(body.base_byte()).unwrap();
    assert_eq!(
        profile
            .apply_body_edit(ordinal, &noop)
            .unwrap()
            .result()
            .record(),
        profile.record()
    );
}

#[test]
fn complete_record_validation_composes_metadata_edits_and_preserves_the_entire_backend() {
    for case in profile_fixture::cases() {
        let profile = open(&case.raw, options()).unwrap();
        let noop = profile
            .metadata_prefix()
            .replace_version_byte(profile.metadata_prefix().version_byte())
            .unwrap();
        assert_eq!(
            profile
                .apply_metadata_edit(&noop)
                .unwrap()
                .result()
                .record(),
            profile.record()
        );
        for object in profile.metadata_prefix().objects() {
            check_metadata_edit(&profile, object.ordinal(), object.state_byte());
        }
    }
}
fn check_metadata_edit(profile: &FlowMediumKindOneProfile, ordinal: usize, old: u8) {
    let local = profile
        .metadata_prefix()
        .replace_object_state(ordinal, old ^ 255)
        .unwrap();
    let edit = profile.apply_metadata_edit(&local).unwrap();
    assert_eq!(
        edit.result().payload_bytes(),
        local.result().payload_bytes()
    );
    assert_eq!(
        edit.result().metadata_prefix().opaque_tail(),
        profile.metadata_prefix().opaque_tail()
    );
    check_history(&edit);
    reverse_metadata(profile, &edit, ordinal, old);
}

fn reverse_metadata(
    profile: &FlowMediumKindOneProfile,
    edit: &FlowMediumProfileEdit,
    ordinal: usize,
    old: u8,
) {
    let local = edit
        .result()
        .metadata_prefix()
        .replace_object_state(ordinal, old)
        .unwrap();
    let reverse = edit.result().apply_metadata_edit(&local).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        profile.source().as_bytes()
    );
}
