use super::*;
#[derive(Clone, Copy)]
enum Cell {
    Version,
    Vector(usize, usize),
    State(usize),
    Word(usize),
    Unit(usize, FlowMetadataStringRole, usize),
}
#[test]
fn all_small_existing_prefix_fields_and_large_endpoints_have_exact_noops_patches_history_and_fresh_reverses()
 {
    for case in fixture::prefix_cases() {
        let view = open(&case.raw, options()).unwrap();
        check_edit(&view, Cell::Version);
        for vector in view.vectors() {
            for index in selected(vector.values().len()) {
                check_edit(&view, Cell::Vector(vector.ordinal(), index));
            }
        }
        for object in view.objects() {
            check_object_edits(&view, object);
        }
    }
}
fn selected(count: usize) -> Vec<usize> {
    if count <= 256 {
        return (0..count).collect();
    }
    vec![0, count - 1]
}
fn check_object_edits(view: &FlowMetadataPrefix, object: FlowMetadataObject<'_>) {
    check_edit(view, Cell::State(object.ordinal()));
    check_edit(view, Cell::Word(object.ordinal()));
    for role in roles() {
        if let Some(text) = object.string(role) {
            for index in selected(text.stored_units()) {
                check_edit(view, Cell::Unit(object.ordinal(), role, index));
            }
        }
    }
}
impl Cell {
    fn original(self, view: &FlowMetadataPrefix) -> (usize, Vec<u8>) {
        let offset = self.offset(view);
        let length = match self {
            Self::Version | Self::State(_) => 1,
            Self::Unit(..) => 2,
            _ => 4,
        };
        (
            offset,
            view.payload_bytes()[offset..offset + length].to_vec(),
        )
    }
    fn offset(self, view: &FlowMetadataPrefix) -> usize {
        match self {
            Self::Version => 0,
            Self::Vector(vector, index) => {
                view.vector(vector).unwrap().payload_interval().0 + 4 + index * 4
            }
            Self::State(object) => state_offset(view.object(object).unwrap()),
            Self::Word(object) => state_offset(view.object(object).unwrap()) + 1,
            Self::Unit(object, role, index) => string_offset(view, object, role) + index * 2,
        }
    }
    fn apply(
        self,
        view: &FlowMetadataPrefix,
        bytes: &[u8],
    ) -> bessemer::Result<FlowMetadataPrefixEdit> {
        match self {
            Self::Version => view.replace_version_byte(bytes[0]),
            Self::Vector(vector, index) => view.replace_vector_value(
                vector,
                index,
                i32::from_le_bytes(bytes.try_into().unwrap()),
            ),
            Self::State(object) => view.replace_object_state(object, bytes[0]),
            Self::Word(object) => {
                view.replace_member_c4(object, i32::from_le_bytes(bytes.try_into().unwrap()))
            }
            Self::Unit(object, role, index) => view.replace_string_unit(
                object,
                role,
                index,
                u16::from_le_bytes(bytes.try_into().unwrap()),
            ),
        }
    }
}
fn state_offset(object: FlowMetadataObject<'_>) -> usize {
    object
        .string(FlowMetadataStringRole::Second)
        .unwrap()
        .payload_interval()
        .1
}
fn check_edit(view: &FlowMetadataPrefix, cell: Cell) {
    let (offset, old) = cell.original(view);
    let noop = cell.apply(view, &old).unwrap();
    assert_eq!(noop.result().record(), view.record());
    assert_eq!(noop.result().source().as_bytes(), view.source().as_bytes());
    check_history(&noop);
    let changed = changed_bytes(view, cell, &old);
    let edit = cell.apply(view, &changed).unwrap();
    let mut expected = view.payload_bytes().to_vec();
    expected[offset..offset + changed.len()].copy_from_slice(&changed);
    assert_eq!(edit.result().payload_bytes(), expected);
    assert_eq!(edit.result().opaque_tail(), view.opaque_tail());
    assert_eq!(edit.source().source().as_bytes(), view.source().as_bytes());
    check_history(&edit);
    let reverse = cell.apply(edit.result(), &old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        view.source().as_bytes()
    );
}
fn changed_bytes(view: &FlowMetadataPrefix, cell: Cell, old: &[u8]) -> Vec<u8> {
    if matches!(cell, Cell::Version) {
        return old.to_vec();
    }
    if let Cell::Unit(object, role, index) = cell {
        let text = view.object(object).unwrap().string(role).unwrap();
        if index + 1 == text.stored_units() {
            return old.to_vec();
        }
    }
    let mut result = old.to_vec();
    result[0] ^= 255;
    result
}
fn check_history(edit: &FlowMetadataPrefixEdit) {
    let carrier = edit.carrier_edit();
    assert_eq!(carrier.history().len(), 3);
    let changed = carrier.source().as_bytes() != carrier.result().as_bytes();
    for ((before, after), relation) in carrier
        .source()
        .records()
        .zip(carrier.result().records())
        .zip(carrier.history())
    {
        assert_eq!(relation.source, before.id);
        assert_eq!(relation.result, after.id);
        let kind = if before.id.ordinal() == 1 && changed {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        };
        assert_eq!(relation.relation, kind);
    }
}

fn string_offset(view: &FlowMetadataPrefix, object: usize, role: FlowMetadataStringRole) -> usize {
    view.object(object)
        .unwrap()
        .string(role)
        .unwrap()
        .payload_interval()
        .0
        + 4
}
