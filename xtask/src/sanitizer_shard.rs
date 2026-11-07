use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SanitizerShard {
    index: usize,
    total: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SanitizerTarget {
    Library,
    Integration(String),
}

pub(super) fn parse_shard(arguments: &[String]) -> Result<Option<SanitizerShard>, String> {
    if arguments.is_empty() {
        return Ok(None);
    }
    let [flag, value] = arguments else {
        return Err(format!(
            "arguments {arguments:?}; expected --shard INDEX/TOTAL"
        ));
    };
    if flag != "--shard" {
        return Err(format!("flag {flag:?}; expected --shard INDEX/TOTAL"));
    }
    parse_shard_value(value).map(Some)
}

pub(super) fn select_targets(
    targets: &[SanitizerTarget],
    shard: SanitizerShard,
) -> Vec<SanitizerTarget> {
    targets
        .iter()
        .enumerate()
        .filter(|(index, _)| index % shard.total == shard.index)
        .map(|(_, target)| target.clone())
        .collect()
}

fn validate_layout(root: &Path, manifest: &str) -> Result<(), String> {
    validate_manifest_layout(manifest)?;
    if !root.join("src/lib.rs").is_file()
        || root.join("src/main.rs").exists()
        || root.join("src/bin").exists()
    {
        return Err(format!(
            "target layout under {root:?}; expected a default library without binary targets"
        ));
    }
    Ok(())
}

fn integration_name(path: PathBuf) -> Result<Option<String>, String> {
    let candidate = if path.is_dir() && path.join("main.rs").is_file() {
        path.file_name()
    } else if path.is_file() && path.extension().is_some_and(|extension| extension == "rs") {
        path.file_stem()
    } else {
        return Ok(None);
    };
    let name = candidate
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("test path {path:?}; expected a UTF-8 Rust target name"))?;
    Ok(Some(name.to_owned()))
}

pub(super) fn discover_targets(root: &Path) -> Result<Vec<SanitizerTarget>, String> {
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).map_err(|error| {
        format!("manifest under {root:?}: {error}; expected readable Cargo.toml")
    })?;
    validate_layout(root, &manifest)?;
    let names = integration_names(root)?;
    let mut targets = vec![SanitizerTarget::Library];
    targets.extend(names.into_iter().map(SanitizerTarget::Integration));
    Ok(targets)
}

fn parse_shard_value(value: &str) -> Result<SanitizerShard, String> {
    let Some((index, total)) = value.split_once('/') else {
        return Err(format!("shard {value:?}; expected INDEX/TOTAL"));
    };
    let index = index
        .parse::<usize>()
        .map_err(|error| format!("index {index:?}: {error}; expected unsigned integer"))?;
    let total = total
        .parse::<usize>()
        .map_err(|error| format!("total {total:?}: {error}; expected integer1 through64"))?;
    if total == 0 || total > 64 || index >= total {
        return Err(format!("shard {value:?}; expected0 <= INDEX < TOTAL <=64"));
    }
    Ok(SanitizerShard { index, total })
}

fn validate_manifest_layout(manifest: &str) -> Result<(), String> {
    let unsupported = [
        "[[test",
        "[[bin",
        "[[example",
        "autotests",
        "autobins",
        "autolib",
        "[lib]",
    ];
    if let Some(marker) = unsupported
        .iter()
        .find(|marker| manifest.contains(**marker))
    {
        return Err(format!(
            "manifest marker {marker:?}; expected default src/lib.rs and tests/*.rs target layout for sanitizer partitioning"
        ));
    }
    Ok(())
}

fn integration_names(root: &Path) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(root.join("tests")).map_err(|error| {
        format!("test directory under {root:?}: {error}; expected readable tests/*.rs")
    })?;
    let mut names = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|error| {
                format!("test directory {root:?}: {error}; expected a readable entry")
            })?
            .path();
        if let Some(name) = integration_name(path)? {
            names.push(name);
        }
    }
    names.sort();
    validate_unique_names(&names)?;
    Ok(names)
}

fn validate_unique_names(names: &[String]) -> Result<(), String> {
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(format!(
            "integration names {names:?}; expected unique Cargo test target names"
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "sanitizer_shard_tests.rs"]
mod tests;
