use std::path::Path;

pub(super) fn run() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().skip(2).collect();
    let [flag, version] = arguments.as_slice() else {
        return Err(format!(
            "release arguments {arguments:?}; expected --version X.Y.Z"
        ));
    };
    if flag != "--version" {
        return Err(format!("release flag {flag:?}; expected --version"));
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let manifest = read_release_text(&root.join("Cargo.toml"))?;
    let changelog = read_release_text(&root.join("CHANGELOG.md"))?;
    validate_release(version, &manifest, &changelog)
}

fn read_release_text(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|error| format!("release file {path:?}: {error}; expected readable UTF-8 text"))
}

fn validate_release(version: &str, manifest: &str, changelog: &str) -> Result<(), String> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 || parts.iter().any(|part| !valid_component(part)) {
        return Err(format!(
            "release version {version:?}; expected canonical X.Y.Z"
        ));
    }
    let declaration = format!("version = \"{version}\"");
    if !manifest.lines().any(|line| line == declaration) {
        return Err(format!(
            "release version {version:?}; expected matching Cargo.toml version"
        ));
    }
    let heading = format!("## [{version}] - ");
    if !changelog.lines().any(|line| line.starts_with(&heading)) {
        return Err(format!(
            "release version {version:?}; expected dated CHANGELOG section"
        ));
    }
    Ok(())
}

fn valid_component(component: &str) -> bool {
    component == "0"
        || (!component.starts_with('0')
            && !component.is_empty()
            && component.bytes().all(|byte| byte.is_ascii_digit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_requires_matching_version_and_dated_notes() {
        let manifest = "version = \"0.1.0\"";
        let notes = "## [Unreleased]\n\n## [0.1.0] - 2026-11-07";
        assert!(validate_release("0.1.0", manifest, notes).is_ok());
        assert!(validate_release("0.2.0", manifest, notes).is_err());
        assert!(validate_release("0.1.0", manifest, "## [Unreleased]").is_err());
        for version in ["", "v0.1.0", "0.01.0", "0.1", "0.1.0-beta"] {
            assert!(validate_release(version, manifest, notes).is_err());
        }
    }

    #[test]
    fn components_are_canonical_decimal_words() {
        assert!(valid_component("0"));
        assert!(valid_component("123"));
        for component in ["", "00", "01", "-1", "a"] {
            assert!(!valid_component(component));
        }
    }

    #[test]
    fn release_text_reports_missing_path() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("missing-release-fixture");
        let error = read_release_text(&path).unwrap_err();
        assert!(error.contains("missing-release-fixture"));
        assert!(error.contains("expected readable UTF-8 text"));
    }
}
