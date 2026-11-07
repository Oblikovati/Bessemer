use std::collections::BTreeMap;

use super::context::VersionContext;
use super::fields::{FieldRole, FieldView, Phase};
use super::kinds::ClassKind;
use super::{
    ArchiveVersion, HeaderContents, HeaderObject, HeaderObjectId, HeaderReference, SharedFileName,
    invalid,
};
use crate::Result;

pub(super) type VersionPlan = BTreeMap<HeaderObjectId, VersionContext>;
struct Visit {
    reference: HeaderReference,
    version: ArchiveVersion,
    role: FieldRole,
}

pub(super) fn contents(contents: &HeaderContents, version: ArchiveVersion) -> Result<VersionPlan> {
    let mut plan = BTreeMap::new();
    let mut tasks = vec![Visit {
        reference: HeaderReference::Object(contents.root),
        version,
        role: FieldRole::Any,
    }];
    require_root(contents)?;
    while let Some(visit) = tasks.pop() {
        visit_object(contents, visit, &mut plan, &mut tasks)?;
    }
    if plan.len() != contents.objects.len() {
        return Err(invalid(format!(
            "{} reachable of {} stored objects; expected no orphan objects",
            plan.len(),
            contents.objects.len()
        )));
    }
    Ok(plan)
}

fn require_root(contents: &HeaderContents) -> Result<()> {
    let kind = contents
        .objects
        .get(&contents.root)
        .map(|node| node.object.kind());
    if kind == Some(ClassKind::Header) {
        return Ok(());
    }
    Err(invalid(format!(
        "root {} has class {kind:?}; expected moHeader_c",
        contents.root.value()
    )))
}

fn visit_object(
    contents: &HeaderContents,
    visit: Visit,
    plan: &mut VersionPlan,
    tasks: &mut Vec<Visit>,
) -> Result<()> {
    let HeaderReference::Object(id) = visit.reference else {
        return Ok(());
    };
    let node = contents.objects.get(&id).ok_or_else(|| {
        invalid(format!(
            "object {}; expected an identity in the header graph",
            id.value()
        ))
    })?;
    validate_role(&node.object, visit.role, contents)?;
    if plan.contains_key(&id) {
        return Ok(());
    }
    let context = context_for(&node.object, visit.version, contents)?;
    plan.insert(id, context);
    let mut children = validate_fields(&node.object, context)?;
    children.reverse();
    tasks.extend(children);
    Ok(())
}

fn context_for(
    object: &HeaderObject,
    version: ArchiveVersion,
    contents: &HeaderContents,
) -> Result<VersionContext> {
    super::context::require_supported(object.kind(), version)?;
    let context = VersionContext::new(version);
    match object {
        HeaderObject::Header(record) => context.with_users(record.users, &contents.objects),
        _ => Ok(context),
    }
}

fn role_matches(kind: ClassKind, role: FieldRole) -> bool {
    match role {
        FieldRole::Any => true,
        FieldRole::Strings => kind == ClassKind::Strings,
        FieldRole::ObjectList | FieldRole::RevisionLogs => {
            matches!(kind, ClassKind::List | ClassKind::ExternalList)
        }
        FieldRole::Log => kind == ClassKind::Log,
        FieldRole::Stamp => kind == ClassKind::Stamp,
        FieldRole::ExternalList => kind == ClassKind::ExternalList,
        FieldRole::External => kind == ClassKind::External,
        FieldRole::Handle => kind == ClassKind::Handle,
    }
}

fn validate_role(object: &HeaderObject, role: FieldRole, contents: &HeaderContents) -> Result<()> {
    if !role_matches(object.kind(), role) {
        return Err(invalid(format!(
            "class {:?}; expected field role {role:?}",
            object.kind()
        )));
    }
    let element_role = match role {
        FieldRole::RevisionLogs => FieldRole::Log,
        FieldRole::ExternalList => FieldRole::External,
        _ => return Ok(()),
    };
    let items = match object {
        HeaderObject::List(list) | HeaderObject::ExternalList(list) => &list.items,
        _ => return Ok(()),
    };
    for reference in items {
        validate_element(*reference, element_role, contents)?;
    }
    Ok(())
}

fn validate_element(
    reference: HeaderReference,
    role: FieldRole,
    contents: &HeaderContents,
) -> Result<()> {
    let HeaderReference::Object(id) = reference else {
        return Ok(());
    };
    let kind = contents.objects.get(&id).map(|node| node.object.kind());
    if kind.is_some_and(|kind| role_matches(kind, role)) {
        return Ok(());
    }
    Err(invalid(format!(
        "list element {} class {kind:?}; expected {role:?}",
        id.value()
    )))
}

fn validate_fields(object: &HeaderObject, context: VersionContext) -> Result<Vec<Visit>> {
    let mut children = Vec::new();
    for (rule, value) in object.kind().rules().iter().zip(object.views()) {
        let enabled = context.enabled(rule);
        require_presence(rule, value.is_some(), enabled, context)?;
        let Some(value) = value else {
            continue;
        };
        let version = match rule.phase {
            Phase::Outer => context.outer,
            Phase::Body => context.body,
        };
        validate_value(value, version)?;
        append_children(value, version, rule.role, &mut children);
    }
    Ok(children)
}

fn require_presence(
    rule: &super::fields::FieldRule,
    present: bool,
    enabled: bool,
    context: VersionContext,
) -> Result<()> {
    if present == enabled {
        return Ok(());
    }
    Err(invalid(format!(
        "field {} presence {present}; expected {enabled} at outer {} / body {} (optional {})",
        rule.name,
        context.outer.value(),
        context.body.value(),
        rule.optional
    )))
}

fn validate_value(value: FieldView<'_>, version: ArchiveVersion) -> Result<()> {
    match value {
        FieldView::Bounds(bounds) => super::values::require_bounds(bounds),
        FieldView::SharedName(name)
            if matches!(name, SharedFileName::Handle(_)) != (version.value() >= 0xfa7) =>
        {
            Err(invalid(format!(
                "shared file name {name:?}; expected handle iff version {} >= 0xFA7",
                version.value()
            )))
        }
        FieldView::References(items) => validate_count(items.len()),
        FieldView::Texts(items) => validate_count(items.len()),
        _ => Ok(()),
    }
}

fn validate_count(count: usize) -> Result<()> {
    if u32::try_from(count).is_ok() {
        return Ok(());
    }
    Err(invalid(format!(
        "collection length {count}; expected at most {}",
        u32::MAX
    )))
}

fn append_children(
    value: FieldView<'_>,
    version: ArchiveVersion,
    role: FieldRole,
    children: &mut Vec<Visit>,
) {
    match value {
        FieldView::Reference(reference) => children.push(Visit {
            reference,
            version,
            role,
        }),
        FieldView::SharedName(SharedFileName::Handle(reference)) => children.push(Visit {
            reference: *reference,
            version,
            role,
        }),
        FieldView::References(items) => children.extend(items.iter().map(|reference| Visit {
            reference: *reference,
            version,
            role,
        })),
        _ => {}
    }
}
