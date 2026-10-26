"""Packet preparation preserves explicit context and refuses changed inputs."""
from pathlib import Path
import hashlib
import json
import struct
import tempfile
import unittest
from prepare_resolved_corpus import WORKSPACE, bind_case, prepare


class ResolvedCorpusPreparationTests(unittest.TestCase):
    def test_owned_packet_binds_every_context_stream(self) -> None:
        fixture = WORKSPACE / 'tests/fixtures/blank-part.SLDPRT'
        case = {'path': str(fixture), 'stream': 'Contents/Config-0', 'version': 19000,
                'stream_sha256': 'a changed hash'}
        reference = WORKSPACE.parent / 'inventor-interface/solidworks'
        with self.assertRaises(ValueError):
            bind_case(reference, case, 0)
        import swcontainer
        case['stream_sha256'] = hashlib.sha256(swcontainer.Document(fixture, table={}).read(case['stream'])).hexdigest()
        packet, row = bind_case(reference, case, 0)
        version, *sizes = struct.unpack('<6I', packet[:24])
        self.assertEqual(version, 19000)
        self.assertEqual(sum(sizes)+24, len(packet))
        self.assertEqual(len(row['sha256']), 4)

    def test_preparation_rejects_an_incomplete_case_manifest_before_writing(self) -> None:
        with tempfile.TemporaryDirectory(prefix='bessemer-resolved-corpus-') as temporary:
            root = Path(temporary)
            manifest = root / 'manifest.json'
            manifest.write_text(json.dumps({'records': []}))
            with self.assertRaises(ValueError):
                prepare(WORKSPACE.parent / 'inventor-interface/solidworks', manifest, root / 'output')
            self.assertFalse((root / 'output').exists())


if __name__ == '__main__':
    unittest.main()
