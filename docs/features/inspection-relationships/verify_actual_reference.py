"""Compare complete Rust relationship packets with the unchanged original helper (#197)."""
from pathlib import Path
from xml.dom import Node, minidom
import os
import struct
import unittest
import inspectionrelationships as relationships


class PacketReader:
    def __init__(self, payload: bytes) -> None:
        """Bind one complete packet; e.g. PacketReader(b'IRL1')."""
        self.payload = payload
        self.cursor = 0

    def take(self, length: int) -> bytes:
        """Read exactly length bytes; e.g. reader.take(4)."""
        end = self.cursor + length
        assert end <= len(self.payload), (end, len(self.payload))
        result = self.payload[self.cursor:end]
        self.cursor = end
        return result

    def number(self) -> int:
        """Read one little-endian count; e.g. reader.number()."""
        return struct.unpack('<Q', self.take(8))[0]

    def blob(self) -> bytes:
        """Read one length-delimited byte string; e.g. reader.blob()."""
        return self.take(self.number())

    def attempt(self) -> dict:
        """Decode every selected attempt field; e.g. reader.attempt()."""
        kind = self.take(1)[0]
        named = self.take(1)[0]
        assert named in (0, 1), named
        name = self.blob().decode('utf-8') if named else None
        key = self.blob().decode('utf-8')
        values = [self.blob().decode('utf-8') for _ in range(self.number())]
        return dict(kind=kind, name=name, key=key, values=values)

    def view(self) -> dict:
        """Decode one policy and complete output; e.g. reader.view()."""
        policy = self.take(1)[0]
        coverage = tuple(self.number() for _ in range(4))
        attempts = [self.attempt() for _ in range(self.number())]
        entries = [self.number() for _ in range(self.number())]
        writer = [self.number() for _ in range(self.number())]
        assert all(index < len(attempts) for index in entries + writer)
        return dict(policy=policy, coverage=coverage, attempts=attempts, entries=entries, writer=writer)


def packet_cases(payload: bytes) -> list:
    """Require complete framing and EOF; e.g. packet_cases(path.read_bytes())."""
    reader = PacketReader(payload)
    assert reader.take(4) == b'IRL1'
    cases = [(reader.blob(), [reader.view(), reader.view()]) for _ in range(reader.number())]
    assert reader.cursor == len(payload), (reader.cursor, len(payload))
    return cases


def expected_attempt(entry: dict) -> dict:
    """Project actual helper node identity into exported fields; e.g. expected_attempt(entry)."""
    node = entry['node']
    name = node.nodeName if node.nodeType == Node.ELEMENT_NODE else None
    return dict(kind=node.nodeType, name=name, key=entry['key'], values=entry['values'])


def selected_entries(view: dict, selection: str) -> list:
    """Resolve physical indices before comparing helper map values; e.g. selected_entries(view,'entries')."""
    return [dict(key=view['attempts'][index]['key'], values=view['attempts'][index]['values'])
            for index in view[selection]]


class ActualRelationshipReferenceTests(unittest.TestCase):
    def test_two_complete_process_packets_are_identical(self) -> None:
        """Require byte equality across two independently constructed complete Rust results."""
        first = Path(os.environ['BESSEMER_RELATIONSHIP_PACKET_1']).read_bytes()
        second = Path(os.environ['BESSEMER_RELATIONSHIP_PACKET_2']).read_bytes()
        self.assertEqual(first, second)
        self.assertEqual(len(packet_cases(first)), 12)

    def test_every_actual_attempt_map_and_writer_view_matches_original_source(self) -> None:
        """Compare all fields with original Source on a fresh explicit retained DOM."""
        payload = Path(os.environ['BESSEMER_RELATIONSHIP_PACKET_1']).read_bytes()
        for xml, views in packet_cases(payload):
            document = minidom.parseString(xml)
            packages = document.getElementsByTagName('PackageOptions')
            attempts = relationships.load_attempts(packages[0]) if packages else []
            for policy, view in enumerate(views):
                with self.subTest(xml=xml[:100], policy=policy):
                    selected = [entry for entry in attempts if policy == 0 or entry['node'].nodeType == Node.ELEMENT_NODE]
                    self.assertEqual(view['policy'], policy)
                    self.assertEqual(view['coverage'][0], len(xml))
                    self.assertEqual(view['coverage'][1], len(document.getElementsByTagName('*')))
                    self.assertEqual(view['coverage'][3], 0)
                    self.assertEqual(view['attempts'], list(map(expected_attempt, selected)))
                    expected = relationships.insert_first(selected)
                    self.assertEqual(selected_entries(view, 'entries'), expected)
                    self.assertEqual(selected_entries(view, 'writer'), relationships.writer_entries(expected))
