"""Independent complete ellipse-backup contexts and literal scalar locations (#192)."""
from pathlib import Path
import hashlib
import itertools
import json
import struct
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT/'docs/features/model-archives'))
from fixture_object_map import WireMap, shared_object

ELLIPSE = 'moEllipseBackedUpData_c'
ARC = 'moArcBackedUpData_c'
WORDS = (-2147483648, -1431655766, -2, -1, 0, 1, 2, 1431655765, 2147483647)
VERSIONS = (11000, 18000, 19000)
POINT_BITS = ((0x8000000000000000, 0x7ff0000000000000, 0x7ff8000000001234),
              (0x7ff0000000000042, 1, 0xfff0000000000000))


def point_pointer(mapping: WireMap, identities: dict[str, int], style: int,
                  owner: int, position: int) -> bytes:
    """Retain null, shared or distinct point identities; e.g. point_pointer(w, ids, 0, 0, 0)."""
    if style == 0 or (style == 3 and position in (1, 5)):
        return bytes(2)
    key = f'point-{owner}-{position}' if style == 2 else f'point-{position % 2 if style == 3 else 0}'
    body = struct.pack('<3Q', *POINT_BITS[position % 2])
    return shared_object(mapping, identities, key, 'moPointBackedUpData_c', lambda: body)


def ellipse_body(mapping: WireMap, identities: dict[str, int], style: int,
                 owner: int, word: int, offsets: list[int]) -> bytes:
    """Locate the raw signed snapshot after the complete common prefix; e.g. ellipse_body(w, ids, 0, 0, -1, [])."""
    prefix = b''.join(point_pointer(mapping, identities, style, owner, index) for index in range(2))
    prefix += struct.pack('<2i', -2147483648, 2147483647)
    offsets.append(len(prefix))
    suffix = b''.join(point_pointer(mapping, identities, style, owner, index) for index in range(2, 7))
    return prefix + struct.pack('<i', word) + suffix


def second_ellipse(mapping: WireMap, identities: dict[str, int], style: int,
                   word: int, offsets: list[int], inherited: int) -> bytes:
    """Exercise a cached class independently of object aliases; e.g. second_ellipse(w, ids, 3, -2, offsets, 97)."""
    if style != 3:
        return shared_object(mapping, identities, 'second', ELLIPSE,
                             lambda: ellipse_body(mapping, identities, style, 1, word, offsets))
    mapping.slot += 1
    identities['second'] = mapping.slot
    return struct.pack('<H', 0x8000 | inherited) + ellipse_body(mapping, identities, style, 1, word, offsets)


def complete_ellipse_case(version: int, style: int, word: int,
                          inherited: int) -> tuple[bytes, list[int], int]:
    """Build two physical ellipses, an alias and an unrelated arc; e.g. complete_ellipse_case(19000, 0, -1, 97)."""
    mapping = WireMap(1)
    mapping.slot = inherited - 1
    identities: dict[str, int] = {}
    offsets: list[int] = []
    first = shared_object(mapping, identities, 'first', ELLIPSE,
                          lambda: ellipse_body(mapping, identities, style, 0, word, offsets))
    second = second_ellipse(mapping, identities, style, ~word, offsets, inherited)
    alias = struct.pack('<H', identities['first'])
    arc_body = bytes(4) + struct.pack('<3i', -2147483648, 2147483647, word) + bytes(10)
    arc = mapping.new(ARC, lambda: arc_body)
    starts = (6 + 6 + len(ELLIPSE), 6 + len(first) + (2 if style == 3 else 6 + len(ELLIPSE)))
    scalars = [starts[index] + offset for index, offset in enumerate(offsets)]
    scalars.append(6 + len(first) + len(second) + len(alias) + 6 + len(ARC) + 12)
    raw = struct.pack('<iH', -71, 4) + first + second + alias + arc
    return raw + (bytes(4) if version == 19000 else b''), scalars, mapping.slot + 1


contexts = ROOT/'tests/fixtures/bitmap-appearance/native-contexts.tsv'
inherited = {int(row.split('\t')[0]): int(row.split('\t')[3]) for row in contexts.read_text().splitlines()}
inherited[19000] = 97
cases: list[bytes] = []
rows: list[str] = []
for version, style, word in itertools.product(VERSIONS, range(4), WORDS):
    raw, offsets, slots = complete_ellipse_case(version, style, word, inherited[version])
    for offset, expected in zip(offsets, (word, ~word, word)):
        assert raw[offset:offset + 4] == struct.pack('<i', expected)
    rows.append('\t'.join(map(str, (len(cases), version, style, *offsets, word, ~word, word, slots))))
    cases.append(struct.pack('<II', version, len(raw)) + raw)
packet = b'EBR1' + struct.pack('<I', len(cases)) + b''.join(cases)
artifacts = {'roots.bin': packet, 'cases.tsv': ('\n'.join(rows) + '\n').encode('utf-8')}
destination = ROOT/'tests/fixtures/ellipse-backup-rotation'
if '--write-owned-ellipse-fixtures' in sys.argv:
    destination.mkdir(parents=True, exist_ok=True)
    for name, payload in artifacts.items():
        (destination/name).write_bytes(payload)
for name, payload in artifacts.items():
    assert (destination/name).read_bytes() == payload, name
assert not any(getattr(module, '__file__', None) and
               Path(module.__file__).resolve().is_relative_to(ROOT.parent/'inventor-interface')
               for module in sys.modules.copy().values())
report = {'issue': 192, 'cases': len(cases), 'physical_ellipses': 2 * len(cases),
          'aliases': len(cases), 'unrelated_arcs': len(cases), 'signed_words': WORDS,
          'archive_versions': VERSIONS, 'point_styles': 4, 'source_parser_executed': False,
          'context_reference_sha256': hashlib.sha256(contexts.read_bytes()).hexdigest(),
          'artifacts': {name: {'bytes': len(payload), 'sha256': hashlib.sha256(payload).hexdigest()}
                        for name, payload in artifacts.items()}}
Path(__file__).with_name('owned-fixture-reference.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
print(json.dumps({'cases': len(cases), 'bytes': len(packet)}))
