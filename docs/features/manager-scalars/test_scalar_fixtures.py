"""Regenerate bare scalar graphs and complete owned CMgr compositions (#54)."""
from pathlib import Path
import json,tempfile,unittest,struct,sys
from freeze_scalars import REFERENCE,WORKSPACE,freeze
from manager_tag_observation import observe_manager,relocated_tail


class ObservationDocument:
    def read(self, name: str) -> bytes:
        return b'fake manager wire'

    def names(self) -> list[str]:
        return ['Contents/CMgr']


class ObservationFailureManager:
    def read_supported(self, document: ObservationDocument) -> dict[str,object]:
        raise ValueError('named observation failure')


class ManagerScalarFixtureTests(unittest.TestCase):
    def test_tag_relocation_preserves_scalar_words_and_earlier_references(self) -> None:
        raw=struct.pack('<2Hi2H',2,0,12,12,0x800c)+struct.pack('<HIHIH',0x7fff,12,0x7fff,0x8000000c,0xffff)
        events=[(offset,20) for offset in (0,2,8,10,12,18,24)]
        expected=struct.pack('<2Hi2H',2,0,12,15,0x800f)+struct.pack('<HIHIH',0x7fff,15,0x7fff,0x8000000f,0xffff)
        self.assertEqual(relocated_tail(raw,events,2,8,3),expected)

    def test_failed_observation_restores_the_previous_profiler(self) -> None:
        previous=sys.getprofile();document=ObservationDocument()
        self.assertEqual(document.names(),['Contents/CMgr'])
        with self.assertRaisesRegex(ValueError,'named observation failure'):
            observe_manager(document,ObservationFailureManager(),ObservationDocument.read.__code__)
        self.assertIs(sys.getprofile(),previous)

    def test_complete_scalar_and_manager_fixtures_regenerate(self) -> None:
        """Retain every primitive observation, map insertion and complete document byte."""
        expected=json.loads(Path(__file__).with_name('reference.json').read_text())
        with tempfile.TemporaryDirectory() as temporary:
            destination=Path(temporary);actual=freeze(REFERENCE,destination)
            for key in ('cases','owned','reads','packet_sha256'):
                self.assertEqual(actual[key],expected[key])
            for fixture in destination.iterdir():
                self.assertEqual(fixture.read_bytes(),(WORKSPACE/'tests/fixtures/manager-scalars'/fixture.name).read_bytes())
