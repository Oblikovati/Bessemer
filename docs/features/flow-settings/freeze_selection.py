"""Freeze the native normal-return selector across every precedence byte and GUID byte position."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
import flowsettings


def guid_cases() -> list[bytes]:
    keys = [bytes(16)]
    for index in range(16):
        key = bytearray(16)
        key[index] = index + 1
        keys.append(bytes(key))
    return keys


def selection_row(incoming: bytes, cached: bytes, precedence: int) -> bytes:
    selected = flowsettings.select_static_settings(incoming, cached, from_fld=precedence)
    reason = {"fld_precedence": 0, "cached_identity": 1, "identity_changed": 2}[selected["reason"]]
    return (incoming + cached + bytes([precedence, reason, selected["load_required"]])
            + selected["cached_guid_after_loaders_return"] + bytes([selected["overwrite_warning"]]))


def freeze(output_root: Path = ROOT) -> None:
    keys = guid_cases()
    rows = [selection_row(incoming, cached, precedence)
            for incoming in keys for cached in keys for precedence in range(256)]
    packet = b"FSS1" + struct.pack("<I", len(rows)) + b"".join(rows)
    (output_root / "tests/fixtures/flow-settings/selection.bin.gz").write_bytes(gzip.compress(packet, mtime=0))
    report = dict(cases=len(rows), guid_values=len(keys), precedence_bytes=256,
                  bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                  reference_sha256=hashlib.sha256((SOURCE / "flowsettings.py").read_bytes()).hexdigest(),
                  scope="All precedence bytes; nil and one distinguishing value in each of sixteen byte positions")
    (output_root / "docs/features/flow-settings/fixture-verification.json").write_text(
        json.dumps(report, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(dict(cases=len(rows), packet_bytes=len(packet)))


if __name__ == "__main__":
    freeze()
