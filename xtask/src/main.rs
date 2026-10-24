use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let gate = std::env::args().nth(1).unwrap_or_default();
    if !matches!(gate.as_str(), "pr" | "full") {
        eprintln!("gate {gate:?}; expected pr or full");
        return ExitCode::FAILURE;
    }
    match run_gate() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run_gate() -> Result<(), String> {
    let commands: &[&[&str]] = &[
        &["fmt", "--all", "--", "--check"],
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
        &["build", "--workspace", "--locked"],
        &["test", "--workspace", "--locked"],
    ];
    for arguments in commands {
        run_cargo(arguments)?;
    }
    Ok(())
}

fn run_cargo(arguments: &[&str]) -> Result<(), String> {
    let program = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let status = Command::new(program)
        .args(arguments)
        .status()
        .map_err(|error| format!("cargo {arguments:?}: {error}; expected executable cargo"))?;
    if status.success() {
        return Ok(());
    }
    Err(format!(
        "cargo {arguments:?}: {status}; expected successful exit"
    ))
}
