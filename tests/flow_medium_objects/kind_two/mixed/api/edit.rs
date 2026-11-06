use super::*;
#[derive(Clone, Copy)]
enum Cell {
    FirstKey(usize),
    ReferenceKey(usize),
    State(usize),
    Inner(usize, usize),
    Outer(usize),
    Attribute(usize),
    BodyBase(usize),
    BodyState(usize),
}
fn fields(case: &MixedCase) -> Vec<Cell> {
    let mut result = Vec::new();
    for index in 0..case.first.len() {
        result.extend([
            Cell::FirstKey(index),
            Cell::BodyBase(index),
            Cell::BodyState(index),
        ]);
    }
    reference_fields(case, &mut result);
    for index in 0..case.footer.keys.len() {
        result.extend([Cell::Outer(index), Cell::Attribute(index)]);
    }
    result
}
impl Cell {
    fn original(self, case: &MixedCase) -> (usize, Vec<u8>) {
        let (offset, length) = match self {
            Self::FirstKey(i) => (case.first[i].start as usize + 4, 16),
            Self::ReferenceKey(i) => (case.second[i].start as usize + 6, 16),
            Self::State(i) => (case.second[i].start as usize + 22, 1),
            Self::Inner(i, j) => (
                case.second[i].start as usize
                    + if case.second[i].kind == 1 { 27 } else { 26 }
                    + j * 16,
                16,
            ),
            Self::Outer(i) => (case.backend_end as usize + 4 + i * 16, 16),
            Self::Attribute(i) => (
                case.backend_end as usize + 4 + case.footer.keys.len() * 16 + i * 4,
                4,
            ),
            Self::BodyBase(i) => (case.first[i].start as usize + 21, 1),
            Self::BodyState(i) => (case.first[i].end as usize - 1, 1),
        };
        (offset, case.raw[offset..offset + length].to_vec())
    }
    fn replacement(self, case: &MixedCase, original: &[u8]) -> Vec<u8> {
        match self {
            Self::FirstKey(_) | Self::ReferenceKey(_) => vec![0x42; 16],
            Self::Inner(i, _) => case
                .first
                .iter()
                .find(|entry| entry.kind == case.second[i].kind && entry.key != original)
                .map(|entry| entry.key.clone())
                .unwrap_or(original.to_vec()),
            Self::Outer(_) => case
                .second
                .iter()
                .find(|entry| entry.key != original)
                .map(|entry| entry.key.clone())
                .unwrap_or(original.to_vec()),
            _ => original.iter().map(|byte| byte ^ 0x80).collect(),
        }
    }
    fn valid(self, case: &MixedCase, original: &[u8]) -> bool {
        match self {
            Self::FirstKey(index) => {
                !case.second.iter().any(|entry| {
                    entry.kind == case.first[index].kind
                        && entry.references.iter().any(|key| key == original)
                }) || case.first.iter().enumerate().any(|(i, entry)| {
                    i != index && entry.kind == case.first[index].kind && entry.key == original
                })
            }
            Self::ReferenceKey(index) => {
                !case.footer.keys.iter().any(|key| key == original)
                    || case
                        .second
                        .iter()
                        .enumerate()
                        .any(|(i, entry)| i != index && entry.key == original)
            }
            _ => true,
        }
    }
    fn apply(
        self,
        profile: &FlowMediumMixedProfile,
        raw: &[u8],
    ) -> bessemer::Result<FlowMediumMixedEdit> {
        match self {
            Self::FirstKey(i) => profile.replace_first_key(i, guid(raw)),
            Self::ReferenceKey(i) => profile.replace_reference_key(i, guid(raw)),
            Self::State(i) => profile.replace_reference_state(i, raw[0]),
            Self::Inner(i, j) => profile.replace_reference(i, j, guid(raw)),
            Self::Outer(i) => profile.replace_outer_reference(i, guid(raw)),
            Self::Attribute(i) => {
                profile.replace_outer_attribute(i, i32::from_le_bytes(raw.try_into().unwrap()))
            }
            Self::BodyBase(i) => body_patch(profile, i, raw[0], false),
            Self::BodyState(i) => body_patch(profile, i, raw[0], true),
        }
    }
}
fn body_patch(
    profile: &FlowMediumMixedProfile,
    ordinal: usize,
    byte: u8,
    state: bool,
) -> bessemer::Result<FlowMediumMixedEdit> {
    match profile.first_objects()[ordinal].body() {
        FlowMediumBody::KindOne(body) => {
            let edit = if state {
                body.replace_state_byte(byte)?
            } else {
                body.replace_base_byte(byte)?
            };
            profile.apply_kind_one_body_edit(ordinal, &edit)
        }
        FlowMediumBody::KindTwo(body) => {
            let edit = if state {
                body.replace_state_byte(byte)?
            } else {
                body.replace_base_byte(byte)?
            };
            profile.apply_kind_two_body_edit(ordinal, &edit)
        }
    }
}
#[test]
fn every_mixed_key_reference_state_attribute_and_body_composition_has_exact_noop_patch_history_and_reverse()
 {
    for case in mixed_fixture::cases() {
        let profile = open(&case.raw, options()).unwrap();
        for cell in fields(&case) {
            check_cell(&profile, &case, cell);
        }
    }
}
fn check_cell(profile: &FlowMediumMixedProfile, case: &MixedCase, cell: Cell) {
    let (offset, original) = cell.original(case);
    check_noop(profile, cell, &original);
    let replacement = cell.replacement(case, &original);
    let result = cell.apply(profile, &replacement);
    if !cell.valid(case, &original) {
        assert!(result.is_err());
        assert_eq!(profile.payload_bytes(), case.raw);
        return;
    }
    check_changed(
        profile,
        case,
        cell,
        result.unwrap(),
        offset,
        &original,
        &replacement,
    );
}
fn check_changed(
    profile: &FlowMediumMixedProfile,
    case: &MixedCase,
    cell: Cell,
    edit: FlowMediumMixedEdit,
    offset: usize,
    original: &[u8],
    replacement: &[u8],
) {
    let mut expected = case.raw.clone();
    expected[offset..offset + replacement.len()].copy_from_slice(replacement);
    assert_eq!(
        edit.source().source().as_bytes(),
        profile.source().as_bytes()
    );
    assert_eq!(
        edit.result().source().as_bytes(),
        crate::api::carrier(&expected).as_bytes()
    );
    assert_eq!(profile.payload_bytes(), case.raw);
    check_history(&edit);
    let fresh = open(&expected, options()).unwrap();
    let reverse = cell.apply(&fresh, original).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        profile.source().as_bytes()
    );
}
fn relation(ordinal: usize, changed: bool) -> FlowPayloadRelation {
    if ordinal == 1 && changed {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    }
}
fn check_history(edit: &FlowMediumMixedEdit) {
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
#[test]
fn mixed_profiles_compose_metadata_only_edits_without_changing_backend_bytes() {
    for case in mixed_fixture::cases() {
        let profile = open(&case.raw, options()).unwrap();
        let version = if case.raw[0] == 2 {
            2
        } else if case.raw[0] == 127 {
            126
        } else {
            case.raw[0] + 1
        };
        let metadata = profile
            .metadata_prefix()
            .replace_version_byte(version)
            .unwrap();
        let edit = profile.apply_metadata_edit(&metadata).unwrap();
        let mut expected = case.raw.clone();
        expected[0] = version;
        assert_eq!(edit.result().payload_bytes(), expected);
        assert_eq!(profile.payload_bytes(), case.raw);
        check_history(&edit);
    }
}

fn reference_fields(case: &MixedCase, result: &mut Vec<Cell>) {
    for (ordinal, entry) in case.second.iter().enumerate() {
        result.push(Cell::ReferenceKey(ordinal));
        if entry.kind == 1 {
            result.push(Cell::State(ordinal));
        }
        for index in 0..entry.references.len() {
            result.push(Cell::Inner(ordinal, index));
        }
    }
}

fn check_noop(profile: &FlowMediumMixedProfile, cell: Cell, original: &[u8]) {
    let noop = cell.apply(profile, original).unwrap();
    assert_eq!(noop.result().record(), profile.record());
    assert_eq!(
        noop.result().source().as_bytes(),
        profile.source().as_bytes()
    );
    check_history(&noop);
}
