use super::sanitizer_shard::{SanitizerTarget, discover_targets, parse_shard, select_targets};
use std::path::Path;
use std::process::Command;

const TOOLCHAIN: &str = "nightly-2026-10-23";
const TARGET: &str = "x86_64-unknown-linux-gnu";

pub(super) fn run() -> Result<(), String> {
    supported_platform(std::env::consts::OS, std::env::consts::ARCH)?;
    let arguments: Vec<_> = std::env::args().skip(2).collect();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| "xtask workspace parent missing; expected workspace/xtask".to_owned())?;
    let targets = parse_shard(&arguments)?
        .map(|shard| discover_targets(root).map(|targets| select_targets(&targets, shard)))
        .transpose()?;
    if targets.as_ref().is_some_and(Vec::is_empty) {
        return Ok(());
    }
    let mut command = sanitizer_command(targets.as_deref());
    execute_sanitizer(&mut command)
}

fn supported_platform(os: &str, architecture: &str) -> Result<(), String> {
    if os == "linux" && architecture == "x86_64" {
        return Ok(());
    }
    Err(format!(
        "ThreadSanitizer host {os}/{architecture}; expected linux/x86_64"
    ))
}

fn sanitizer_command(targets: Option<&[SanitizerTarget]>) -> Command {
    let mut command = Command::new("rustup");
    command.args([
        "run",
        TOOLCHAIN,
        "cargo",
        "test",
        "--package",
        "bessemer",
        "--locked",
        "-Zbuild-std",
        "--target",
        TARGET,
    ]);
    append_target_arguments(&mut command, targets);
    command.env("RUSTFLAGS", "-Zsanitizer=thread");
    command.env("RUSTDOCFLAGS", "-Zsanitizer=thread");
    command.env("CARGO_TARGET_DIR", "target/thread-sanitizer");
    command
}

fn append_target_arguments(command: &mut Command, targets: Option<&[SanitizerTarget]>) {
    let Some(targets) = targets else {
        command.arg("--tests");
        return;
    };
    for target in targets {
        match target {
            SanitizerTarget::Library => {
                command.arg("--lib");
            }
            SanitizerTarget::Integration(name) => {
                command.args(["--test", name]);
            }
        }
    }
}

fn execute_sanitizer(command: &mut Command) -> Result<(), String> {
    let status = command.status().map_err(|error| {
        format!("rustup run {TOOLCHAIN}: {error}; expected installed toolchain with rust-src")
    })?;
    if status.success() {
        return Ok(());
    }
    Err(format!(
        "ThreadSanitizer tests returned {status}; expected successful instrumented tests"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn platform_guard_is_explicit() {
        assert!(supported_platform("linux", "x86_64").is_ok());
        for (os, architecture) in [("windows", "x86_64"), ("linux", "aarch64")] {
            let error = supported_platform(os, architecture).unwrap_err();
            assert!(
                error.contains(os)
                    && error.contains(architecture)
                    && error.contains("expected linux/x86_64")
            );
        }
    }
    #[test]
    fn instruments_the_library_dependencies_and_standard_library() {
        let command = sanitizer_command(None);
        let arguments = command_arguments(&command);
        assert!(arguments.iter().any(|argument| argument == "-Zbuild-std"));
        assert!(arguments.iter().any(|argument| argument == "--tests"));
        assert!(
            arguments
                .windows(2)
                .any(|pair| pair == ["--package", "bessemer"])
        );
        assert!(
            arguments
                .windows(2)
                .any(|pair| pair == ["--target", TARGET])
        );
        assert_instrumented_environment(&command);
    }
    fn command_arguments(command: &Command) -> Vec<String> {
        command
            .get_args()
            .map(|value| value.to_string_lossy().into_owned())
            .collect()
    }
    fn assert_instrumented_environment(command: &Command) {
        let environment: Vec<_> = command
            .get_envs()
            .map(|(key, value)| {
                (
                    key.to_string_lossy().into_owned(),
                    value.map(|value| value.to_string_lossy().into_owned()),
                )
            })
            .collect();
        assert!(environment.contains(&("RUSTFLAGS".into(), Some("-Zsanitizer=thread".into()))));
        assert!(environment.contains(&("RUSTDOCFLAGS".into(), Some("-Zsanitizer=thread".into()))));
        assert!(environment.contains(&(
            "CARGO_TARGET_DIR".into(),
            Some("target/thread-sanitizer".into())
        )));
    }
    #[test]
    fn partition_commands_select_only_their_declared_complete_targets() {
        let targets = [
            SanitizerTarget::Library,
            SanitizerTarget::Integration("simulation_post_dialog".into()),
        ];
        let command = sanitizer_command(Some(&targets));
        let arguments = command_arguments(&command);
        assert!(!arguments.iter().any(|argument| argument == "--tests"));
        assert!(arguments.iter().any(|argument| argument == "--lib"));
        assert!(
            arguments
                .windows(2)
                .any(|pair| pair == ["--test", "simulation_post_dialog"])
        );
        assert!(
            !arguments
                .iter()
                .any(|argument| argument == "--skip" || argument == "--ignored")
        );
    }
    struct FakeSanitizerProcess;
    impl FakeSanitizerProcess {
        fn command(fail: bool) -> Command {
            let mut command = Command::new(std::env::current_exe().unwrap());
            command.arg(if fail {
                "--invalid-sanitizer-test-option"
            } else {
                "--list"
            });
            command
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null());
            command
        }
    }
    #[test]
    fn process_failure_is_not_reported_as_completed_success() {
        assert!(execute_sanitizer(&mut FakeSanitizerProcess::command(false)).is_ok());
        let error = execute_sanitizer(&mut FakeSanitizerProcess::command(true)).unwrap_err();
        assert!(error.contains("expected successful instrumented tests"));
    }
}
