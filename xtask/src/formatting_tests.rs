use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const REGISTRY: &str = "src/model_archives/test_modules.rs";
const PRIVATE_CASE: &str = "tests/private_cases/example.rs";

struct FormattingCargo {
    root: PathBuf,
    commands: Vec<String>,
}

impl FormattingCargo {
    fn new() -> Result<Self, String> {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("fixture time {error}; expected time after UNIX_EPOCH"))?;
        let root = std::env::temp_dir().join(format!(
            "bessemer-fmt-{}-{}",
            std::process::id(),
            stamp.as_nanos()
        ));
        fs::create_dir(&root).map_err(|error| {
            format!("fixture root {root:?}: {error}; expected new writable directory")
        })?;
        let fixture = Self {
            root,
            commands: Vec::new(),
        };
        fixture.write_fixture()?;
        Ok(fixture)
    }

    fn write_fixture(&self) -> Result<(), String> {
        for directory in ["src/model_archives", "tests/private_cases"] {
            fs::create_dir_all(self.root.join(directory)).map_err(|error| {
                format!("fixture directory {directory:?}: {error}; expected writable directory")
            })?;
        }
        for (name, text) in [
            (
                "Cargo.toml",
                "[package]\nname=\"fmt-fixture\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n",
            ),
            ("src/lib.rs", "mod model_archives;\n"),
            (
                "src/model_archives/mod.rs",
                "#[cfg(test)]\ninclude!(\"test_modules.rs\");\n",
            ),
            (
                REGISTRY,
                "#[path = \"../../tests/private_cases/example.rs\"]\nmod example_tests;\n",
            ),
            (PRIVATE_CASE, "#[test]\nfn example(){assert_eq!(1,1);}\n"),
        ] {
            fs::write(self.root.join(name), text).map_err(|error| {
                format!("fixture file {name:?}: {error}; expected writable UTF-8 file")
            })?;
        }
        Ok(())
    }

    fn execute(&mut self, arguments: &[&str]) -> Result<(), String> {
        let Some(command) = arguments.first() else {
            return Err("cargo arguments []; expected a subcommand".into());
        };
        self.commands.push((*command).to_owned());
        if *command != "fmt" {
            return Ok(());
        }
        self.format(arguments)
    }

    fn format(&self, arguments: &[&str]) -> Result<(), String> {
        let arguments: Vec<OsString> = arguments
            .iter()
            .map(|argument| {
                if *argument == super::MODEL_TEST_REGISTRY {
                    return self.root.join(REGISTRY).into_os_string();
                }
                OsString::from(argument)
            })
            .collect();
        let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        let output = Command::new(cargo)
            .current_dir(&self.root)
            .args(&arguments)
            .output()
            .map_err(|error| {
                format!("fixture cargo {arguments:?}: {error}; expected executable cargo")
            })?;
        if output.status.success() {
            return Ok(());
        }
        Err(format!(
            "fixture cargo {arguments:?}: {}; expected formatted private modules; {}{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

impl Drop for FormattingCargo {
    fn drop(&mut self) {
        let cleanup = fs::remove_dir_all(&self.root);
        if !std::thread::panicking() {
            assert!(
                cleanup.is_ok(),
                "fixture cleanup {:?}: {cleanup:?}",
                self.root
            );
        }
    }
}

#[test]
fn gate_checks_private_modules_behind_include_and_stops_before_later_steps() {
    let mut cargo = FormattingCargo::new().unwrap();
    // #69: ordinary cargo fmt misses the included registry and its nested private suite.
    cargo.format(&["fmt", "--all", "--", "--check"]).unwrap();
    let error = super::run_commands(|arguments| cargo.execute(arguments)).unwrap_err();
    assert!(error.contains("expected formatted private modules"));
    assert_eq!(cargo.commands, ["fmt"]);
    cargo
        .format(&["fmt", "--all", "--", super::MODEL_TEST_REGISTRY])
        .unwrap();
    let private = fs::read_to_string(cargo.root.join(PRIVATE_CASE)).unwrap();
    assert!(private.contains("fn example() {"));
    super::run_commands(|arguments| cargo.execute(arguments)).unwrap();
}
