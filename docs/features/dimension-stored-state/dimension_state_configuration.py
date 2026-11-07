"""Complete owned configuration carrier with physically defined dimension map entries (#179)."""
from pathlib import Path
from typing import TYPE_CHECKING, cast
import hashlib
import importlib
import json
import struct
from dimension_state_evidence import FEATURE, SOURCE
from dimension_state_owned import owned_dimension_context, owned_dimension_stream, lock_dimension_artifact
if TYPE_CHECKING:
    from freeze_owned_features import ReferenceDocument

CONFIGURATION_STREAM = 'Contents/Config-0'
CONFIGURATION_FEATURES = struct.pack('<iHI', -71, 0, 0)


class ReplacedDimensionConfiguration:
    """Supply an owned configuration and feature stream with matching inherited slots."""
    def __init__(self, original: 'ReferenceDocument', replacement: bytes) -> None:
        """Retain the immutable original context; e.g. ReplacedDimensionConfiguration(document, wire)."""
        self.original = original
        self.replacement = replacement

    def names(self) -> list[str]:
        """Keep the original stream inventory; e.g. document.names()."""
        return self.original.names()

    def read(self, name: str) -> bytes:
        """Keep new inherited slots out of stale feature aliases; e.g. document.read('Contents/Config-0')."""
        if name == 'Contents/Config-0-ResolvedFeatures':
            return CONFIGURATION_FEATURES
        return self.replacement if name == CONFIGURATION_STREAM else self.original.read(name)


def dimension_configuration_map() -> tuple[bytes, list[int]]:
    """Frame three map entries with independent signed keys and two physical bases; e.g. dimension_configuration_map()."""
    raw, bases = owned_dimension_stream('Dimension_c', 0x8000001e, -2147483648, True)
    first, second, alias = raw[6:bases[1] - 2], raw[bases[1] - 2:-6], raw[-6:-4]
    packet = bytearray(struct.pack('<ii', 1, 3))
    starts: list[int] = []
    for key, record, offset in ((-71, first, bases[0] - 6), (-72, second, 2), (-73, alias, None)):
        packet.extend(struct.pack('<i', key))
        if offset is not None:
            starts.append(len(packet) + offset)
        packet.extend(record)
    packet.extend(struct.pack('<i', 0))
    return bytes(packet), starts


def configuration_dimension_insertion(document: 'ReferenceDocument') -> tuple[bytes, list[int], int]:
    """Locate the original absent map through unchanged Source primitive calls; e.g. configuration_dimension_insertion(document)."""
    from primitive_inventory import observe_primitives
    archive = importlib.import_module('swarchive')
    module = importlib.import_module('partresolved')
    lines = Path(module.__file__).read_text().splitlines()
    matching = [index + 1 for index, line in enumerate(lines) if line.strip() == "out['map_presence'] = a.i32()"]
    assert len(matching) == 1, f'{matching}; expected the single pinned map-presence assignment'
    original = document.read(CONFIGURATION_STREAM)
    with observe_primitives(archive.Archive, original) as reads:
        root, context = importlib.import_module('modelprefix').read(document, configuration_context=True, through_part_resolved=True)
    observed = [read for read in reads if read.source_file == 'partresolved.py' and read.source_line == matching[0]]
    assert len(observed) == 1 and observed[0].raw_hex == '00000000', observed
    assert context.p == len(original) and len(context.map) == 97, context.p
    return inserted_dimension_configuration(original, root, observed[0].start)


def inserted_dimension_configuration(original: bytes, root: dict[str, object], offset: int) -> tuple[bytes, list[int], int]:
    """Update the existing resolved extent after one literal map replacement; e.g. inserted_dimension_configuration(raw, root, offset)."""
    replacement, bases = dimension_configuration_map()
    result = bytearray(original[:offset] + replacement + original[offset + 4:])
    boundary = cast(int, root['lightweight_end'])
    resolved = cast(dict[str, object], root['resolved'])
    assert struct.unpack_from('<i', original, boundary)[0] == resolved['size'], boundary
    struct.pack_into('<i', result, boundary, cast(int, resolved['size']) + len(replacement) - 4)
    assert result[:boundary] == original[:boundary] and result[boundary + 4:offset] == original[boundary + 4:offset], offset
    assert result[offset + len(replacement):] == original[offset + 4:], offset
    return bytes(result), [offset + base for base in bases], offset


def verify_configuration_dimension_values(root: dict[str, object], context: object, length: int) -> None:
    """Require original semantic roles, aliases and complete consumption; e.g. verify_configuration_dimension_values(root, context, length)."""
    resolved = cast(dict[str, object], root['resolved'])
    tail = cast(dict[str, object], resolved['part_tail'])
    entries = cast(list[dict[str, object]], tail['object_map'])
    assert [entry['key'] for entry in entries] == [-71, -72, -73], entries
    first, second, alias = [cast(dict[str, object], entry['value']) for entry in entries]
    assert first is alias and first is not second and tail['map_presence'] == 1, entries
    assert first['mask'] == -2147483618 and first['redundant_id'] == -2147483648, first
    assert first['enforce_bend_angle_range'] is True and first['fillet_ref_point_change_number'] == 7, first
    assert second['mask'] == 0 and second['redundant_id'] == 0, second
    assert second['enforce_bend_angle_range'] is False and second['fillet_ref_point_change_number'] == 0, second
    assert cast(int, getattr(context, 'p')) == length, length
    assert len(cast(list[object], getattr(context, 'map'))) == 100, entries


def capture_configuration_dimension_oracle(document: 'ReferenceDocument', raw: bytes, bases: list[int]) -> bytes:
    """Partition the complete configuration and verify all literal dimension words; e.g. capture_configuration_dimension_oracle(doc, raw, bases)."""
    from primitive_inventory import observe_primitives, verify_primitive_coverage
    archive = importlib.import_module('swarchive')
    selected = ReplacedDimensionConfiguration(document, raw)
    with observe_primitives(archive.Archive, raw) as reads:
        root, context = importlib.import_module('modelprefix').read(selected, configuration_context=True, through_part_resolved=True)
    verify_configuration_dimension_values(root, context, len(raw))
    physical = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, physical)
    expected = [(base + delta, base + delta + 4) for base in bases for delta in (0, 8, 12, 16, 20)]
    words = [read for read in physical if (read.start, read.end) in expected]
    assert [(read.start, read.end) for read in words] == expected and all(read.kind == 'i' for read in words), words
    return ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in physical).encode()


def verify_configuration_feature_companion(document: 'ReferenceDocument', raw: bytes) -> None:
    """Prove the companion archive completes with the new100-slot inheritance; e.g. verify_configuration_feature_companion(document, raw)."""
    selected = ReplacedDimensionConfiguration(document, raw)
    parsed = importlib.import_module('resolvedfeatures').read_supported(selected, 'Contents/Config-0-ResolvedFeatures')
    assert parsed['end'] == 10 and parsed['items'] == [], parsed['end']
    assert len(cast(list[object], getattr(parsed['archive'], 'map'))) == 100, parsed['end']


def verify_owned_dimension_configuration() -> None:
    """Keep a whole configuration carrier and independent oracle for the public configuration entry points; e.g. verify_owned_dimension_configuration()."""
    expected = json.loads((FEATURE / 'configuration-reader-sources.json').read_bytes())
    actual = hashlib.sha256((SOURCE / 'partresolved.py').read_bytes()).hexdigest()
    assert actual == expected['partresolved.py'], f'{actual}; expected pinned part-resolved suffix reader'
    document, contexts = owned_dimension_context()
    raw, bases, insertion = configuration_dimension_insertion(document)
    primitives = capture_configuration_dimension_oracle(document, raw, bases)
    verify_configuration_feature_companion(document, raw)
    destination = FEATURE.parents[2] / 'tests/fixtures/dimension-stored-state'
    for name, payload in (('configuration.bin', raw), ('configuration-primitives.tsv', primitives),
                          ('configuration-features.bin', CONFIGURATION_FEATURES)):
        lock_dimension_artifact(destination / name, payload)
    oracle = ''.join(f'{base + 16}\t{base + 20}\n' for base in bases).encode()
    lock_dimension_artifact(destination / 'configuration-dimensions.tsv', oracle)
    report = dict(context_streams_sha256=contexts, insertion=insertion, base_offsets=bases, wire_bytes=len(raw),
        sha256=hashlib.sha256(raw).hexdigest(), primitive_reads=len(primitives.splitlines()),
        primitives_sha256=hashlib.sha256(primitives).hexdigest(), original_slots=97, final_slots=100)
    lock_dimension_artifact(FEATURE / 'configuration-reference.json', (json.dumps(report, indent=2) + '\n').encode())
