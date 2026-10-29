use super::*;
use crate::archive_codec::ArchiveRevision;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
struct FileMaps {
    primary: BTreeMap<String, u64>,
    overrides: BTreeMap<String, u64>,
}

/// Immutable supplied maps; no runtime file registration or graph ownership is inferred.
#[derive(Debug, Clone)]
pub struct ComposerGeodeServer {
    files: Arc<BTreeMap<String, FileMaps>>,
    revision: ComposerServerRevision,
    limits: ComposerServerLimits,
    coverage: ComposerServerCoverage,
}

impl ComposerGeodeServer {
    /// Validate explicit map input, e.g. `ComposerGeodeServer::from_files(&files, limits)?`.
    pub fn from_files(
        files: &[ComposerSuppliedFile],
        limits: ComposerServerLimits,
    ) -> Result<Self> {
        preflight(files, limits)?;
        let mut normalized = BTreeMap::new();
        for supplied in files {
            add_supplied_file(&mut normalized, supplied)?;
        }
        Self::publish(normalized, limits)
    }

    fn publish(files: BTreeMap<String, FileMaps>, limits: ComposerServerLimits) -> Result<Self> {
        let coverage = check_limits(&files, limits)?;
        Ok(Self {
            files: Arc::new(files),
            revision: ComposerServerRevision(ArchiveRevision::default()),
            limits,
            coverage,
        })
    }

    /// Query override before primary while retaining miss output, e.g. `server.get(&file, &guid, 42)?`.
    pub fn get(
        &self,
        file: &ComposerReferenceText,
        guid: &ComposerReferenceText,
        initial_pointer: u64,
    ) -> Result<ComposerGeodeLookup> {
        let key = file.ascii()?.to_ascii_lowercase();
        let guid = guid.ascii()?;
        Ok(lookup(self.files.get(&key), &guid, initial_pointer))
    }

    /// Register on an isolated snapshot, e.g. `server.register(&file, &guid, 7)?`.
    pub fn register(
        &self,
        file: &ComposerReferenceText,
        guid: &ComposerReferenceText,
        incoming: u64,
    ) -> Result<ComposerRegistration> {
        let key = file.ascii()?.to_ascii_lowercase();
        let guid = guid.ascii()?;
        let previous = lookup(self.files.get(&key), &guid, 0);
        if previous.pointer != 0 {
            return Ok(self.registration(
                self.clone(),
                ComposerRegistrationBranch::ExistingNonnull,
                previous.pointer,
            ));
        }
        self.register_null(key, guid, incoming)
    }

    fn register_null(
        &self,
        key: String,
        guid: String,
        incoming: u64,
    ) -> Result<ComposerRegistration> {
        let mut files = self.files.as_ref().clone();
        let branch = insert(files.entry(key).or_default(), guid, incoming);
        let server = match branch {
            ComposerRegistrationBranch::InnerUnchanged => self.clone(),
            _ => Self::publish(files, self.limits)?,
        };
        let pointer = match branch {
            ComposerRegistrationBranch::Inserted(_) => incoming,
            _ => 0,
        };
        Ok(self.registration(server, branch, pointer))
    }

    fn registration(
        &self,
        server: Self,
        branch: ComposerRegistrationBranch,
        pointer: u64,
    ) -> ComposerRegistration {
        ComposerRegistration {
            completion: if self.revision == server.revision {
                crate::display_lists::DisplayEditCompletion::Unchanged
            } else {
                crate::display_lists::DisplayEditCompletion::Changed
            },
            fidelity: ComposerReferenceFidelity::ExactSuppliedState,
            parent: self.revision.clone(),
            result: server.revision.clone(),
            coverage: server.coverage,
            server,
            branch,
            pointer,
            detach_incoming_first_parent_if_present: branch
                == ComposerRegistrationBranch::ExistingNonnull,
            diagnostics: DIAGNOSTICS,
        }
    }

    /// Replay the final block with an explicit resolved file key, e.g. `server.lookup_nonempty(&file, &project, &guid)?`.
    pub fn lookup_nonempty(
        &self,
        file: &ComposerReferenceText,
        project: &ComposerReferenceText,
        guid: &ComposerReferenceText,
    ) -> Result<ComposerReferenceLookup> {
        if guid.0.is_empty() {
            return Err(invalid(
                "GUID has 0 units; expected nonempty GUID because empty-GUID merging is unresolved",
            ));
        }
        let guid = guid.ascii()?;
        let attempts = self.lookup_attempts(file, project, &guid)?;
        Ok(ComposerReferenceLookup {
            completion: ComposerReferenceCompletion::Completed,
            fidelity: ComposerReferenceFidelity::ExactSuppliedState,
            pointer: attempts.last().map_or(0, |attempt| attempt.result.pointer),
            attempts,
            revision: self.revision.clone(),
            coverage: self.coverage,
            diagnostics: DIAGNOSTICS,
        })
    }

    fn lookup_attempts(
        &self,
        file: &ComposerReferenceText,
        project: &ComposerReferenceText,
        guid: &str,
    ) -> Result<Vec<ComposerLookupAttempt>> {
        let mut attempts = Vec::new();
        let mut pointer = 0;
        for candidate in file.fallback_file_keys(project) {
            let key = candidate.ascii()?.to_ascii_lowercase();
            let result = lookup(self.files.get(&key), guid, pointer);
            pointer = result.pointer;
            attempts.push(ComposerLookupAttempt {
                file_key: candidate,
                result,
            });
            if pointer != 0 {
                break;
            }
        }
        Ok(attempts)
    }

    /// Exact supplied-state validation counts, e.g. `server.coverage().entries`.
    pub fn coverage(&self) -> ComposerServerCoverage {
        self.coverage
    }

    /// Immutable map identity, e.g. `server.revision()`.
    pub fn revision(&self) -> &ComposerServerRevision {
        &self.revision
    }

    /// Unexecuted native responsibilities, e.g. `server.diagnostics()`.
    pub fn diagnostics(&self) -> [ComposerReferenceDiagnostic; 5] {
        DIAGNOSTICS
    }
}

fn lookup(maps: Option<&FileMaps>, guid: &str, initial_pointer: u64) -> ComposerGeodeLookup {
    let mut tiers = maps.into_iter().flat_map(|maps| {
        [
            (ComposerGeodeTier::Override, &maps.overrides),
            (ComposerGeodeTier::Primary, &maps.primary),
        ]
    });
    let matched =
        tiers.find_map(|(tier, entries)| entries.get(guid).map(|pointer| (tier, *pointer)));
    lookup_result(matched, initial_pointer)
}

fn lookup_result(
    matched: Option<(ComposerGeodeTier, u64)>,
    initial_pointer: u64,
) -> ComposerGeodeLookup {
    ComposerGeodeLookup {
        found: matched.is_some(),
        pointer: matched.map_or(initial_pointer, |(_, pointer)| pointer),
        tier: matched.map(|(tier, _)| tier),
    }
}

fn insert(maps: &mut FileMaps, guid: String, incoming: u64) -> ComposerRegistrationBranch {
    let Some(previous) = maps.primary.get(&guid) else {
        maps.primary.insert(guid, incoming);
        return ComposerRegistrationBranch::Inserted(ComposerGeodeTier::Primary);
    };
    if *previous == incoming || maps.overrides.contains_key(&guid) {
        return ComposerRegistrationBranch::InnerUnchanged;
    }
    maps.overrides.insert(guid, incoming);
    ComposerRegistrationBranch::Inserted(ComposerGeodeTier::Override)
}

fn supplied_map(entries: &[(ComposerReferenceText, u64)]) -> Result<BTreeMap<String, u64>> {
    let mut normalized = BTreeMap::new();
    for (guid, pointer) in entries {
        let key = guid.ascii()?;
        if normalized.insert(key.clone(), *pointer).is_some() {
            return Err(invalid(format!(
                "duplicate GUID key {key:?}; expected unique exact keys within one supplied tier"
            )));
        }
    }
    Ok(normalized)
}

fn check_limits(
    files: &BTreeMap<String, FileMaps>,
    limits: ComposerServerLimits,
) -> Result<ComposerServerCoverage> {
    let mut coverage = ComposerServerCoverage {
        mode: ComposerReferenceCheck::ExactSuppliedState,
        files: files.len(),
        entries: 0,
        key_bytes: 0,
    };
    budget("files", coverage.files, limits.maximum_files)?;
    for (file, maps) in files {
        charge_key(&mut coverage, file.len(), limits)?;
        for (guid, _) in maps.primary.iter().chain(&maps.overrides) {
            coverage.entries = charged(coverage.entries, 1, limits.maximum_entries, "entries")?;
            charge_key(&mut coverage, guid.len(), limits)?;
        }
    }
    Ok(coverage)
}

fn charge_key(
    coverage: &mut ComposerServerCoverage,
    bytes: usize,
    limits: ComposerServerLimits,
) -> Result<()> {
    coverage.key_bytes = charged(
        coverage.key_bytes,
        bytes,
        limits.maximum_key_bytes,
        "key bytes",
    )?;
    Ok(())
}

fn charged(current: usize, addition: usize, maximum: usize, label: &str) -> Result<usize> {
    let total = current.checked_add(addition).ok_or_else(|| {
        invalid(format!(
            "{label} {current} + {addition}; expected usize sum at most {maximum}"
        ))
    })?;
    budget(label, total, maximum)?;
    Ok(total)
}

fn budget(label: &str, actual: usize, maximum: usize) -> Result<()> {
    if actual > maximum {
        return Err(invalid(format!(
            "{label} {actual}; expected at most {maximum}"
        )));
    }
    Ok(())
}

const DIAGNOSTICS: [ComposerReferenceDiagnostic; 5] = [
    ComposerReferenceDiagnostic::NativeLocaleNotMeasured,
    ComposerReferenceDiagnostic::FileResolutionNotPerformed,
    ComposerReferenceDiagnostic::GraphSideEffectsNotPerformed,
    ComposerReferenceDiagnostic::EmptyGuidMergingNotSupported,
    ComposerReferenceDiagnostic::NativeApplicationNotChecked,
];

fn add_supplied_file(
    files: &mut BTreeMap<String, FileMaps>,
    supplied: &ComposerSuppliedFile,
) -> Result<()> {
    let key = supplied.file.ascii()?.to_ascii_lowercase();
    if files.contains_key(&key) {
        return Err(invalid(format!(
            "duplicate file key {key:?}; expected unique ASCII-lowercase keys"
        )));
    }
    let maps = FileMaps {
        primary: supplied_map(&supplied.primary)?,
        overrides: supplied_map(&supplied.overrides)?,
    };
    files.insert(key, maps);
    Ok(())
}

fn preflight(files: &[ComposerSuppliedFile], limits: ComposerServerLimits) -> Result<()> {
    budget("supplied files", files.len(), limits.maximum_files)?;
    let mut entries = 0;
    let mut bytes = 0;
    for supplied in files {
        bytes = charged(
            bytes,
            supplied.file.0.len(),
            limits.maximum_key_bytes,
            "key bytes",
        )?;
        for (guid, _) in supplied.primary.iter().chain(&supplied.overrides) {
            entries = charged(entries, 1, limits.maximum_entries, "entries")?;
            bytes = charged(bytes, guid.0.len(), limits.maximum_key_bytes, "key bytes")?;
        }
    }
    Ok(())
}
