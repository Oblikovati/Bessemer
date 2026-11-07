"""Build independent full-carrier expectations for every existing-member edit."""
import gzip
import struct
from freeze_body import ROOT, blob, corpus, flowsettingsbody


def carrier(payload: bytes) -> bytes:
    records = [(999, b"before"), (511, payload), (111, b"after")]
    return b"\x08\0" + b"".join(struct.pack("<IQ", identifier, len(raw)) + raw for identifier, raw in records)


def changed_bytes(field: dict, raw: bytes) -> bytes:
    if field["kind"] == "u8":
        return bytes([raw[0] ^ 0xff])
    if field["kind"] == "u32":
        return struct.pack("<I", int.from_bytes(raw, "little") ^ 0x80000001)
    mask = 0xa5 if len(raw) == 4 else 0x5a
    return bytes(byte ^ mask for byte in raw)


def edit_row(version: int, payload: bytes, name: str, field: dict) -> bytes:
    start, width = field["offset"], field["size"]
    changed = changed_bytes(field, payload[start:start + width])
    patched = payload[:start] + changed + payload[start + width:]
    parsed = flowsettingsbody.read_payload(patched)
    assert parsed["end"] == len(payload)
    original_wire = carrier(payload)
    prefix = struct.pack("<IB", version, int(name.removeprefix("member_"), 16))
    wires = blob(original_wire) * 2 + blob(carrier(patched)) + blob(original_wire)
    history = struct.pack("<I", 3) + b"".join(struct.pack("<IIB", index, index, index == 1) for index in range(3))
    return prefix + wires + history


def packet() -> bytes:
    frozen = gzip.decompress((ROOT / "tests/fixtures/flow-settings-body/body.bin.gz").read_bytes())
    edits = []
    for row in corpus()["synthetic_fixtures"]:
        payload = bytes.fromhex(row["payload_hex"])
        fields = flowsettingsbody.read_payload(payload)["fields"]
        edits += [edit_row(row["version"], payload, name, field)
                  for name, field in fields.items() if name.startswith("member_")]
    assert len(edits) == 145
    return frozen + b"BSE1" + struct.pack("<I", len(edits)) + b"".join(edits)
