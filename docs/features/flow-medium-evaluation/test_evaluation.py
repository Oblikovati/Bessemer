"""Independent field reconstruction, closed forms, exact rationals and arithmetic edge cases."""
import gzip
import math
import struct
import tempfile
import unittest
from fractions import Fraction
from pathlib import Path
import freeze_evaluation as frozen
from test_objects import rebuild_body
from swarchive import ArchiveError


def read(raw: bytes) -> dict:
    return frozen.cases.objects.schema.read_kind1_body(raw,string_mode=0)


def evaluate(raw: bytes, ordinal: int, query: int) -> float:
    return frozen.schema.evaluate_kind1_curve(read(raw),ordinal,frozen.oracle.number(query))


class MediumEvaluationBaselineTests(unittest.TestCase):
    def test_complete_expected_packet_regenerates_exactly(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);frozen.freeze(root)
            relative=Path('tests/fixtures/flow-medium-evaluation/curves.bin.gz')
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((frozen.ROOT/relative).read_bytes()))

    def test_every_input_body_rebuilds_all_fields_and_retains_unused_poison_patterns(self) -> None:
        for name,raw,_,_ in frozen.cases.cases():
            self.assertEqual(rebuild_body(read(raw)),raw,name)

    def test_exact_real_oracle_bounds_every_finite_measured_error_without_claiming_sequence_accuracy(self) -> None:
        for name,raw,ordinal,query_pattern in frozen.cases.cases():
            self.check_exact_oracle(name,raw,ordinal,query_pattern)

    def check_exact_oracle(self, name: str, raw: bytes, ordinal: int, query_pattern: int) -> None:
        try:
            value=evaluate(raw,ordinal,query_pattern)
        except ArchiveError:
            return
        curve=read(raw)['curves'][ordinal];query=frozen.oracle.number(query_pattern)
        _,indices=frozen.oracle.branch(curve,query)
        exact=frozen.oracle.exact_value(curve,query,indices)
        bound=frozen.oracle.error_bound(value,exact)
        if not math.isfinite(value):
            self.assertIsNone(bound,name);return
        error=abs(Fraction.from_float(value)-exact)
        self.assertGreaterEqual(Fraction.from_float(frozen.oracle.number(bound)),error,name)
        self.assertTrue(math.isfinite(float(exact)),name)

    def test_affine_tables_use_independent_closed_forms_and_exact_endpoint_values(self) -> None:
        for name,raw,ordinal,query in frozen.cases.tables():
            body=read(raw);curve=body['curves'][ordinal]
            coordinates=tuple(map(frozen.oracle.number,curve['first_patterns']))
            selected=min(max(frozen.oracle.number(query),coordinates[0]),coordinates[-1])
            self.assertEqual(evaluate(raw,ordinal,query),3*selected-5,name)

    def test_every_finite_constant_pattern_and_all_flag_bytes_are_preserved(self) -> None:
        for name,raw,ordinal,query in frozen.cases.constants():
            curve=read(raw)['curves'][ordinal]
            self.assertEqual(frozen.oracle.bits(evaluate(raw,ordinal,query)),curve['scalar_pattern'],name)
        flags=set()
        for name,raw,ordinal,query in frozen.cases.flags():
            flags.add(read(raw)['curves'][ordinal]['extra_flag_byte'])
            self.assertEqual(evaluate(raw,ordinal,query),0.,name)
        self.assertEqual(flags,set(range(256)))

    def test_invalid_accessors_queries_selected_numbers_and_nonincreasing_coordinates_fail(self) -> None:
        for name,raw,ordinal,query in frozen.cases.invalid_cases():
            with self.assertRaises(ArchiveError,msg=name):
                evaluate(raw,ordinal,query)
            self.assertEqual(frozen.outcome(raw,ordinal,query),b'\0')

    def test_finite_ordered_inputs_retain_overflow_nan_and_lost_accuracy_results(self) -> None:
        rows=frozen.cases.extremes()
        values=[evaluate(raw,ordinal,query) for _,raw,ordinal,query in rows[:8]]
        self.assertTrue(math.isinf(values[0]))
        self.assertTrue(math.isnan(values[1]))
        self.assertEqual(values[2],0.)
        self.assertTrue(math.isnan(values[3]))
        self.assertTrue(math.isinf(values[4]))
        self.assertEqual(frozen.oracle.bits(values[5]),0)
        self.assertEqual(values[6],0.)
        self.assertEqual(frozen.oracle.bits(values[7]),0)

    def test_barycentric_oracle_and_branch_selection_have_independent_exact_examples(self) -> None:
        row=frozen.cases.row('rational',2,(0.,3.),(1.,5.),1.)
        curve=read(row[1])['curves'][2]
        self.assertEqual(frozen.oracle.branch(curve,1.),(3,(0,1)))
        self.assertEqual(frozen.oracle.exact_value(curve,1.,(0,1)),Fraction(7,3))
        for query,expected in [(-1.,(1,(0,0))),(0.,(1,(0,0))),(3.,(2,(1,1))),(4.,(2,(1,1)))]:
            self.assertEqual(frozen.oracle.branch(curve,query),expected)
        self.assertEqual([frozen.oracle.classification(value) for value in (0.,math.inf,-math.inf,math.nan)],[0,1,2,3])
        self.assertEqual(frozen.oracle.error_bound(1.,Fraction(1)),frozen.oracle.bits(0.))
        self.assertIsNone(frozen.oracle.error_bound(math.inf,Fraction(1)))
        self.assertEqual(frozen.oracle.error_bound(-frozen.cases.MAX,Fraction.from_float(frozen.cases.MAX)),frozen.oracle.bits(math.inf))
