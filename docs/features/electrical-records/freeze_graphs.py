"""Freeze complete reference project graphs, including literal duplicate joins."""
from __future__ import annotations
import gzip
import hashlib
import io
import json
from pathlib import Path
import sqlite3
import struct
import sys
import warnings
import zipfile
from typing import TypedDict, cast

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT.parent / "inventor-interface/solidworks"))
from electrical import project_records, read
sys.path.insert(0, str(ROOT / "docs/features/sqlite-images"))
from freeze_fixtures import SqlValue, blob, text, value
sys.path.insert(0, str(ROOT / "docs/features/electrical-projects"))
from zip_reference import inspect_zip

StoredRecord = dict[str, SqlValue]
ENUM_SOURCE = ROOT.parent / "inventor-interface/solidworks/research/electrical-project-record-evidence.json"
FILE_CONSTANTS = {field["value"]: field["name"] for field in json.loads(ENUM_SOURCE.read_text(encoding="utf-8"))["enums"]["EwFileType"]["fields"]}


class FileGraph(TypedDict):
    record: StoredRecord
    member_name: str | None
    member_indexes: list[int]


class DatabaseGraph(TypedDict):
    entry_index: int
    files: list[FileGraph]
    locations: list[StoredRecord]
    functions: list[StoredRecord]
    bundles: list[StoredRecord]
    userdata: list[StoredRecord]
    versions: list[StoredRecord]


class ProjectGraph(TypedDict):
    catalog_index: int
    project: StoredRecord
    catalog_versions: list[StoredRecord]
    databases: list[DatabaseGraph]


def record(row: StoredRecord) -> bytes:
    return struct.pack("<I", len(row)) + b"".join(text(name) + value(cell) for name, cell in row.items())


def table(rows: list[StoredRecord]) -> bytes:
    return struct.pack("<I", len(rows)) + b"".join(record(row) for row in rows)


def file_link(link: FileGraph) -> bytes:
    indexes = link["member_indexes"]
    code = link["record"].get("fil_filetype")
    constant = FILE_CONSTANTS.get(code) if type(code) is int else None
    return record(link["record"]) + value(link["member_name"]) + struct.pack("<I", len(indexes)) + b"".join(struct.pack("<I", index) for index in indexes) + value(constant)


def database_graph(database: DatabaseGraph) -> bytes:
    encoded = struct.pack("<II", database["entry_index"], len(database["files"]))
    encoded += b"".join(file_link(link) for link in database["files"])
    return encoded + b"".join(table(database[key]) for key in ("locations", "functions", "bundles", "userdata", "versions"))


def project_graph(project: ProjectGraph) -> bytes:
    encoded = struct.pack("<I", project["catalog_index"]) + record(project["project"]) + table(project["catalog_versions"])
    return encoded + struct.pack("<I", len(project["databases"])) + b"".join(database_graph(database) for database in project["databases"])


def graph_projection(raw: bytes) -> bytes:
    projects = cast(list[ProjectGraph], project_records(literal_archive(raw)))
    return struct.pack("<I", len(projects)) + b"".join(project_graph(project) for project in projects)


def literal_archive(raw: bytes) -> dict[str, list[dict[str, str | bytes]]]:
    # #236: Python ZipInfo normalizes backslashes on Windows; retain stored names.
    end = len(raw) - 22
    assert raw[end:end+4] == b"PK\x05\x06" and raw[-2:] == b"\0\0"
    count, = struct.unpack_from("<H", raw, end+10)
    entries = inspect_zip(raw, b"", count)
    return dict(entries=[dict(name=entry.name, data=entry.payload) for entry in entries])


def sql_image(sql: str) -> bytes:
    with sqlite3.connect(":memory:") as connection:
        connection.executescript(sql)
        return connection.serialize()


def project_database() -> bytes:
    sql = "CREATE TABLE tew_file(fil_filename,fil_filetype INTEGER,unknown BLOB);"
    sql += "INSERT INTO tew_file VALUES('same.ewg',0,x'00ff'),('missing.ewg',999,NULL),(NULL,-1,x''),(17,20,NULL),('../literal.ewg',-2,NULL);"
    for name in ("tew_location", "tew_function", "tew_bundle", "tew_userdata"):
        sql += f"CREATE TABLE {name}(id INTEGER, unknown TEXT); INSERT INTO {name} VALUES(-7,'');"
    sql += "CREATE TABLE tew_version(ver_projectdata,ver_projectdatas,unknown); INSERT INTO tew_version VALUES(292,119,'local');"
    return sql_image(sql)


def catalog_database() -> bytes:
    return sql_image("CREATE TABLE tew_project(pro_id,pro_directory,pro_name,unknown); INSERT INTO tew_project VALUES(2,'2','one',NULL),(7,NULL,'null',''),(8,17,'numeric',x'00'),(9,'../raw/','literal',NULL),(10,'missing','absent',NULL),(2,'2','duplicate',NULL); CREATE TABLE tew_version(ver_projects,ver_projectsdatas,unknown); INSERT INTO tew_version VALUES(32,3,'catalog');")


def zip_graph(entries: list[tuple[str, bytes]]) -> bytes:
    output = io.BytesIO()
    with warnings.catch_warnings(), zipfile.ZipFile(output, "w") as archive:
        warnings.filterwarnings("ignore", message="Duplicate name:", category=UserWarning)
        for name, payload in entries:
            info = zipfile.ZipInfo("fixture", (2026, 1, 2, 3, 4, 6))
            info.filename = name
            info.orig_filename = name
            archive.writestr(info, payload)
    return output.getvalue()


def owned_graphs() -> list[tuple[str, bytes]]:
    catalog, database = catalog_database(), project_database()
    entries = [("Database/tew_app_project.sqlite", catalog)] * 2
    entries += [("Projects/2/Database/tew_project_data.sqlite", database)] * 2
    entries += [("Projects/2/Drawings/same.ewg", b"opaque")] * 2
    entries += [("Projects/2/Drawings/../literal.ewg", b"literal"), ("Projects\\2\\Drawings\\same.ewg", b"not a match")]
    entries += [("Projects/../raw//Database/tew_project_data.sqlite", database)]
    return [("owned-empty", zip_graph([])), ("owned-duplicates", zip_graph(entries)), ("owned-all-constants", enum_graph())]


def enum_graph() -> bytes:
    catalog = sql_image("CREATE TABLE tew_project(pro_directory); INSERT INTO tew_project VALUES('目录'); CREATE TABLE tew_version(v); INSERT INTO tew_version VALUES(17);")
    with sqlite3.connect(":memory:") as connection:
        connection.deserialize(project_database())
        connection.execute("DROP TABLE tew_file")
        connection.execute("CREATE TABLE tew_file(fil_filename,fil_filetype)")
        codes = list(FILE_CONSTANTS) + [-9223372036854775808, 9223372036854775807, None, "0", 0.5]
        connection.executemany("INSERT INTO tew_file VALUES(NULL,?)", [(code,) for code in codes])
        connection.commit()  # #237: freeze committed bytes, then verify the intended corpus.
        assert connection.execute("SELECT count(*) FROM tew_file").fetchone() == (28,)
        database = connection.serialize()
    with sqlite3.connect(":memory:") as verification:
        verification.deserialize(database)
        assert verification.execute("SELECT count(*) FROM tew_file").fetchone() == (28,)
    return zip_graph([("Database/tew_app_project.sqlite", catalog), ("Projects/目录/Database/tew_project_data.sqlite", database)])


def graph_cases() -> list[tuple[str, bytes]]:
    fixtures = ROOT / "tests/fixtures/electrical-projects"
    return owned_graphs() + [(path.name, path.read_bytes()) for path in sorted(fixtures.glob("*.tewzip"))]


def freeze_graphs() -> None:
    cases = graph_cases()
    packet = b"EPR1" + struct.pack("<I", len(cases))
    manifest = []
    for name, raw in cases:
        expected = graph_projection(raw)
        packet += text(name) + blob(raw) + blob(expected)
        manifest.append(dict(name=name, archive_sha256=hashlib.sha256(raw).hexdigest(), projection_sha256=hashlib.sha256(expected).hexdigest()))
    root = ROOT / "tests/fixtures/electrical-records"
    root.mkdir(parents=True, exist_ok=True)
    frozen = gzip.compress(packet, mtime=0)
    (root / "graphs.bin.gz").write_bytes(frozen)
    (root / "integrity.json").write_text(json.dumps(dict(cases=manifest, raw_bytes=len(packet), raw_sha256=hashlib.sha256(packet).hexdigest(), gzip_sha256=hashlib.sha256(frozen).hexdigest()), indent=2) + "\n", encoding="utf-8")
    print(dict(cases=len(cases), raw_bytes=len(packet), gzip_bytes=len(frozen)))


if __name__ == "__main__":
    freeze_graphs()
