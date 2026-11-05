"""Check enclosing-map reservations without production reader assumptions (#211)."""
import struct
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "docs/features/sweeps"))
from sweep_fixture_bytes import SweepOptions, sweep_object
from fixture_object_map import WireMap, object_tag
from twist_pointer_sequence import TWIST_CLASS, TwistPointerSequence


class TwistPointerSequenceTests(unittest.TestCase):
    def test_fresh_shared_and_cached_class_have_independent_literal_tags(self) -> None:
        first, second = struct.pack('<BQQ', 255, 0, 1), struct.pack('<BQQ', 128, 2, 3)
        for style, end, slots in [('fresh', 100, [98, 100]), ('shared', 98, [98]),
                                  ('cached-class', 99, [98, 99])]:
            mapping = WireMap(); mapping.slot = 96
            sequence = TwistPointerSequence(style, (first, second))
            self.assertEqual(sequence(mapping), object_tag(TWIST_CLASS, 1) + first)
            expected = {'fresh': object_tag(TWIST_CLASS, 1) + second,
                        'shared': struct.pack('<H', 98),
                        'cached-class': struct.pack('<H', 0x8000 | 97) + second}
            self.assertEqual(sequence(mapping), expected[style])
            self.assertEqual((mapping.slot, sequence.object_slots), (end, slots))

    def test_complete_sweeps_embed_the_declared_body_and_reserve_two_slots(self) -> None:
        payload = struct.pack('<BQQ', 255, 0x8000000000000000, 0x7FF8000000001234)
        for version in (0x27CC, 0x27D9, 0x27DA, 19000):
            for name in ('moSweep_c', 'moSweepCut_c'):
                for rich in (False, True):
                    baseline, old_slots = sweep_object(version, rich, SweepOptions(), name=name)
                    sequence = TwistPointerSequence('fresh', (payload,))
                    changed, new_slots = sweep_object(version, rich, SweepOptions(), name=name, twist=sequence)
                    encoded = object_tag(TWIST_CLASS, 1) + payload
                    self.assertEqual(changed.count(encoded), 1)
                    self.assertEqual(new_slots, old_slots + 2)
                    self.assertEqual(len(changed), len(baseline) + len(encoded) - 2)
                    self.assertEqual(sequence.position, 1)

    def test_invalid_plans_fail_before_mutating_either_identity_state(self) -> None:
        for style, payloads in [('invalid', (bytes(17),)), ('fresh', ()),
                                ('shared', (bytes(16),)), ('cached-class', (bytes(18),))]:
            mapping = WireMap()
            sequence = TwistPointerSequence(style, payloads)
            with self.assertRaisesRegex(ValueError, 'expected'):
                sequence(mapping)
            self.assertEqual((mapping.slot, sequence.position, sequence.object_slots), (0, 0, []))

    def test_separate_operations_do_not_share_cached_identities(self) -> None:
        sequences = [TwistPointerSequence('cached-class', (bytes(17),)) for _ in range(2)]
        for sequence in sequences:
            self.assertEqual(sequence(WireMap()), object_tag(TWIST_CLASS, 1) + bytes(17))
            self.assertEqual(sequence.object_slots, [2])
        self.assertIsNot(sequences[0].object_slots, sequences[1].object_slots)


if __name__ == '__main__':
    unittest.main()
