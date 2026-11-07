use std::collections::BTreeMap;

use super::fields::{FieldRule, Phase};
use super::{ArchiveVersion, HeaderNode, HeaderObject, HeaderObjectId, HeaderReference, invalid};
use crate::Result;

pub(crate) const VERSION_MARKER: &str =
    "SOLIDWORKS_INTERNAL_D95A9473_4FF5_42DF_A100_ECC502EF9F7D_HEADER_VERSION_";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VersionContext {
    pub outer: ArchiveVersion,
    pub body: ArchiveVersion,
}

impl VersionContext {
    pub fn new(version: ArchiveVersion) -> Self {
        Self {
            outer: version,
            body: version,
        }
    }

    pub fn enabled(self, rule: &FieldRule) -> bool {
        let version = match rule.phase {
            Phase::Outer => self.outer,
            Phase::Body => self.body,
        };
        (rule.enabled)(version.value())
    }

    pub fn with_users(
        self,
        users: HeaderReference,
        objects: &BTreeMap<HeaderObjectId, HeaderNode>,
    ) -> Result<Self> {
        let HeaderReference::Object(identity) = users else {
            return Ok(self);
        };
        let strings = user_strings(identity, objects)?;
        if strings.items.len() < 2 {
            return Ok(self);
        }
        let text = strings.items.last().map(|item| item.text()).unwrap_or("");
        if !text.contains(VERSION_MARKER) {
            return Ok(self);
        }
        let body = ArchiveVersion::decimal(&text.replace(VERSION_MARKER, ""))?;
        Ok(Self { body, ..self })
    }
}

fn user_strings(
    identity: HeaderObjectId,
    objects: &BTreeMap<HeaderObjectId, HeaderNode>,
) -> Result<&super::StringArray> {
    let node = objects.get(&identity).ok_or_else(|| {
        invalid(format!(
            "users object {}; expected a completed string array",
            identity.value()
        ))
    })?;
    match &node.object {
        HeaderObject::Strings(strings) => Ok(strings),
        object => Err(invalid(format!(
            "users object {:?}; expected su_CStringArray",
            object.kind()
        ))),
    }
}

pub(super) fn require_supported(
    kind: super::kinds::ClassKind,
    version: ArchiveVersion,
) -> Result<()> {
    if kind == super::kinds::ClassKind::External && version.value() < 0xf18 {
        return Err(super::unsupported(format!(
            "external-object archive version {}; expected >= 0xF18",
            version.value()
        )));
    }
    Ok(())
}
