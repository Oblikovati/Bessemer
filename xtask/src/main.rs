use std::process::{Command, ExitCode};

mod sanitizer;

fn main() -> ExitCode {
    let gate = std::env::args().nth(1).unwrap_or_default();
    match requested_gate(&gate) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn requested_gate(gate: &str) -> Result<(), String> {
    match gate {
        "pr" | "full" => run_gate(),
        "tsan" => sanitizer::run(),
        _ => Err(format!("gate {gate:?}; expected pr, full or tsan")),
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

#[cfg(test)]
mod tests {
    #[test]
    fn rejects_unknown_gate_without_running_a_process() {
        let error = super::requested_gate("unknown").unwrap_err();
        assert!(error.contains("unknown"));
        assert!(error.contains("expected pr, full or tsan"));
    }
}
