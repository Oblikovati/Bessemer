use super::invalid;
use crate::Result;

pub(super) fn valid_character(character: char) -> bool {
    matches!(character,'\t'|'\n'|'\r'|'\u{20}'..='\u{d7ff}'|'\u{e000}'..='\u{fffd}'|'\u{10000}'..='\u{10ffff}')
}

pub(super) fn check_characters(text: &str) -> Result<()> {
    if let Some(character) = text.chars().find(|character| !valid_character(*character)) {
        return Err(invalid(format!(
            "XML character U+{:04X}; expected XML 1.0 character range",
            character as u32
        )));
    }
    Ok(())
}

fn name_start(character: char) -> bool {
    matches!(character,'A'..='Z'|'_'|'a'..='z'|'\u{c0}'..='\u{d6}'|'\u{d8}'..='\u{f6}'|'\u{f8}'..='\u{2ff}'|'\u{370}'..='\u{37d}'|'\u{37f}'..='\u{1fff}'|'\u{200c}'..='\u{200d}'|'\u{2070}'..='\u{218f}'|'\u{2c00}'..='\u{2fef}'|'\u{3001}'..='\u{d7ff}'|'\u{f900}'..='\u{fdcf}'|'\u{fdf0}'..='\u{fffd}'|'\u{10000}'..='\u{effff}')
}

fn name_character(character: char) -> bool {
    name_start(character)
        || matches!(character,'-'|'.'|'0'..='9'|'\u{b7}'|'\u{300}'..='\u{36f}'|'\u{203f}'..='\u{2040}')
}

pub(super) fn qualified(name: &str) -> Result<(Option<&str>, &str)> {
    let mut parts = name.split(':');
    let first = parts.next().unwrap_or_default();
    ncname(first)?;
    let Some(local) = parts.next() else {
        return Ok((None, first));
    };
    ncname(local)?;
    if parts.next().is_some() {
        return Err(invalid(format!(
            "XML qualified name {name:?}; expected zero or one colon"
        )));
    }
    Ok((Some(first), local))
}

fn ncname(name: &str) -> Result<()> {
    let mut characters = name.chars();
    if characters.next().is_some_and(name_start) && characters.all(name_character) {
        return Ok(());
    }
    Err(invalid(format!(
        "XML name {name:?}; expected an XML 1.0 NCName"
    )))
}

pub(super) fn processing_target(name: &str) -> Result<()> {
    let mut characters = name.chars();
    let valid = characters
        .next()
        .is_some_and(|character| name_start(character) || character == ':')
        && characters.all(|character| name_character(character) || character == ':');
    if valid && !name.eq_ignore_ascii_case("xml") {
        return Ok(());
    }
    Err(invalid(format!(
        "XML PI target {name:?}; expected an XML Name other than xml"
    )))
}

pub(super) fn space(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\r' | '\n')
}
