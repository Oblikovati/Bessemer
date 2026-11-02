"""Independent complete dimension carriers and original primitive oracles (#179)."""
from pathlib import Path
from typing import TYPE_CHECKING, cast
import hashlib
import importlib
import json
import struct
import sys
from dimension_state_evidence import FEATURE, SOURCE
if TYPE_CHECKING:
    from freeze_owned_features import ReferenceDocument

GEOMETRIC_DIMENSIONS = ('moNumberDim_c', 'moScalerDim_c', 'moScalerLengthDim_c',
    'ParallelPlaneDistanceDim_c', 'ThreeDRadiusDim_c', 'ThreeDdiameterDim_c')
REFERENCE_DIMENSIONS = ('moRefDim_c', 'moRefDim1E_c', 'moRefDim2E_c')
DIMENSION_CLASSES = ('Dimension_c', *GEOMETRIC_DIMENSIONS, 'AngleDim_c',
    'ThreeDRadialOrDiameterDim_c', 'moRevolvedRadiusDim_c', *REFERENCE_DIMENSIONS,
    'sgValG1Dim', 'sgValG2Dim', 'sgValG3Dim', 'sgOffsetDim')
SIGNED_IDENTIFIERS = (-2147483648, -1, 0, 1, 2147483647)


def verify_owned_dimension_sources() -> None:
    """Pin the inherited-map and derived-body readers used by the owned cases; e.g. verify_owned_dimension_sources()."""
    pins = cast(dict[str, str], json.loads((FEATURE / 'owned-reader-sources.json').read_bytes()))
    assert len(pins) == 6, f'{len(pins)}; expected six additional owned-carrier reader pins'
    for name, expected in pins.items():
        actual = hashlib.sha256((SOURCE / name).read_bytes()).hexdigest()
        assert actual == expected, f'{name}/{actual}; expected owned-carrier reader {expected}'


def lock_dimension_artifact(path: Path, raw: bytes | bytearray) -> None:
    """Require immutable owned evidence outside explicit initialization; e.g. lock_dimension_artifact(path, raw)."""
    path.parent.mkdir(parents=True, exist_ok=True)
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged dimension evidence'


def owned_dimension_base(mask: int, identifier: int, text: bool) -> bytes:
    """Keep raw signed neighbors and optional Unicode/NUL text; e.g. owned_dimension_base(2, -1, True)."""
    prefix = struct.pack('<iHHiiIiHH', -7, 0, 0, 19, -31, mask, identifier, 0, 65535)
    assert len(prefix) == 28, f'{len(prefix)}; expected the exact fixed modern dimension prefix'
    optional = b'\xff\xff\xfe\xff\x03\xa9\x03\x00\x00\x42\x00' if text else b'\0'
    return prefix + optional


def owned_dimension_suffix(name: str) -> bytes:
    """Use complete independently framed class bodies with null references; e.g. owned_dimension_suffix('AngleDim_c')."""
    geometric = struct.pack('<9Q3H', 0x8000000000000000, 0x7ff8000000000042, *range(7), 0, 0, 0)
    if name == 'Dimension_c':
        return b''
    if name in GEOMETRIC_DIMENSIONS:
        return geometric
    if name in REFERENCE_DIMENSIONS:
        return bytes(64 + (0, 8, 20)[REFERENCE_DIMENSIONS.index(name)])
    if name in ('AngleDim_c', 'ThreeDRadialOrDiameterDim_c', 'moRevolvedRadiusDim_c'):
        return geometric + bytes({'AngleDim_c': 2, 'ThreeDRadialOrDiameterDim_c': 4, 'moRevolvedRadiusDim_c': 6}[name])
    widths = {'sgValG1Dim': 48, 'sgValG2Dim': 50, 'sgValG3Dim': 20, 'sgOffsetDim': 16}
    assert name in widths, f'{name}; expected one of the seventeen complete owned dimension classes'
    return bytes(widths[name])


def owned_dimension_stream(name: str, mask: int, identifier: int, text: bool) -> tuple[bytes, list[int]]:
    """Store new/loaded definitions and an alias against the owned97-slot map; e.g. owned_dimension_stream('Dimension_c', 0, -1, False)."""
    encoded = name.encode('ascii')
    tag = struct.pack('<3H', 65535, 1, len(encoded)) + encoded
    suffix = owned_dimension_suffix(name)
    first = owned_dimension_base(mask, identifier, text) + suffix
    other_id = struct.unpack('<i', struct.pack('<I', (identifier & 0xffffffff) ^ 0x80000000))[0]
    second = owned_dimension_base(mask ^ 0x8000001e, other_id, not text) + suffix
    header = struct.pack('<iH', -71, 3)
    starts = [len(header) + len(tag), len(header) + len(tag) + len(first) + 2]
    return header + tag + first + struct.pack('<H', 0x8061) + second + struct.pack('<HI', 98, 0), starts


def owned_dimension_context() -> tuple['ReferenceDocument', list[str]]:
    """Use the existing owned complete configuration without modifying it; e.g. owned_dimension_context()."""
    fixture = FEATURE.parents[2] / 'tests/fixtures/blank-part.SLDPRT'
    actual = hashlib.sha256(fixture.read_bytes()).hexdigest()
    assert actual == 'b52e080ba2cc64d1d07ed28306f27281b07521aef550bc0c7fea8cd54be2b276', actual
    document = importlib.import_module('swcontainer').Document(fixture, table={})
    _, context = importlib.import_module('modelprefix').read(document, 'Contents/Config-0',
        through_part_resolved=True, configuration_context=True)
    assert len(context.map) == 97, f'{len(context.map)}; expected the original inherited97-slot map'
    names = ('Contents/Config-0-ModelHeader', 'Contents/Config-0', 'Contents/CMgr')
    return document, [hashlib.sha256(document.read(name)).hexdigest() for name in names]


def verify_owned_dimension_values(parsed: dict[str, object], name: str, mask: int, identifier: int) -> None:
    """Check both physical values and reference identity through original semantics; e.g. verify_owned_dimension_values(result, 'Dimension_c', 0, -1)."""
    items = cast(list[dict[str, object]], parsed['items'])
    assert len(items) == 3 and items[0] is items[2], name
    assert len(cast(list[object], getattr(parsed['archive'], 'map'))) == 100, name
    other_id = struct.unpack('<i', struct.pack('<I', (identifier & 0xffffffff) ^ 0x80000000))[0]
    for item, word, redundant in zip(items[:2], (mask, mask ^ 0x8000001e), (identifier, other_id)):
        assert item['_class'] == name, item['_class']
        base = cast(dict[str, object], item['dimension']) if name in REFERENCE_DIMENSIONS else item
        signed = struct.unpack('<i', struct.pack('<I', word))[0]
        assert base['mask'] == signed and base['redundant_id'] == base['unknown_5c'] == redundant, base
        assert base['enforce_bend_angle_range'] == bool(word & 2), base
        assert base['fillet_ref_point_change_number'] == (word >> 2) & 7, base
        assert base['mask_unresolved_bits'] == word & 0xffffffe1, base
        assert base['reference_count'] == -7 and base['am_object_id'] == 19, base
        assert base['is_design_table_dimension'] == -31 and base['driven_dimension'] == 65535, base
        assert base['has_string_78'] == (255 if word & 2 else 0), base
        if word & 2:
            assert base['unknown_78'] == '\u03a9\0B', base


def capture_dimension_primitives(document: 'ReferenceDocument', raw: bytes, starts: list[int], index: int) -> tuple[dict[str, object], bytes]:
    """Corroborate literal scalar offsets and the whole byte partition; e.g. capture_dimension_primitives(doc, raw, starts, 0)."""
    from freeze_owned_features import ReplacedFeatureDocument, STREAM
    from primitive_inventory import observe_primitives, verify_primitive_coverage
    archive = importlib.import_module('swarchive')
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = importlib.import_module('resolvedfeatures').read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    physical = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, physical)
    expected = [(start + delta, start + delta + 4) for start in starts for delta in (0, 8, 12, 16, 20)]
    words = [read for read in physical if (read.start, read.end) in expected]
    observed = [(read.start, read.end) for read in words]
    assert observed == expected, f'{index}/{observed}; expected all ten literal dimension-base i32 spans {expected}'
    assert all(read.kind == 'i' and read.source_file in ('dimensions.py', 'modelcomponents.py') for read in words), words
    oracle = ''.join(f'{index}\t{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in physical).encode()
    return parsed, oracle


def append_dimension_case(document: 'ReferenceDocument', name: str, mask: int, identifier: int,
                          packet: bytearray, rows: list[str], primitives: bytearray) -> None:
    """Retain complete two-definition carriers and every original physical read; e.g. append_dimension_case(doc, name, 0, -1, packet, rows, primitives)."""
    text = bool((mask >> 1) & 1)
    raw, starts = owned_dimension_stream(name, mask, identifier, text)
    parsed, physical = capture_dimension_primitives(document, raw, starts, len(rows))
    assert parsed['end'] == len(raw), f'{name}; expected complete EOF at{len(raw)}'
    verify_owned_dimension_values(parsed, name, mask, identifier)
    packet.extend(struct.pack('<I', len(raw)) + raw)
    primitives.extend(physical)
    rows.append(f'{name}\t{mask}\t{identifier}\t{starts[0]}\t{starts[1]}\t{len(raw)}\n')


def persist_dimension_oracles(packet: bytearray, rows: list[str], primitives: bytearray, contexts: list[str]) -> None:
    """Keep independent literal and original-reader artifacts immutable; e.g. persist_dimension_oracles(packet, rows, primitives, contexts)."""
    destination = FEATURE.parents[2] / 'tests/fixtures/dimension-stored-state'
    oracle = ''.join(rows).encode()
    for name, raw in (('owned.bin', packet), ('dimensions.tsv', oracle), ('primitives.tsv', primitives)):
        lock_dimension_artifact(destination / name, raw)
    report = dict(classes=list(DIMENSION_CLASSES), cases=len(rows), physical_definitions=2 * len(rows), aliases=len(rows),
        context_streams_sha256=contexts, inherited_slots=97, final_slots=100, wire_bytes=len(packet) - 4 * len(rows),
        packet_bytes=len(packet), packet_sha256=hashlib.sha256(packet).hexdigest(),
        dimensions_sha256=hashlib.sha256(oracle).hexdigest(), primitive_reads=len(primitives.splitlines()),
        primitives_sha256=hashlib.sha256(primitives).hexdigest(), scalar_spans_checked=10 * len(rows))
    lock_dimension_artifact(FEATURE / 'owned-reference.json', (json.dumps(report, indent=2) + '\n').encode())


def verify_owned_dimension_archives() -> None:
    """Cover every stored flag/counter combination and signed-ID limit in complete carriers; e.g. verify_owned_dimension_archives()."""
    verify_owned_dimension_sources()
    document, contexts = owned_dimension_context()
    packet, primitives = bytearray(), bytearray()
    rows: list[str] = []
    combinations = [(selection << 1) | extra for selection in range(16) for extra in (0, 1, 0x80000000, 0xffffffe1)]
    for name in DIMENSION_CLASSES:
        for mask, identifier in ((mask, identifier) for mask in combinations for identifier in SIGNED_IDENTIFIERS):
            append_dimension_case(document, name, mask, identifier, packet, rows, primitives)
    assert len(rows) == 5440, f'{len(rows)}; expected17 classes times64 masks times5 signed identifiers'
    persist_dimension_oracles(packet, rows, primitives, contexts)
