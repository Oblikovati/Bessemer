"""Regression coverage for stored-field projection without alias duplication (#45)."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'docs/features/model-archives'))
from oracle_projection import physical


class CompleteDetailOracleTests(unittest.TestCase):
    def test_tolerance_aliases_remain_one_projected_body(self) -> None:
        tolerance = {'raw_08': 'tol', 'presence_1c': -1}
        projected = physical({'tolerances': [tolerance, tolerance]})
        self.assertIs(projected['tolerances'][0], projected['tolerances'][1])
        self.assertIsNot(projected['tolerances'][0], tolerance)

    def test_mfc_self_cycles_terminate_and_keep_identity(self) -> None:
        reference: dict[str, object] = {'kind': 4}
        reference['helper'] = reference
        projected = physical(reference)
        self.assertIs(projected['helper'], projected)
        self.assertEqual(projected['kind'], 4)

    def test_metadata_unwrapping_preserves_shared_helper_identity(self) -> None:
        helper = {'raw_10': -7}
        wrappers = [{'member': '2e8', 'kind': 'sf', 'data': helper}, {'member': '4d8', 'kind': 'note', 'data': helper}]
        projected = physical(wrappers)
        self.assertIs(projected[0], projected[1])
        self.assertEqual(projected, [{'raw_10': -7}, {'raw_10': -7}])

    def test_stored_kind_values_survive_while_constructor_labels_are_removed(self) -> None:
        source = {'member': '730', 'kind': 'weld', 'start': 10, 'end': 20, 'raw_08': 3, 'stored': {'kind': -7}}
        self.assertEqual(physical(source), {'raw_08': 3, 'stored': {'kind': -7}})

    def test_projection_isolated_between_calls_and_from_source_mutation(self) -> None:
        source = {'items': [{'raw_08': 3}]}
        first, second = physical(source), physical(source)
        source['items'][0]['raw_08'] = 9
        self.assertEqual(first['items'][0]['raw_08'], 3)
        self.assertIsNot(first, second)
        self.assertIsNot(first['items'], second['items'])

    def test_mutually_recursive_list_and_object_keep_their_cycle(self) -> None:
        items: list[object] = []
        root = {'items': items}
        items.append(root)
        projected = physical(root)
        self.assertIs(projected['items'][0], projected)


if __name__ == '__main__':
    unittest.main()
