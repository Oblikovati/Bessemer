use super::namespaces::QualifiedName;
use super::{
    XmlLimits, encoding, grammar, invalid, lexical,
    namespaces::{NamePool, ParsedAttribute, qualified},
};
use crate::Result;
use quick_xml::{
    XmlVersion,
    events::{BytesStart, attributes::Attribute},
};
use std::sync::Arc;
use std::{collections::BTreeSet, ops::Range};

pub(super) fn parse_attributes(
    start: &BytesStart<'_>,
    source: &str,
    pool: &mut NamePool,
    total: &mut usize,
    limits: XmlLimits,
) -> Result<Vec<ParsedAttribute>> {
    let mut parsed = Vec::new();
    let mut names = BTreeSet::new();
    for attribute in start.attributes().with_checks(false) {
        let attribute = attribute.map_err(|error| {
            invalid(format!(
                "XML attribute {error}; expected quoted name/value pair"
            ))
        })?;
        charge_attribute(total, limits)?;
        let name = qualified(pool, attribute.key.as_ref())?;
        require_unique(&mut names, &name)?;
        let value = normalized_value(&attribute)?;
        let key_range = source_range(source, attribute.key.as_ref())?;
        let span = lexical::attribute_span(source, key_range.start)?;
        parsed.push(ParsedAttribute { name, value, span });
    }
    Ok(parsed)
}

fn source_range(source: &str, fragment: &str) -> Result<Range<usize>> {
    let start = (fragment.as_ptr() as usize)
        .checked_sub(source.as_ptr() as usize)
        .ok_or_else(|| invalid("XML token outside source; expected borrowed source token"))?;
    let end = start
        .checked_add(fragment.len())
        .ok_or_else(|| invalid("XML token end overflow; expected source range"))?;
    if source.get(start..end) != Some(fragment) {
        return Err(invalid(format!(
            "XML token range {start}..{end}; expected matching source bytes"
        )));
    }
    Ok(start..end)
}

fn charge_attribute(total: &mut usize, limits: XmlLimits) -> Result<()> {
    *total = total
        .checked_add(1)
        .ok_or_else(|| invalid("XML attribute count overflow; expected usize"))?;
    encoding::require_size(
        *total,
        limits.maximum_attributes,
        "XML attributes including namespace declarations",
    )
}

fn require_unique(
    names: &mut BTreeSet<(Option<Arc<str>>, Arc<str>)>,
    name: &QualifiedName,
) -> Result<()> {
    if names.insert((name.prefix.clone(), name.local.clone())) {
        return Ok(());
    }
    Err(invalid(format!(
        "duplicate XML attribute {:?}:{:?}; expected unique qualified names",
        name.prefix, name.local
    )))
}

fn normalized_value(attribute: &Attribute<'_>) -> Result<String> {
    if attribute.value.contains('<') {
        return Err(invalid("literal < in XML attribute; expected escaped &lt;"));
    }
    let value = attribute
        .normalized_value(XmlVersion::Explicit1_0)
        .map_err(|error| {
            invalid(format!(
                "XML attribute {error}; expected predefined or numeric references"
            ))
        })?
        .into_owned();
    grammar::check_characters(&value)?;
    Ok(value)
}
