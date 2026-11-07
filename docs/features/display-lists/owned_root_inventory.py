"""Read complete owned-root frames with their persisted kinds, versions and hashes."""
from dataclasses import dataclass
from pathlib import Path
import hashlib
import json
import struct

FIXED_PART_GROUPS = frozenset({"bucket-vectors","item-vectors","light-vectors","reference-vectors","scene-vectors"})


@dataclass(frozen=True)
class OwnedRoot:
    label: str
    version: int
    kind: str
    file_version: int
    external_ids: tuple[str,...]
    payload: bytes

    def context_row(self) -> dict[str,object]:
        return {"version":self.version,"kind":self.kind,"file_version":self.file_version,
            "external_ids":list(self.external_ids)}


@dataclass
class RootFrameCursor:
    payload: bytes
    position: int = 0

    def take(self, count: int) -> bytes:
        end = self.position+count
        if count<0 or end>len(self.payload):
            raise ValueError(f"frame offset {self.position}, size {count}; expected within {len(self.payload)} bytes")
        result = self.payload[self.position:end]
        self.position = end
        return result

    def u32(self) -> int:
        value, = struct.unpack("<I",self.take(4))
        return value

    def blob(self) -> bytes:
        return self.take(self.u32())

    def finish(self) -> None:
        if self.position!=len(self.payload):
            raise ValueError(f"frame end {self.position}; expected {len(self.payload)}")


def root_rows(manifest: Path) -> tuple[dict[str,object],list[dict[str,object]]] | None:
    content = json.loads(manifest.read_text(encoding="utf-8"))
    if not isinstance(content,dict):return None
    rows = content.get("fixtures")
    if not isinstance(rows,list) or not rows:return None
    if manifest.stem in {"version-range","legacy-buckets"}:
        rows = [dict(row,kind="part",scenario=row.get("scenario",int(row.get("edited",False)))) for row in rows]
    required = {"version","kind","scenario","bytes","sha256"}
    if not all(isinstance(row,dict) and required<=row.keys()
               and row["kind"] in ("part","assembly") for row in rows):return None
    return content,rows


def read_context(cursor: RootFrameCursor, row: dict[str,object], name: str) -> None:
    if name.startswith("file-context-"):
        actual = struct.unpack("<IIBBII",cursor.take(18))
        expected = (row["version"],row["file_version"],row["kind"]=="assembly",row["scenario"],
            row["aggregate_entries"],row["objects"])
    elif name=="version-range":
        actual = struct.unpack("<IB",cursor.take(5))
        expected = (row["version"],row["edited"])
    else:
        actual = struct.unpack("<IBB",cursor.take(6))
        flag = row["two_dimensional"] if name=="legacy-buckets" else row["kind"]=="assembly"
        expected = (row["version"],flag,row["scenario"])
    if actual!=expected:raise ValueError(f"{name} context {actual}; expected {expected}")


def read_root(cursor: RootFrameCursor, row: dict[str,object], name: str, index: int) -> OwnedRoot:
    read_context(cursor,row,name)
    payload = cursor.blob()
    actual = hashlib.sha256(payload).hexdigest()
    if actual!=row["sha256"] or len(payload)!=row["bytes"]:
        raise ValueError(f"{name}[{index}] hash {actual}, length {len(payload)}; expected {row['sha256']}, {row['bytes']}")
    cursor.blob()
    if name not in {"graphics-references","version-range","legacy-buckets"} and not name.startswith("file-context-"):cursor.blob()
    return OwnedRoot(f"{name}[{index}]",row["version"],row["kind"],row.get("file_version",19000),
        tuple(row.get("external_ids",[])),payload)


def inventory_group(manifest: Path, fixtures: Path) -> list[OwnedRoot]:
    binary = fixtures/(manifest.stem+".bin")
    if not binary.is_file():return []
    parsed = root_rows(manifest)
    if parsed is None:return []
    content,rows = parsed
    cursor = RootFrameCursor(binary.read_bytes())
    actual = hashlib.sha256(cursor.payload).hexdigest()
    expected_hash = content.get("sha256",content.get("fixture_sha256",actual))
    if expected_hash!=actual:
        raise ValueError(f"{binary} hash {actual}; expected {expected_hash}")
    count = cursor.u32()
    if count!=len(rows):raise ValueError(f"{binary} count {count}; expected {len(rows)}")
    roots = [read_root(cursor,row,manifest.stem,index) for index,row in enumerate(rows)]
    cursor.finish()
    return roots


def inventory_roots(evidence: Path, fixtures: Path) -> list[OwnedRoot]:
    framed = [root for manifest in sorted(evidence.glob("*.json"))
        for root in inventory_group(manifest,fixtures)]
    return framed+fixed_context_roots(evidence,fixtures)


def fixed_context_roots(evidence: Path, fixtures: Path) -> list[OwnedRoot]:
    groups = [*sorted(FIXED_PART_GROUPS),"shaded-vectors","component-vectors"]
    roots = []
    for name in groups:
        manifest = evidence/(name+".json")
        if not manifest.is_file():continue
        content = json.loads(manifest.read_text(encoding="utf-8"))
        rows = content if isinstance(content,list) else content["fixtures"]
        roots.extend(fixed_context_root(name,index,row,fixtures) for index,row in enumerate(rows))
    return roots


def fixed_context_root(name: str, index: int, row: dict[str,object], fixtures: Path) -> OwnedRoot:
    fixture = f"component-mode-{row['mode']}" if name=="component-vectors" else row["fixture"]
    kind = "assembly" if name=="component-vectors" or fixture=="external-shaded-assembly" else "part"
    payload = (fixtures/(fixture+".bin")).read_bytes()
    actual = hashlib.sha256(payload).hexdigest()
    if actual!=row["sha256"] or len(payload)!=row["bytes"]:
        raise ValueError(f"{fixture} hash {actual}, length {len(payload)}; expected {row['sha256']}, {row['bytes']}")
    return OwnedRoot(f"{name}[{index}]",19000,kind,19000,tuple(row.get("external_components",[])),payload)
