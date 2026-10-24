use super::{ExpandedName, invalid};
use crate::Result;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
    sync::Arc,
};

pub(super) const XML_URI: &str = "http://www.w3.org/XML/1998/namespace";
const XMLNS_URI: &str = "http://www.w3.org/2000/xmlns/";
pub(super) type NamePool = BTreeSet<Arc<str>>;

pub(super) struct QualifiedName {
    pub prefix: Option<Arc<str>>,
    pub local: Arc<str>,
}
pub(super) struct ParsedAttribute {
    pub name: QualifiedName,
    pub value: String,
    pub span: Range<usize>,
}
pub(super) struct NamespaceScope {
    bindings: BTreeMap<Arc<str>, Vec<Option<Arc<str>>>>,
}

pub(super) fn intern(pool: &mut NamePool, text: &str) -> Arc<str> {
    if let Some(value) = pool.get(text) {
        return value.clone();
    }
    let value: Arc<str> = Arc::from(text);
    pool.insert(value.clone());
    value
}

pub(super) fn qualified(pool: &mut NamePool, name: &str) -> Result<QualifiedName> {
    let (prefix, local) = super::grammar::qualified(name)?;
    Ok(QualifiedName {
        prefix: prefix.map(|value| intern(pool, value)),
        local: intern(pool, local),
    })
}

impl ParsedAttribute {
    pub fn declaration(&self) -> Option<&str> {
        if self.name.prefix.as_deref() == Some("xmlns") {
            return Some(&self.name.local);
        }
        if self.name.prefix.is_none() && self.name.local.as_ref() == "xmlns" {
            return Some("");
        }
        None
    }
}

impl NamespaceScope {
    pub fn new() -> Self {
        let mut bindings = BTreeMap::new();
        bindings.insert(Arc::from("xml"), vec![Some(Arc::from(XML_URI))]);
        Self { bindings }
    }

    pub fn push(
        &mut self,
        attributes: &[ParsedAttribute],
        pool: &mut NamePool,
    ) -> Result<Vec<Arc<str>>> {
        let mut declared = Vec::new();
        for attribute in attributes {
            let Some(prefix) = attribute.declaration() else {
                continue;
            };
            require_binding(prefix, &attribute.value)?;
            let prefix = intern(pool, prefix);
            let uri = if attribute.value.is_empty() {
                None
            } else {
                Some(intern(pool, &attribute.value))
            };
            self.bindings.entry(prefix.clone()).or_default().push(uri);
            declared.push(prefix);
        }
        Ok(declared)
    }

    pub fn pop(&mut self, declared: Vec<Arc<str>>) {
        for prefix in declared.into_iter().rev() {
            if let Some(bindings) = self.bindings.get_mut(&prefix) {
                bindings.pop();
                if bindings.is_empty() {
                    self.bindings.remove(&prefix);
                }
            }
        }
    }

    pub fn resolve(&self, name: &QualifiedName, element: bool) -> Result<ExpandedName> {
        let prefix = name.prefix.as_deref();
        if prefix == Some("xmlns") {
            return Err(invalid(
                "XML element prefix xmlns; expected a non-reserved prefix",
            ));
        }
        let namespace = self.lookup_namespace(prefix, element);
        if prefix.is_some() && namespace.is_none() {
            return Err(invalid(format!(
                "XML prefix {prefix:?}; expected an in-scope nonempty namespace binding"
            )));
        }
        Ok(ExpandedName {
            namespace,
            local: name.local.clone(),
        })
    }
    fn lookup_namespace(&self, prefix: Option<&str>, element: bool) -> Option<Arc<str>> {
        if prefix.is_none() && !element {
            return None;
        }
        self.bindings
            .get(prefix.unwrap_or_default())
            .and_then(|bindings| bindings.last())
            .cloned()
            .flatten()
    }
}

fn require_binding(prefix: &str, uri: &str) -> Result<()> {
    if (!prefix.is_empty() && uri.is_empty())
        || prefix == "xmlns"
        || uri == XMLNS_URI
        || (prefix == "xml" && uri != XML_URI)
        || (prefix != "xml" && uri == XML_URI)
    {
        return Err(invalid(format!(
            "XML namespace declaration {prefix:?}={uri:?}; expected reserved xml/xmlns namespace rules"
        )));
    }
    Ok(())
}
