use super::*;
use crate::conditional_roles::{append_scope, field_suffix};

pub(super) fn scope_path(owner: &FlowLegacyGroup155, scope: FlowConditionalScope<'_>) -> String {
    format!("[{}]", scope_parts(owner, scope).join(","))
}
fn scope_parts(owner: &FlowLegacyGroup155, scope: FlowConditionalScope<'_>) -> Vec<String> {
    let Some(parent) = scope.parent() else {
        assert_eq!(scope.kind(), FlowConditionalScopeKind::Record);
        return Vec::new();
    };
    let parent = owner.scope(&parent).unwrap();
    let mut parts = scope_parts(owner, parent);
    append_scope(
        &mut parts,
        true,
        scope.kind(),
        parent.kind(),
        scope.ordinal_in_parent(),
    );
    parts
}
pub(super) fn field_path(owner: &FlowLegacyGroup155, field: FlowConditionalField<'_>) -> String {
    let mut parts = scope_parts(owner, owner.scope(&field.scope()).unwrap());
    parts.extend(field_suffix(false, field.kind()));
    format!("[{}]", parts.join(","))
}
