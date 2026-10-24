use std::process::Command;

const TOOLCHAIN: &str = "nightly-2026-10-23";
const TARGET: &str = "x86_64-unknown-linux-gnu";

pub(super) fn run() -> Result<(), String> {
    supported_platform(std::env::consts::OS, std::env::consts::ARCH)?;
    let status = sanitizer_command().status().map_err(|error| {
        format!("rustup run {TOOLCHAIN}: {error}; expected installed toolchain with rust-src")
    })?;
    if status.success() {
        return Ok(());
    }
    Err(format!(
        "ThreadSanitizer tests returned {status}; expected successful instrumented tests"
    ))
}

fn supported_platform(os: &str, architecture: &str) -> Result<(), String> {
    if os == "linux" && architecture == "x86_64" {
        return Ok(());
    }
    Err(format!(
        "ThreadSanitizer host {os}/{architecture}; expected linux/x86_64"
    ))
}

fn sanitizer_command() -> Command {
    let mut command = Command::new("rustup");
    command.args([
        "run",
        TOOLCHAIN,
        "cargo",
        "test",
        "--package",
        "bessemer",
        "--tests",
        "--locked",
        "-Zbuild-std",
        "--target",
        TARGET,
    ]);
    command.env("RUSTFLAGS", "-Zsanitizer=thread");
    command.env("RUSTDOCFLAGS", "-Zsanitizer=thread");
    command.env("CARGO_TARGET_DIR", "target/thread-sanitizer");
    command
}

#[cfg(test)]
mod tests {
    use super::{sanitizer_command, supported_platform};

    #[test]
    fn platform_guard_is_explicit() {
        assert!(supported_platform("linux", "x86_64").is_ok());
        for (os, architecture) in [("windows", "x86_64"), ("linux", "aarch64")] {
            let error = supported_platform(os, architecture).unwrap_err();
            assert!(error.contains(os));
            assert!(error.contains(architecture));
            assert!(error.contains("expected linux/x86_64"));
        }
    }

    #[test]
    fn instruments_the_library_dependencies_and_standard_library() {
        let command = sanitizer_command();
        let arguments: Vec<_> = command
            .get_args()
            .map(|value| value.to_string_lossy().into_owned())
            .collect();
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
                .any(|pair| pair == ["--target", "x86_64-unknown-linux-gnu"])
        );
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
        assert!(environment.contains(&(
            "CARGO_TARGET_DIR".into(),
            Some("target/thread-sanitizer".into())
        )));
    }
}
