use super::*;
use FlowConditionalFieldKind as Kind;
use FlowConditionalScopeKind as Scope;

pub(super) fn scope_path(owner: &FlowConditionalItems, scope: FlowConditionalScope<'_>) -> String {
    format!("[{}]", scope_parts(owner, scope).join(","))
}

fn scope_parts(owner: &FlowConditionalItems, scope: FlowConditionalScope<'_>) -> Vec<String> {
    let Some(parent) = scope.parent() else {
        assert_eq!(scope.kind(), Scope::Record);
        return Vec::new();
    };
    let parent = owner.scope(&parent).unwrap();
    let mut parts = scope_parts(owner, parent);
    append_scope(
        &mut parts,
        owner.format(),
        scope.kind(),
        parent.kind(),
        scope.ordinal_in_parent(),
    );
    parts
}

pub(super) fn field_path(owner: &FlowConditionalItems, field: FlowConditionalField<'_>) -> String {
    let mut parts = scope_parts(owner, owner.scope(&field.scope()).unwrap());
    parts.extend(field_suffix(owner.format(), field.kind()));
    format!("[{}]", parts.join(","))
}

fn quoted(value: &str) -> String {
    format!("\"{value}\"")
}

fn field_suffix(format: FlowConditionalFormat, kind: Kind) -> Vec<String> {
    match kind {
        Kind::LegacyArrayCount(index) => vector("legacy_arrays", Some(index), "count"),
        Kind::LegacyArrayValues(index) if format == FlowConditionalFormat::Record126 => {
            vec![quoted("arrays"), index.to_string()]
        }
        Kind::LegacyArrayValues(index) => vector("legacy_arrays", Some(index), "values"),
        Kind::LegacyVectorCount(index) => vector("legacy_vectors", Some(index), "count"),
        Kind::LegacyVectorValues(index) => vector("legacy_vectors", Some(index), "values"),
        Kind::CommonCount => vector("common_vector", None, "count"),
        Kind::CommonValues => vector("common_vector", None, "values"),
        Kind::WideCount => vector("wide", None, "count"),
        Kind::WideValues => vector("wide", None, "values"),
        Kind::BytesCount => vector("bytes", None, "count"),
        Kind::Bytes => vector("bytes", None, "values"),
        Kind::FirstCount => vector("first", None, "count"),
        Kind::FirstValues => vector("first", None, "values"),
        Kind::SubExtraCount => vector("extra", None, "count"),
        Kind::SubExtraValues => vector("extra", None, "values"),
        _ => vec![quoted(scalar_role(kind))],
    }
}

fn vector(name: &str, index: Option<u8>, role: &str) -> Vec<String> {
    let mut parts = vec![quoted(name)];
    if let Some(index) = index {
        parts.push(index.to_string());
    }
    parts.push(quoted(role));
    parts
}

fn scalar_role(kind: Kind) -> &'static str {
    match kind {
        Kind::Version => "version",
        Kind::Count => "count",
        Kind::InnerWord => "inner_word_i32",
        Kind::MemberA0Signed => "member_a0_i32",
        Kind::Mode => "mode_i32",
        Kind::Dimension => "dimension",
        Kind::GateSigned => "gate_i32",
        Kind::ExtraCount => "extra_count",
        Kind::ExtraMode => "extra_mode_i32",
        Kind::ExtraValues => "extra_patterns",
        Kind::MemberA0Pattern => "member_a0_pattern",
        Kind::Protocol => "protocol_i32",
        Kind::GatePattern => "gate_pattern",
        _ => item_scalar_role(kind),
    }
}

fn append_scope(
    parts: &mut Vec<String>,
    format: FlowConditionalFormat,
    kind: Scope,
    parent_kind: Scope,
    index: usize,
) {
    match kind {
        Scope::HeaderGroup if format == FlowConditionalFormat::Record128 => {
            parts.push(quoted("group"))
        }
        Scope::HeaderGroup | Scope::ItemGroup => {
            parts.extend([quoted("groups"), index.to_string()])
        }
        Scope::Entry => parts.extend([quoted("items"), index.to_string()]),
        Scope::Item => parts.push(quoted("item")),
        Scope::Subrecord if parent_kind == Scope::ItemGroup => parts.push(quoted("subrecord")),
        Scope::Subrecord => parts.extend([quoted("subrecords"), index.to_string()]),
        other => panic!("Unexpected nested scope {other:?}"),
    }
}

fn item_scalar_role(kind: Kind) -> &'static str {
    match kind {
        Kind::Presence => "present_byte",
        Kind::Active => "active_byte",
        Kind::LegacyWord => "legacy_word_i32",
        Kind::Member84 => "member_84_i32",
        Kind::ItemMode => "mode_byte",
        Kind::GroupCount => "group_count",
        Kind::SubKind => "kind_i32",
        Kind::MemberC => "member_c_i32",
        Kind::Member10 => "member_10_i32",
        other => panic!("Expected scalar role, got {other:?}"),
    }
}
