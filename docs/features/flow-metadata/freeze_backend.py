"""Freeze exact backend-prefix boundaries before medium-object decoding."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
from wire_packet import ROOT, SOURCE, blob, native_records, flowmetadataprefix
import flowmediumbackend


def cases() -> list[tuple[str, bytes]]:
    native = []
    for name, raw in native_records(424):
        prefix = flowmetadataprefix.read_metadata_prefix(raw, string_mode=0)
        native.append((f"native-{name}", prefix["opaque_tail"]))
    result = native + [("empty", b"\0\0" + struct.pack("<i", 0) + b"\0" + struct.pack("<i", 0))]
    for table in range(2):
        for kind in [-2147483648, -1, *range(256), 2147483647]:
            for count in (1, 2147483647):
                raw = b"\0"
                if table:
                    raw += b"\0" + struct.pack("<i", 0)
                raw += b"\0" + struct.pack("<ii", count, kind)
                if table == 0 and kind == 1:
                    raw += bytes(range(16))
                result.append((f"table-{table}-kind-{kind}-count-{count}", raw + b"opaque\0\xff"))
    result.append(("empty-framing-with-tail", result[len(native)][1] + b"\x78\x9c\x00"))
    return result


def packet_case(name: str, raw: bytes) -> bytes:
    parsed = flowmediumbackend.read_backend_prefix(raw)
    out = blob(name.encode()) + blob(raw) + bytes((parsed["version"],))
    out += struct.pack("<I", len(parsed["tables"]))
    for table in parsed["tables"]:
        out += struct.pack("<IIBi", table["ordinal"], table["start"], table["version"], table["count"])
        kind = table.get("first_kind_i32")
        out += bytes((kind is not None,))
        if kind is not None:
            out += struct.pack("<i", kind)
        out += blob(table.get("first_key_raw", b""))
    out += struct.pack("<I", parsed["prefix_end"]) + blob(parsed["opaque_tail"])
    out += bytes((parsed["backend_framing_complete"], parsed["full_record_schema"], parsed["native_runtime_state_reproduced"]))
    return out + blob(parsed["stop_kind"].encode())


def freeze(output_root: Path = ROOT) -> None:
    rows = cases()
    packet = b"FMB1" + struct.pack("<I", len(rows)) + b"".join(packet_case(*row) for row in rows)
    destination = output_root / "tests/fixtures/flow-metadata/backend.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(rows), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowmediumbackend.py").read_bytes()).hexdigest(),
                 scope="Exact two-table backend framing through first populated object kind and conditional first-table kind-1 key; arbitrary tails are opaque, including completed empty framing. No full record or runtime acceptance.")
    report = output_root / "docs/features/flow-metadata/backend-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
