"""Independent kernel field oracles must preserve raw bits and graph identities."""
from pathlib import Path
import struct
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]/'docs/features/model-archives'))
from kernel_field_oracle import neutral_field_values
from oracle_projection import physical, prune_compound_kernel


class ModelKernelOracleTests(unittest.TestCase):
    def test_signed_integer_and_wide_code_units_keep_raw_values(self) -> None:
        self.assertEqual(neutral_field_values('d', struct.pack('>2i', -2**31, 2**31-1)), ['i:-2147483648', 'i:2147483647'])
        self.assertEqual(neutral_field_values('n', b'\x80\0\x7f\xff'), ['i:-32768', 'i:32767'])
        self.assertEqual(neutral_field_values('w', b'\xd8\0\xff\xff'), ['i:55296', 'i:65535'])

    def test_pointer_indices_keep_stored_adjustment_and_extended_boundaries(self) -> None:
        raw = struct.pack('>2HhH', 1, 32767, -4, 1)
        self.assertEqual(neutral_field_values('p', raw), ['i:1', 'i:32767', 'i:32771'])
        with self.assertRaisesRegex(ValueError, 'complete'):
            neutral_field_values('p', b'\xff\xff')

    def test_floating_bits_do_not_quiet_nan_or_normalize_negative_zero(self) -> None:
        raw = struct.pack('>2Q', 0x8000000000000000, 0x7ff000000000002a)
        self.assertEqual(neutral_field_values('i', raw), ['f:8000000000000000', 'f:7ff000000000002a'])

    def test_partition_vector_shapes_and_unknown_kinds_fail_explicitly(self) -> None:
        for kind, raw in [('v', bytes(8)), ('z', b''), ('q', bytes(8)), ('t', bytes(8))]:
            with self.assertRaisesRegex(ValueError, 'expected'):
                neutral_field_values(kind, raw)
        self.assertEqual(neutral_field_values('h', bytes(24)), ['f:0000000000000000']*3)
        self.assertEqual(neutral_field_values('l', b'\0\xff'), ['i:0', 'i:255'])

    def test_compound_projection_keeps_aliases_cycles_and_original_input(self) -> None:
        original = {'embedded_body': {'suppressed': 0, 'payload': b'encoded', 'decoded': {'records': 1}}, 'stored': 255}
        original['self'] = original
        projected = physical(original)
        prune_compound_kernel(projected, set())
        self.assertIs(projected['self'], projected)
        self.assertEqual(projected['embedded_body'], {'suppressed': 0})
        self.assertEqual(projected['stored'], 255)
        self.assertEqual(original['embedded_body']['payload'], b'encoded')

    def test_unrecognized_compound_shape_is_not_silently_dropped(self) -> None:
        with self.assertRaisesRegex(ValueError, 'expected a suppression'):
            prune_compound_kernel({'body_a8': {'different': 1}}, set())

    def test_spline_kernel_projection_preserves_suppression_and_original_nodes(self) -> None:
        original = {'kernel_body': {'suppressed': 0, 'nodes': [1, 2]}, 'stored': -1}
        projected = physical(original)
        prune_compound_kernel(projected, set())
        self.assertEqual(projected, {'kernel_body': {'suppressed': 0}, 'stored': -1})
        self.assertEqual(original['kernel_body']['nodes'], [1, 2])
        with self.assertRaisesRegex(ValueError, 'expected a suppression'):
            prune_compound_kernel({'kernel_body': {'nodes': []}}, set())


if __name__ == '__main__':
    unittest.main()
