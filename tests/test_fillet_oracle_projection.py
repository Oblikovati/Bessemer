"""Independent scalar projection keeps cached fillet boundaries and graph identity."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'docs/features/model-archives'))
from fillet_oracle_projection import project


class FilletProjectionTests(unittest.TestCase):
    def test_cached_suppression_remains_in_native_order(self) -> None:
        value = {'bodies': [{'suppressed': 195}, {'suppressed': 0, 'serialized_body': True}],
                 'face_edge_map': [(None, None), (None, None)], 'presence': -1}
        project(value)
        self.assertEqual(value, {'bodies': [{'suppressed': 195}, {'suppressed': 0}],
                                 'face_edge_map': [(None, None), (None, None)], 'presence': -1})

    def test_aliases_cycles_and_other_body_lists_remain_intact(self) -> None:
        child = {'bodies': [1, 2]}
        value = {'first': child, 'second': child}
        value['self'] = value
        project(value)
        self.assertIs(value['first'], value['second'])
        self.assertIs(value['self'], value)
        self.assertEqual(child, {'bodies': [1, 2]})
