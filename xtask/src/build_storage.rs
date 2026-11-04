use std::ffi::OsString;
use std::process::Command;

pub(super) fn gate_cargo_command(program: OsString, arguments: &[&str]) -> Command {
    let mut command = Command::new(program);
    command.args(arguments);
    // Hundreds of test binaries exceeded hosted disk; keep assertions and overflow checks (#196).
    command.env("CARGO_PROFILE_TEST_DEBUG", "0");
    command.env("CARGO_PROFILE_TEST_INCREMENTAL", "false");
    command
}

#[cfg(test)]
mod tests {
    use super::gate_cargo_command;
    use std::collections::BTreeMap;
    use std::ffi::OsStr;

    #[test]
    fn gate_bounds_only_transient_test_artifacts_without_changing_requested_workload() {
        let arguments = ["test", "--workspace", "--locked"];
        let command = gate_cargo_command("cargo-fixture".into(), &arguments);
        assert_eq!(command.get_program(), OsStr::new("cargo-fixture"));
        let actual: Vec<_> = command.get_args().collect();
        let expected: Vec<_> = arguments.iter().map(OsStr::new).collect();
        assert_eq!(actual, expected);
        let environment: BTreeMap<_, _> = command.get_envs().collect();
        let expected = BTreeMap::from([
            (
                OsStr::new("CARGO_PROFILE_TEST_DEBUG"),
                Some(OsStr::new("0")),
            ),
            (
                OsStr::new("CARGO_PROFILE_TEST_INCREMENTAL"),
                Some(OsStr::new("false")),
            ),
        ]);
        assert_eq!(environment, expected);
    }

    #[test]
    fn all_gate_subcommands_keep_their_complete_arguments_and_avoid_global_environment_mutation() {
        let before: BTreeMap<_, _> = std::env::vars_os().collect();
        for arguments in [
            vec!["fmt", "--all"],
            vec!["clippy", "--workspace", "--all-targets"],
            vec!["build", "--workspace"],
        ] {
            let command = gate_cargo_command("cargo-fixture".into(), &arguments);
            let actual: Vec<_> = command.get_args().collect();
            let expected: Vec<_> = arguments.iter().map(OsStr::new).collect();
            assert_eq!(actual, expected);
        }
        assert_eq!(std::env::vars_os().collect::<BTreeMap<_, _>>(), before);
    }
}
