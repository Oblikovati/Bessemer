"""Independent complete multi-parameter edit goldens (#202)."""
from pathlib import Path
import copy
import hashlib
import json
import os
import struct
import unittest
from parameter_bytes import parameter_graph, parameter_value
from verify_parameter_exports import observe_parameters

ROOT = Path(__file__).resolve().parents[3]
STYLES = (1, 2, 3, 1, 2, 3)


def multiple_parameter_graph(edited: int | None = None) -> bytes:
    """Reserve every preceding class/name identity before the next root; e.g. multiple_parameter_graph(3)."""
    roots, slot = [], 96
    for kind, style in enumerate(STYLES):
        raw, following = parameter_graph(kind, style, kind == edited, slot)
        roots.append(raw[6:])
        slot = following - 1
    return struct.pack('<iH', -123, 6) + b''.join(roots)


class MultipleParameterReferenceTests(unittest.TestCase):
    def test_each_primary_change_preserves_all_other_physical_parameters(self) -> None:
        """Compare seven complete graphs through unchanged original readers."""
        import swcontainer
        from sheet_owned_fixtures import SavedSheetMetalDocument
        document = swcontainer.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
        manager = (ROOT / 'tests/fixtures/sheet-metal/manager-0.bin').read_bytes()
        original = multiple_parameter_graph()
        _, rows = observe_parameters(SavedSheetMetalDocument(document, manager, original), 'Contents/Config-0-ResolvedFeatures')
        self.assertEqual([row['kind'] for row in rows], list(range(6)))
        self.assertEqual(len({row['definition'][3] for row in rows}), 6)
        fixtures = {'original.bin': original}
        for kind in range(6):
            changed = multiple_parameter_graph(kind)
            expected = copy.deepcopy(rows)
            expected[kind]['primary'] = self.edited_primary(kind)
            _, actual = observe_parameters(SavedSheetMetalDocument(document, manager, changed), 'Contents/Config-0-ResolvedFeatures')
            self.assertEqual(actual, expected)
            fixtures[f'edited-{kind}.bin'] = changed
        self.retain_fixtures(fixtures)

    def edited_primary(self, kind: int) -> tuple[object, ...]:
        """Use independently assembled primitive images; e.g. edited_primary(5)."""
        if kind == 5:
            return 2, 'updated', parameter_value(kind, STYLES[kind], True)
        if kind == 3:
            return 1, struct.pack('<i', 12345)
        return 0, struct.pack('<Q', 0x3ff4000000000000)

    def retain_fixtures(self, fixtures: dict[str, bytes]) -> None:
        """Retain exact independently parsed full images; e.g. retain_fixtures(fixtures)."""
        destination = Path(os.environ['BESSEMER_PARAMETER_FIXTURES'])
        destination.mkdir(parents=True, exist_ok=True)
        manifest = []
        for name, raw in fixtures.items():
            (destination / name).write_bytes(raw)
            manifest.append(dict(file=name, bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest()))
        (destination / 'manifest.json').write_bytes((json.dumps(manifest, indent=2) + '\n').encode())
