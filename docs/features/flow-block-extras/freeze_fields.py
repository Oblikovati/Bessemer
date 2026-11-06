"""Freeze supplementary field metadata without changing the FBE1 wire baseline."""
import gzip
import hashlib
import json
import struct
from pathlib import Path
import field_oracle
import freeze_extras as baseline


def freeze(output_root: Path = baseline.ROOT) -> None:
    cases = [baseline.native_case()] + baseline.owned_cases()
    packet = b"FEO1" + struct.pack("<I", len(cases)) + b"".join(map(field_oracle.case_packet, cases))
    destination = output_root / "tests/fixtures/flow-block-extras/fields.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 field_kinds=field_oracle.KINDS, entity_kinds=field_oracle.ENTITIES,
                 scope="Complete independently reconstructed scopes, field types, strides, every raw element, structural admission and stored/derived indices")
    report = output_root / "docs/features/flow-block-extras/field-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
