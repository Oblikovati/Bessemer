use crate::archive_codec::ObjectRecord;
use crate::model_archives::{ModelField, ModelFieldValue};
use crate::{BessemerError, FailureClass, Result};

pub(super) struct IntersectionReferenceScope<'a> {
    pub group: &'a str,
    intersection: &'a str,
    sketch: &'a str,
    edge: usize,
}

pub(super) fn intersection_candidate(path: &str) -> bool {
    path.contains(".styled_intersections.intersections[")
        && path.contains(".edges_48[")
        && [
            ".group_38.raw_00",
            ".group_80.raw_00",
            ".group_c8.raw_00",
            ".group_110.raw_00",
        ]
        .iter()
        .any(|suffix| path.ends_with(suffix))
}

pub(super) fn reference_scope(path: &str) -> Result<IntersectionReferenceScope<'_>> {
    let group = path
        .strip_suffix(".raw_00")
        .ok_or_else(|| reference_error(path, "not leading raw_00"))?;
    let (edge_path, position) = group
        .rsplit_once('.')
        .ok_or_else(|| reference_error(path, "missing group scope"))?;
    if !["group_38", "group_80", "group_c8", "group_110"].contains(&position) {
        return Err(reference_error(path, position));
    }
    let (intersection, edge) = indexed_parent(edge_path, ".edges_48[")?;
    let (collection, _) = indexed_parent(intersection, ".intersections[")?;
    let sketch = sketch_parent(collection)?;
    Ok(IntersectionReferenceScope {
        group,
        intersection,
        sketch,
        edge,
    })
}

fn sketch_parent(collection: &str) -> Result<&str> {
    collection
        .strip_suffix(".styled_intersections")
        .filter(|owner| !owner.is_empty())
        .ok_or_else(|| reference_error(collection, "missing sketch owner"))
}

pub(super) fn edge_index_scope(path: &str) -> Result<IntersectionReferenceScope<'_>> {
    let group = path
        .strip_suffix(".raw_168")
        .ok_or_else(|| reference_error(path, "not raw_168"))?;
    let (intersection, edge) = indexed_parent(group, ".edges_48[")?;
    let (collection, _) = indexed_parent(intersection, ".intersections[")?;
    Ok(IntersectionReferenceScope {
        group,
        intersection,
        sketch: sketch_parent(collection)?,
        edge,
    })
}

fn indexed_parent<'a>(path: &'a str, label: &str) -> Result<(&'a str, usize)> {
    let (parent, suffix) = path
        .rsplit_once(label)
        .ok_or_else(|| reference_error(path, label))?;
    let raw = suffix
        .strip_suffix(']')
        .ok_or_else(|| reference_error(path, suffix))?;
    let index = raw
        .parse::<usize>()
        .map_err(|_| reference_error(path, raw))?;
    if parent.is_empty() || raw != index.to_string() {
        return Err(reference_error(path, raw));
    }
    Ok((parent, index))
}

pub(super) fn require_reference_owner(
    fields: &[ModelField],
    scope: &IntersectionReferenceScope<'_>,
) -> Result<()> {
    require_sketch_definition(fields, scope)?;
    let marker = preceding_field(fields, scope.intersection, ".marker")?;
    if marker.value != ModelFieldValue::U16(0) {
        return Err(reference_error(scope.group, &marker.value));
    }
    let count = preceding_field(fields, scope.intersection, ".handle_count")?;
    match count.value {
        ModelFieldValue::I32(value) if value > 0 && scope.edge < value as usize => Ok(()),
        ref other => Err(reference_error(scope.group, other)),
    }
}

fn require_sketch_definition(
    fields: &[ModelField],
    scope: &IntersectionReferenceScope<'_>,
) -> Result<()> {
    let owner = fields
        .iter()
        .rev()
        .find(|field| field.path.as_ref() == scope.sketch);
    let Some(ModelField {
        value: ModelFieldValue::ObjectTag(tag),
        ..
    }) = owner
    else {
        return Err(reference_error(
            scope.group,
            "missing preceding sgSketch definition",
        ));
    };
    match &tag.record {
        ObjectRecord::New { class, .. } if class.name == "sgSketch" => Ok(()),
        other => Err(reference_error(scope.group, other)),
    }
}

fn preceding_field<'a>(
    fields: &'a [ModelField],
    prefix: &str,
    suffix: &str,
) -> Result<&'a ModelField> {
    fields
        .iter()
        .rev()
        .find(|field| field.path.strip_prefix(prefix) == Some(suffix))
        .ok_or_else(|| reference_error(prefix, suffix))
}

pub(super) fn reference_error(path: &str, actual: impl std::fmt::Debug) -> BessemerError {
    BessemerError::new(
        "model intersection reference stored state",
        path,
        FailureClass::Invalid,
        format!(
            "{actual:?} at {path:?}; expected a physical sgSketch marker0 intersection edge group38/80/c8/110 with adjacent same-scope U16 raw_00 and I32 raw_20/raw_24/raw_28, canonical indices and an in-range edge"
        ),
    )
}
