use super::*;
use std::collections::BTreeSet;
use std::time::{SystemTime, UNIX_EPOCH};

struct FakeTargetWorkspace {
    root: PathBuf,
}

impl FakeTargetWorkspace {
    fn new() -> Self {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "bessemer-tsan-shards-{}-{stamp}",
            std::process::id()
        ));
        std::fs::create_dir_all(root.join("src")).unwrap();
        std::fs::create_dir(root.join("tests")).unwrap();
        let fixture = Self { root };
        fixture.write("Cargo.toml", "[package]\nname = 'fixture'\n");
        fixture.write("src/lib.rs", "");
        fixture
    }
    fn write(&self, relative: &str, text: &str) {
        let path = self.root.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
}

impl Drop for FakeTargetWorkspace {
    fn drop(&mut self) {
        let root = self.root.canonicalize().unwrap();
        let temporary = std::env::temp_dir().canonicalize().unwrap();
        assert_eq!(root.parent(), Some(temporary.as_path()));
        assert!(
            root.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("bessemer-tsan-shards-")
        );
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn shard_parser_preserves_default_and_normalizes_every_valid_boundary() {
    assert_eq!(parse_shard(&[]).unwrap(), None);
    for total in 1..=64 {
        for index in 0..total {
            let arguments = ["--shard".into(), format!("{index}/{total}")];
            assert_eq!(
                parse_shard(&arguments).unwrap(),
                Some(SanitizerShard { index, total })
            );
        }
    }
}

#[test]
fn invalid_shards_and_unrecognized_arguments_fail_explicitly() {
    for value in ["", "0", "-1/4", "x/4", "0/x", "0/0", "4/4", "0/65", "0/4/2"] {
        let error = parse_shard(&["--shard".into(), value.into()]).unwrap_err();
        assert!(error.contains("expected"));
    }
    for arguments in [
        vec!["--shard".into()],
        vec!["--wrong".into(), "0/4".into()],
        vec!["--shard".into(), "0/4".into(), "extra".into()],
    ] {
        assert!(parse_shard(&arguments).unwrap_err().contains("expected"));
    }
}

#[test]
fn every_partition_union_is_complete_disjoint_and_stable() {
    let mut targets = vec![SanitizerTarget::Library];
    targets.extend((0..31).map(|index| SanitizerTarget::Integration(format!("test_{index:02}"))));
    for total in 1..=64 {
        let mut union = Vec::new();
        for index in 0..total {
            let selected = select_targets(&targets, SanitizerShard { index, total });
            assert_eq!(
                selected,
                select_targets(&targets, SanitizerShard { index, total })
            );
            union.extend(selected);
        }
        let unique: BTreeSet<_> = union.iter().map(|target| format!("{target:?}")).collect();
        assert_eq!(unique.len(), targets.len());
        assert_eq!(union.len(), targets.len());
    }
    assert!(select_targets(&targets[..1], SanitizerShard { index: 1, total: 2 }).is_empty());
}

#[test]
fn discovery_matches_cargo_default_files_and_directory_targets_in_order() {
    let fixture = FakeTargetWorkspace::new();
    fixture.write("tests/zeta.rs", "");
    fixture.write("tests/alpha/main.rs", "");
    fixture.write("tests/support/helper.rs", "");
    fixture.write("tests/readme.txt", "");
    assert_eq!(
        discover_targets(&fixture.root).unwrap(),
        [
            SanitizerTarget::Library,
            SanitizerTarget::Integration("alpha".into()),
            SanitizerTarget::Integration("zeta".into())
        ]
    );
}

#[test]
fn unsupported_manifests_binary_targets_and_duplicate_names_never_silently_drop_tests() {
    for marker in [
        "[[test]]",
        "[[bin]]",
        "[[example]]",
        "autotests = false",
        "autobins = true",
        "autolib = false",
        "[lib]",
    ] {
        let fixture = FakeTargetWorkspace::new();
        fixture.write("Cargo.toml", marker);
        assert!(
            discover_targets(&fixture.root)
                .unwrap_err()
                .contains("expected")
        );
    }
}

#[test]
fn binary_targets_and_duplicate_names_cannot_escape_the_plan() {
    let fixture = FakeTargetWorkspace::new();
    fixture.write("src/main.rs", "");
    assert!(
        discover_targets(&fixture.root)
            .unwrap_err()
            .contains("expected")
    );
    let duplicate = FakeTargetWorkspace::new();
    duplicate.write("tests/same.rs", "");
    duplicate.write("tests/same/main.rs", "");
    assert!(
        discover_targets(&duplicate.root)
            .unwrap_err()
            .contains("unique")
    );
}

#[test]
fn missing_manifest_or_test_directory_reports_the_offending_workspace() {
    let fixture = FakeTargetWorkspace::new();
    let missing = fixture.root.join("absent");
    let error = discover_targets(&missing).unwrap_err();
    assert!(error.contains("absent") && error.contains("expected"));
    std::fs::remove_dir(fixture.root.join("tests")).unwrap();
    assert!(
        discover_targets(&fixture.root)
            .unwrap_err()
            .contains("tests/*.rs")
    );
}
#[test]
fn repository_default_targets_are_covered_once_by_the_eight_ci_partitions() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let targets = discover_targets(root).unwrap();
    assert!(matches!(targets.first(), Some(SanitizerTarget::Library)));
    assert!(targets.len() > 8);
    let mut all = Vec::new();
    for index in 0..8 {
        all.extend(select_targets(&targets, SanitizerShard { index, total: 8 }));
    }
    let expected: BTreeSet<_> = targets.iter().map(|target| format!("{target:?}")).collect();
    let actual: BTreeSet<_> = all.iter().map(|target| format!("{target:?}")).collect();
    assert_eq!(actual, expected);
    assert_eq!(all.len(), targets.len());
}

#[test]
fn workflow_requires_every_instrumented_partition_before_full_platform_gates() {
    let workflow = include_str!("../../.github/workflows/bessemer.yml").replace("\r\n", "\n");
    assert!(workflow.contains("shard: [0, 1, 2, 3, 4, 5, 6, 7]"));
    assert!(workflow.contains("cargo xtask tsan --shard ${{ matrix.shard }}/8"));
    assert!(workflow.contains("  full:\n    needs: tsan\n"));
    assert!(workflow.contains("fail-fast: false"));
}
