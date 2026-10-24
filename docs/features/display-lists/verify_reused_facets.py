"""Keep the earlier clause-12 native baseline unchanged while reusing its codec in display lists."""
from pathlib import Path
import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
from collections.abc import Iterator

sys.path.insert(0, str(Path("../inventor-interface/solidworks").resolve()))
import facettess
import swcontainer
import swcover

BASELINE = "a105ed302113e04c050e436f7fd6f0e224870219b8f95a1bf8106aab9f4a1113"


def native_records() -> Iterator[tuple[str, str, bytes]]:
    for path in sorted(map(Path, swcover.collect([]))):
        document = swcontainer.Document(str(path))
        # #32: the frozen baseline retains central-directory order, not sorted stream names.
        for entry in document.entries:
            if entry.name.startswith("FaceTessellations/"):
                relative = path.relative_to(Path(swcover.SAMPLES)).as_posix()
                yield relative, entry.name, document.read(entry.name)


def extract(root: Path) -> tuple[dict[str, list[tuple[Path, Path]]], dict[str, object]]:
    groups: dict[str, list[tuple[Path, Path]]] = {"directory": [], "block": []}
    digest = hashlib.sha256()
    counts = {"directories": 0, "blocks": 0, "faces": 0, "bytes": 0}
    for relative, name, raw in native_records():
        kind = "directory" if name == "FaceTessellations/Directory" else "block"
        parsed = facettess.directory(raw) if kind == "directory" else facettess.block(raw)
        counts["directories" if kind == "directory" else "blocks"] += 1
        counts["faces"] += 0 if kind == "directory" else sum(len(item["faces"]) for item in parsed)
        counts["bytes"] += len(raw)
        digest.update(relative.encode()+b"\0"+name.encode()+b"\0"+raw)
        source = root/f"{kind}-{len(groups[kind])}.bin"
        source.write_bytes(raw)
        groups[kind].append((source, source.with_suffix(".rebuilt")))
    return groups, {**counts, "manifest_sha256": digest.hexdigest()}


def verify(executable: str) -> None:
    with tempfile.TemporaryDirectory(prefix="bessemer-reused-facets-") as temporary:
        root = Path(temporary)
        local = root/Path(executable).name
        shutil.copyfile(executable, local)
        groups, evidence = extract(root)
        assert evidence["manifest_sha256"] == BASELINE, evidence
        for kind, records in groups.items():
            for start in range(0, len(records), 20):
                arguments = [str(path) for pair in records[start:start+20] for path in pair]
                subprocess.run([str(local), "19000", "full", kind, *arguments], check=True, capture_output=True)
            for source, destination in records:
                assert source.read_bytes() == destination.read_bytes(), source.name
    evidence["result"] = "all complete forced reconstructions equal the frozen earlier baseline"
    Path(__file__).with_name("reused-facet-verification.json").write_text(json.dumps(evidence, indent=2)+"\n", encoding="utf-8", newline="\n")
    print(json.dumps(evidence))


if __name__ == "__main__":
    verify(sys.argv[1])
