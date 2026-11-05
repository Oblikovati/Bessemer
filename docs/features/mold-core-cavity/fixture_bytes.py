"""Literal complete modern core/cavity composition using reviewed inherited codecs (#215)."""
from pathlib import Path
import struct
import hashlib
import json
from itertools import product
import sys

ROOT = Path(__file__).resolve().parents[3]
for name in ('model-archives', 'reference-axes', 'sweeps', 'resolved-features', 'circular-patterns', 'combine-bodies', 'solid-references', 'weld-corners', 'weldments', 'sheet-metal'):
    sys.path.insert(0, str(ROOT / 'docs/features' / name))
from fillet_fixture_bytes import WireMap, body
from fixture_object_map import shared_object
from combine_body_bytes import body_component
from profile_fixture_bytes import contents as profile_body
from sweep_fixture_bytes import dimension

FEATURE = 'moMoldCoreCavitySolids_c'
PAYLOAD = 'moMoldCoreCavitySolidsData_c'


def direct_body_lists(mapping: WireMap, ids: dict[str, int], rich: bool, version: int) -> bytes:
    """Preserve three direct counts with shared and null items; e.g. direct_body_lists(mapping, ids, True, 19000)."""
    raw = b''
    for _ in range(3):
        if not rich:
            raw += bytes(2)
            continue
        raw += struct.pack('<HI', 65535, 3)
        raw += body_component(mapping, ids, version, 1) + bytes(2)
        raw += body_component(mapping, ids, version, 1)
    return raw


def mold_data(mapping: WireMap, ids: dict[str, int], rich: bool, setback: int, version: int) -> bytes:
    """Retain the parent alias and non-offset scalar/dimension order; e.g. mold_data(mapping, ids, True, -7, 19000)."""
    raw = struct.pack('<H', ids['feature']) + direct_body_lists(mapping, ids, rich, version)
    if rich:
        raw += shared_object(mapping, ids, 'profile', 'moProfileFeature_c',
            lambda: profile_body(mapping, version, False, 0, False, mapping.slot))
    else:
        raw += bytes(2)
    raw += struct.pack('<ii', -7, setback)
    dimensions = ('moDisplayDistanceDim_c', 'moDisplayDistanceDim_c', 'moDisplayAngularDim_c')
    for name in dimensions:
        raw += dimension(mapping, version, ids, name) if rich else bytes(2)
    return raw


def complete_mold_feature(rich: bool, setback: int = -2147483648, first_slot: int = 0, version: int = 19000) -> tuple[bytes, int, dict[str, int]]:
    """Assign feature identity before its data's owner back-reference; e.g. complete_mold_feature(True)."""
    mapping = WireMap(1)
    mapping.slot = first_slot
    ids: dict[str, int] = {}
    def contents() -> bytes:
        inherited = body(mapping, version, rich, ids['feature'])
        return inherited + shared_object(mapping, ids, 'data', PAYLOAD,
            lambda: mold_data(mapping, ids, rich, setback, version))
    raw = shared_object(mapping, ids, 'feature', FEATURE, contents)
    return raw, mapping.slot + 1, ids


def literal_mold_cases() -> dict[str, tuple[bytes, int, dict[str, int]]]:
    """Build all independent bare/owned originals and edits; e.g. literal_mold_cases()['owned-1.bin']."""
    cases: dict[str, tuple[bytes, int, dict[str, int]]] = {}
    for owned, rich, edited, minimum in product((False, True), repeat=4):
        if owned and minimum:
            continue
        raw, slots, identities = complete_mold_feature(rich, 12345 if edited else -2147483648, 96 if owned else 0, 0x27cc if minimum else 19000)
        if owned:
            raw = struct.pack('<iH', -123, 1) + raw
        name = ('owned' if owned else 'feature') + ('-minimum' if minimum else '') + f'-{int(rich)}' + ('-edited' if edited else '') + '.bin'
        cases[name] = raw, slots, identities
    return cases


def verify_literal_mold_cases() -> None:
    """Compare bytes and independently assigned map identities; e.g. python fixture_bytes.py."""
    reference = json.loads(Path(__file__).with_name('fixture-reference.json').read_text(encoding='utf-8'))
    for name, (raw, slots, identities) in literal_mold_cases().items():
        assert raw == (ROOT / 'tests/fixtures/mold-core-cavity' / name).read_bytes(), name
        assert hashlib.sha256(raw).hexdigest() == reference['files'][name], name
        expected = reference['cases'][name]
        assert (len(raw), slots, identities) == (expected['bytes'], expected['slots'], expected['identities']), name


if __name__ == '__main__':
    verify_literal_mold_cases()
