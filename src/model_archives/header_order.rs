use crate::Result;
use crate::archive_codec::ArchiveClass;
use crate::header::{HeaderContents, HeaderObject, HeaderObjectId, HeaderRecord, HeaderReference};

pub(super) fn ordered_imports(
    contents: &HeaderContents,
    maximum: usize,
) -> Result<Vec<HeaderReference>> {
    let root = root_record(contents)?;
    let self_reference = root
        .self_reference
        .filter(|reference| *reference != HeaderReference::Null);
    let external = external_items(contents, root.external_references)?;
    let count = checked_import_count(external.len(), self_reference.is_some(), maximum)?;
    let mut references = Vec::with_capacity(count);
    references.extend(self_reference);
    references.extend_from_slice(external);
    Ok(references)
}

fn root_record(contents: &HeaderContents) -> Result<&HeaderRecord> {
    let node = header_node(contents, contents.root)?;
    if let HeaderObject::Header(root) = &node.object {
        return Ok(root);
    }
    Err(super::invalid(format!(
        "header root {} has class {:?}; expected moHeader_c",
        contents.root.value(),
        node.archive_class().name
    )))
}

fn external_items(
    contents: &HeaderContents,
    reference: HeaderReference,
) -> Result<&[HeaderReference]> {
    let HeaderReference::Object(identity) = reference else {
        return Ok(&[]);
    };
    let node = header_node(contents, identity)?;
    if let HeaderObject::ExternalList(list) = &node.object {
        return Ok(&list.items);
    }
    Err(super::invalid(format!(
        "header external list {} has class {:?}; expected moExtObjectList_c",
        identity.value(),
        node.archive_class().name
    )))
}

pub(super) fn imported_class(
    contents: &HeaderContents,
    reference: HeaderReference,
) -> Result<Option<ArchiveClass>> {
    let HeaderReference::Object(identity) = reference else {
        return Ok(None);
    };
    Ok(Some(header_node(contents, identity)?.archive_class()))
}

fn header_node(
    contents: &HeaderContents,
    identity: HeaderObjectId,
) -> Result<&crate::header::HeaderNode> {
    contents.objects.get(&identity).ok_or_else(|| {
        super::invalid(format!(
            "header object {}; expected an identity in the retained header graph",
            identity.value()
        ))
    })
}

pub(super) fn checked_import_count(
    external: usize,
    has_self: bool,
    maximum: usize,
) -> Result<usize> {
    let count = external.checked_add(usize::from(has_self)).ok_or_else(|| {
        super::invalid(format!(
            "header imports {external} plus self reference; expected within usize"
        ))
    })?;
    if count > maximum {
        return Err(super::invalid(format!(
            "header imports {count}; expected <= {maximum}"
        )));
    }
    Ok(count)
}
