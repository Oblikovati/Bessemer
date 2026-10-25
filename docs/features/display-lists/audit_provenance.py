"""Bind long-running audits to unchanged reader, adapter and runtime inputs."""
from collections.abc import Iterable
import hashlib
from pathlib import Path
from predicate_probe import MODULES

ADAPTERS = ("audit_version_predicates","registered_class_audit","predicate_probe",
    "owned_root_inventory","generate_graphics_references","verify_configuration_processes","reference_corpus")


def file_hashes(paths: Iterable[Path]) -> dict[str,str]:
    return {str(path).replace("\\","/"):hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(paths)}


def audit_source_snapshot(reference: Path, executable: Path, workspace: Path = Path("."), adapters: Path | None = None) -> dict[str,object]:
    readers = [reference/(name+".py") for name in sorted(MODULES|{"mfcobjects","swarchive"})]
    runtime = [*(workspace/"src").rglob("*.rs"),*(workspace/"examples").rglob("*.rs"),
        workspace/"Cargo.toml",workspace/"Cargo.lock",workspace/"rust-toolchain.toml"]
    adapter_root = Path(__file__).parent if adapters is None else adapters
    adapter_files = [adapter_root/(name+".py") for name in ADAPTERS]
    return {"reference":file_hashes(readers),"runtime":file_hashes(runtime),
        "adapters":file_hashes([*adapter_files,Path(__file__)]),
        "executable_sha256":hashlib.sha256(executable.read_bytes()).hexdigest()}


def require_unchanged_snapshot(before: dict[str,object], after: dict[str,object]) -> None:
    changed = [key for key in sorted(set(before)|set(after)) if before.get(key)!=after.get(key)]
    if changed:raise ValueError(f"audit inputs changed in {changed}; expected unchanged sources and executable throughout replay")
