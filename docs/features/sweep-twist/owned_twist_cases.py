"""Retain complete twist streams and literal expected objects independently of Rust (#211)."""
from dataclasses import dataclass
from itertools import product
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
from owned_twist_oracle import owned_twist_stream
from twist_pointer_sequence import TwistStyle


@dataclass(frozen=True)
class OwnedTwistCase:
    label: str
    raw: bytes
    slots: int
    object_slots: tuple[int, ...]
    payloads: tuple[bytes, ...]


def owned_twist_cases(first_slot: int) -> list[OwnedTwistCase]:
    """Cross enclosing features, pointer forms and independent edits; e.g. owned_twist_cases(96)."""
    matrix = product(('moSweep_c', 'moSweepCut_c'), (False, True), ('fresh', 'shared', 'cached-class'))
    return [record for name, rich, style in matrix
            for record in twist_variants(first_slot, rich, name, cast(TwistStyle, style))]


def twist_variants(first_slot: int, rich: bool, name: str, style: TwistStyle) -> list[OwnedTwistCase]:
    """Change only a literal raw byte in each field; e.g. twist_variants(96,False,'moSweep_c','fresh')."""
    original = struct.pack('<BQQ', 255, 0x8000000000000000, 0x7FF8000000001234)
    records: list[OwnedTwistCase] = []
    for offset in (-1, 0, 1, 9):
        payload = bytearray(original)
        if offset >= 0:
            payload[offset] ^= 1
        payloads = (bytes(payload),) if style == 'fresh' else (bytes(payload), bytes(17))
        raw, slots, sequence = owned_twist_stream(first_slot, rich, name, style, payloads)
        label = f'{name}-{int(rich)}-{style}-{offset}'
        records.append(OwnedTwistCase(label, raw, slots, tuple(sequence.object_slots), payloads))
    return records


def write_owned_twists(destination: Path, records: list[OwnedTwistCase]) -> None:
    """Write a framed owned oracle packet; e.g. write_owned_twists(output,cases)."""
    packet = b'BWO1' + struct.pack('<I', len(records)) + b''.join(encode_owned_twist(row) for row in records)
    destination.mkdir(parents=True, exist_ok=True)
    (destination / 'owned-cases.bin').write_bytes(packet)
    manifest = dict(cases=len(records), packet_bytes=len(packet), packet_sha256=hashlib.sha256(packet).hexdigest(),
                    labels=[row.label for row in records], oracle='Original enclosing readers plus fixture-only twist primitives')
    (destination / 'owned-manifest.json').write_text(json.dumps(manifest, indent=2)+'\n', encoding='utf-8', newline='\n')


def encode_owned_twist(row: OwnedTwistCase) -> bytes:
    """Keep expected payload bits separate from full streams; e.g. encode_owned_twist(case)."""
    label = row.label.encode('ascii')
    raw = struct.pack('<H', len(label)) + label + struct.pack('<IH', row.slots, len(row.object_slots))
    raw += b''.join(struct.pack('<I', slot) for slot in row.object_slots)
    raw += struct.pack('<I', len(row.raw)) + row.raw
    return raw + struct.pack('<H', len(row.payloads)) + b''.join(row.payloads)
