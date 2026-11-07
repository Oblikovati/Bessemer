"""Freeze original images and independent Python SQLite schema/row results."""
from __future__ import annotations

import gzip
import hashlib
import io
import json
from pathlib import Path
import sqlite3
import struct
import zipfile


class SqlText(bytes):
    """Retain SQLite TEXT bytes without conflating them with BLOB values."""


SqlValue = None | int | float | str | bytes
ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / "tests/fixtures/sqlite-images"


def blob(payload: bytes) -> bytes:
    return struct.pack("<I", len(payload)) + payload


def text(spelling: str) -> bytes:
    return blob(spelling.encode("utf-8"))


def value(payload: SqlValue) -> bytes:
    if payload is None:
        return b"\0"
    if isinstance(payload, int):
        return b"\1" + struct.pack("<q", payload)
    if isinstance(payload, float):
        return b"\2" + struct.pack("<d", payload)
    if isinstance(payload, str):
        return b"\3" + text(payload)
    tag = b"\3" if isinstance(payload, SqlText) else b"\4"
    return tag + blob(payload)


def quoted(spelling: str) -> str:
    return '"' + spelling.replace('"', '""') + '"'


def rows(connection: sqlite3.Connection, sql: str) -> bytes:
    cursor = connection.execute(sql)
    names = [column[0] for column in cursor.description]
    records = cursor.fetchall()
    encoded = struct.pack("<I", len(names)) + b"".join(text(name) for name in names)
    encoded += struct.pack("<I", len(records))
    return encoded + b"".join(value(cell) for record in records for cell in record)


def schema_object(connection: sqlite3.Connection, record: tuple[SqlValue, ...]) -> bytes:
    kind, name, table, root_page, sql = record
    assert isinstance(kind, bytes) and isinstance(name, bytes) and isinstance(table, bytes)
    assert isinstance(root_page, int)
    encoded = blob(kind) + blob(name) + blob(table) + struct.pack("<q", root_page) + value(sql)
    if kind != b"table":
        return encoded + b"\0"
    identifier = quoted(name.decode("utf-8"))
    return encoded + b"\1" + rows(connection, f"PRAGMA table_xinfo({identifier})") + rows(connection, f"PRAGMA foreign_key_list({identifier})") + rows(connection, f"SELECT * FROM {identifier}")


def projection(image: bytes) -> bytes:
    with sqlite3.connect(":memory:") as connection:
        connection.deserialize(image)
        connection.text_factory = SqlText
        connection.execute("PRAGMA query_only=ON")
        connection.execute("PRAGMA trusted_schema=OFF")
        assert connection.execute("PRAGMA integrity_check").fetchall() == [(b"ok",)]
        records = connection.execute("SELECT type,name,tbl_name,rootpage,sql FROM sqlite_schema ORDER BY type,name").fetchall()
        return struct.pack("<I", len(records)) + b"".join(schema_object(connection, record) for record in records)


def owned_image(sql: str) -> bytes:
    with sqlite3.connect(":memory:") as connection:
        connection.executescript(sql)
        connection.execute("VACUUM")
        return connection.serialize()


def owned_cases() -> list[tuple[str, bytes]]:
    sql = '''CREATE TABLE "odd"";DROP TABLE nope;--" ("na""me" INTEGER, r REAL, t TEXT, b BLOB, unknown);
    INSERT INTO "odd"";DROP TABLE nope;--" VALUES (-9223372036854775808,-1.25,'a'||char(0)||'b',x'00ff',NULL);
    INSERT INTO "odd"";DROP TABLE nope;--" VALUES (9223372036854775807,1e999,CAST(x'80ff00' AS TEXT),x'', '');
    INSERT INTO "odd"";DROP TABLE nope;--" SELECT * FROM "odd"";DROP TABLE nope;--";
    CREATE TABLE parent(id INTEGER PRIMARY KEY);
    CREATE TABLE child(id INTEGER, target INTEGER REFERENCES parent, doubled INTEGER GENERATED ALWAYS AS (id*2) VIRTUAL);
    INSERT INTO child(id,target) VALUES (3,99),(NULL,NULL);
    CREATE TABLE keyed(k TEXT PRIMARY KEY, v BLOB) WITHOUT ROWID;
    INSERT INTO keyed VALUES ('b',x'02'),('a',x'01');
    CREATE VIEW untouched_view AS SELECT * FROM child;
    CREATE TRIGGER untouched_trigger AFTER INSERT ON parent BEGIN SELECT 1; END;
    CREATE INDEX child_target ON child(target);
    CREATE TABLE many(v BLOB);'''
    sql += "INSERT INTO many VALUES (zeroblob(4096));" * 32
    cases = [("owned-empty", owned_image("CREATE TABLE gone(v); DROP TABLE gone;")), ("owned-mixed", owned_image(sql))]
    for page_size in (512, 1024, 65536):
        image = owned_image(f"PRAGMA page_size={page_size}; CREATE TABLE sized(v); INSERT INTO sized VALUES (17);")
        cases.append((f"owned-pages-{page_size}", image))
    return cases


def native_cases() -> list[tuple[str, bytes]]:
    unique: dict[str, tuple[str, bytes]] = {}
    for archive_path in sorted((ROOT / "tests/fixtures/electrical-projects").glob("*.tewzip")):
        with zipfile.ZipFile(io.BytesIO(archive_path.read_bytes())) as archive:
            for member in archive.infolist():
                payload = archive.read(member)
                if payload.startswith(b"SQLite format 3\0"):
                    digest = hashlib.sha256(payload).hexdigest()
                    unique.setdefault(digest, (archive_path.name + ":" + member.filename, payload))
    return [unique[digest] for digest in sorted(unique)]


def cam_case() -> tuple[str, bytes]:
    source = ROOT.parent / "inventor-interface/solidworks/research/electrical-cam-project-evidence.json"
    record = json.loads(source.read_text(encoding="utf-8"))["cam_databases"][0]
    payload = Path(record["path"]).read_bytes()
    assert len(payload) == record["size"]
    assert hashlib.sha256(payload).hexdigest() == record["sha256"]
    return "native-CAM-TechDB.cwdb", payload


def freeze() -> None:
    cases = owned_cases() + native_cases() + [cam_case()]
    packet = b"SQI1" + struct.pack("<I", len(cases))
    manifest: list[dict[str, str | int]] = []
    for name, image in cases:
        expected = projection(image)
        packet += text(name) + blob(image) + blob(expected)
        manifest.append(dict(name=name, image_bytes=len(image), image_sha256=hashlib.sha256(image).hexdigest(), projection_bytes=len(expected), projection_sha256=hashlib.sha256(expected).hexdigest()))
    FIXTURES.mkdir(parents=True, exist_ok=True)
    frozen = gzip.compress(packet, mtime=0)
    (FIXTURES / "images.bin.gz").write_bytes(frozen)
    evidence = dict(format="SQI1", cases=manifest, raw_bytes=len(packet), raw_sha256=hashlib.sha256(packet).hexdigest(), gzip_bytes=len(frozen), gzip_sha256=hashlib.sha256(frozen).hexdigest(), oracle=f"Python sqlite3 {sqlite3.sqlite_version}")
    (FIXTURES / "integrity.json").write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(dict(cases=len(cases), raw_bytes=len(packet), gzip_bytes=len(frozen))))


if __name__ == "__main__":
    freeze()
