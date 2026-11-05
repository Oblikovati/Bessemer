"""Check frozen graph projections without installed Electrical application files."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import unittest
from pathlib import Path
from freeze_graphs import ROOT, graph_projection, owned_graphs, project_records, literal_archive


def graph_packets() -> list[tuple[str, bytes, bytes]]:
    raw = gzip.decompress((ROOT / "tests/fixtures/electrical-records/graphs.bin.gz").read_bytes())
    assert raw[:4] == b"EPR1"
    count, = struct.unpack_from("<I", raw, 4)
    offset = 8
    cases = []
    for _ in range(count):
        fields = []
        for _ in range(3):
            length, = struct.unpack_from("<I", raw, offset)
            offset += 4
            fields.append(raw[offset:offset+length])
            offset += length
        cases.append((fields[0].decode("utf-8"), fields[1], fields[2]))
    assert offset == len(raw)
    return cases


class ElectricalGraphFixtures(unittest.TestCase):
    def test_literal_slashes_and_backslashes_are_distinct_stored_names(self) -> None:
        entries = literal_archive(graph_packets()[1][1])["entries"]
        names = [entry["name"] for entry in entries]
        self.assertEqual(names.count("Projects/2/Drawings/same.ewg"), 2)
        self.assertEqual(names.count("Projects\\2\\Drawings\\same.ewg"), 1)

    def test_all_frozen_graphs_and_original_hashes(self) -> None:
        root = ROOT / "tests/fixtures/electrical-records"
        frozen = (root / "graphs.bin.gz").read_bytes()
        manifest = json.loads((root / "integrity.json").read_text(encoding="utf-8"))
        self.assertEqual(hashlib.sha256(frozen).hexdigest(), manifest["gzip_sha256"])
        self.assertEqual(hashlib.sha256(gzip.decompress(frozen)).hexdigest(), manifest["raw_sha256"])
        cases = graph_packets()
        self.assertEqual(len(cases), 7)
        for (name, raw, expected), record in zip(cases, manifest["cases"], strict=True):
            self.assertEqual(name, record["name"])
            self.assertEqual(hashlib.sha256(raw).hexdigest(), record["archive_sha256"])
            self.assertEqual(hashlib.sha256(expected).hexdigest(), record["projection_sha256"])
            self.assertEqual(graph_projection(raw), expected, name)

    def test_owned_graphs_rebuild_and_independent_join_denominators(self) -> None:
        cases = graph_packets()
        for (name, raw), (frozen_name, frozen, _) in zip(owned_graphs(), cases[:3], strict=True):
            self.assertEqual((name, raw), (frozen_name, frozen))
        projects = project_records(literal_archive(cases[1][1]))
        self.assertEqual(len(projects), 12)
        databases = [database for project in projects for database in project["databases"]]
        self.assertEqual(len(databases), 10)
        self.assertEqual(sum(len(database["files"]) for database in databases), 50)
        self.assertEqual(sum(len(link["member_indexes"]) for database in databases for link in database["files"]), 24)
        native = [project_records(literal_archive(raw)) for _, raw, _ in cases[3:]]
        self.assertEqual(sum(len(database["files"]) for projects in native for project in projects for database in project["databases"]), 17)


if __name__ == "__main__":
    unittest.main()
