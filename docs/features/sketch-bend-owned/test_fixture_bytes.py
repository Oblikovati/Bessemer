"""Reproduce complete canonical-profile graphs and preserve prior default builders (#219)."""
from pathlib import Path
import json
import hashlib
import unittest
from fixture_bytes import ROOT, owned_cases, verify_owned_cases
from sketch_bend_bytes import sketch_bend_object


class OwnedSketchFixtureTests(unittest.TestCase):
    def test_all_owned_graphs_and_edits_reproduce(self) -> None:
        """Check frozen bytes without the Bessemer encoder; e.g. python -m unittest."""
        verify_owned_cases()
        self.assertEqual(len(owned_cases()), 23)

    def test_all_198_old_sketch_bend_graphs_keep_default_kind_and_framing(self) -> None:
        """Retain the pre-profile default behavior; e.g. python -m unittest."""
        ref = json.loads((ROOT / 'docs/features/sketch-bends/reference.json').read_text())
        self.assertEqual(len(ref['cases']), 198)
        for case in ref['cases']:
            kind, style = divmod(case['kind'], 6)
            raw, slots = sketch_bend_object(kind, case['version'], style)
            self.assertEqual(hashlib.sha256(raw).hexdigest(), case['sha256'])
            self.assertEqual(slots, case['slots'])


if __name__ == '__main__':
    unittest.main()
