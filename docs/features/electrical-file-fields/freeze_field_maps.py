"""Freeze documented native descriptor requests with independent SQLite rows."""
from __future__ import annotations
import gzip
import hashlib
import json
from pathlib import Path
import sqlite3
import struct
import sys
from typing import TypedDict, cast

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks/research/electrical-file-field-native-evidence.json"
sys.path.insert(0,str(ROOT / "docs/features/electrical-records"))
from freeze_graphs import literal_archive, project_records, sql_image, zip_graph
from freeze_fixtures import SqlValue, blob, text, value


class Field(TypedDict):
    name: str
    kind: str
    destination: str


class FieldReport(TypedDict):
    fields: list[Field]


FIELDS = cast(FieldReport,json.loads(SOURCE.read_text(encoding="utf-8")))["fields"]
KINDS = {"Long":1,"Bool":2,"Double":3,"Str":4,"Date":5}


def destination(expression: str) -> bytes:
    temporary = {"auStackX_20":1,"auStackX_18":2,"auStackX_8":3,"alStack_28":4}
    if expression in temporary:
        return bytes([temporary[expression]])
    if expression.startswith("(longlong)param_1 + "):
        offset = int(expression.split(" + ")[1],0)
    else:
        assert expression.startswith("param_1 + ")
        offset = 8 * int(expression.split(" + ")[1],0)
    return b"\0" + struct.pack("<I",offset)


def encode_record(record: dict[str,SqlValue]) -> bytes:
    return struct.pack("<I",len(record)) + b"".join(text(name) + value(cell) for name,cell in record.items())


def file_projection(record: dict[str,SqlValue]) -> bytes:
    encoded = encode_record(record) + struct.pack("<I",len(FIELDS))
    for field in FIELDS:
        encoded += text(field["name"]) + bytes([KINDS[field["kind"]]]) + destination(field["destination"]) + value(record[field["name"]])
    unmapped = {name:cell for name,cell in record.items() if name not in {field["name"] for field in FIELDS}}
    return encoded + encode_record(unmapped)


def files(raw: bytes) -> list[dict[str,SqlValue]]:
    projects = project_records(literal_archive(raw))
    return [link["record"] for project in projects for database in project["databases"] for link in database["files"]]


def projection(raw: bytes) -> tuple[str | None, bytes]:
    records = files(raw)
    for record in records:
        for field in FIELDS:
            if field["name"] not in record:
                return field["name"],b""
    encoded = struct.pack("<I",len(records)) + b"".join(blob(file_projection(record)) for record in records)
    return None,encoded


def owned_archive(omitted: str | None, all_null: bool) -> bytes:
    columns = [field["name"] for field in reversed(FIELDS) if field["name"]!=omitted] + ["fil_state","fil_updatedate","future_µ"]
    cells = [owned_cell(index,all_null) for index in range(len(columns))]
    names = ",".join('"' + name + '"' for name in columns)
    with sqlite3.connect(":memory:") as connection:
        connection.execute(f"CREATE TABLE tew_file({names})")
        connection.execute("INSERT INTO tew_file VALUES(" + ",".join("?" for _ in cells) + ")",cells)
        for table in ("tew_location","tew_function","tew_bundle","tew_userdata","tew_version"):
            connection.execute(f"CREATE TABLE {table}(unknown)")
        connection.commit()
        database = connection.serialize()
    catalog = sql_image("CREATE TABLE tew_project(pro_directory); INSERT INTO tew_project VALUES('2'); CREATE TABLE tew_version(v);")
    return zip_graph([("Database/tew_app_project.sqlite",catalog),("Projects/2/Database/tew_project_data.sqlite",database)])


def owned_cell(index: int, all_null: bool) -> SqlValue:
    if all_null:
        return None
    return [-9223372036854775808,-1.25,"raw\0µ",b"\0\xff",None][index % 5]


def cases() -> list[tuple[str,bytes]]:
    owned = [("owned-mixed",owned_archive(None,False)),("owned-null",owned_archive(None,True))]
    owned += [("missing-" + field["name"],owned_archive(field["name"],False)) for field in FIELDS]
    native = [(path.name,path.read_bytes()) for path in sorted((ROOT / "tests/fixtures/electrical-projects").glob("*.tewzip"))]
    return owned + native


def freeze() -> None:
    fixtures = cases()
    assert len(FIELDS)==len({field["name"] for field in FIELDS})==38
    packet = b"EFF1" + struct.pack("<I",len(fixtures))
    manifest = []
    for name,raw in fixtures:
        missing,expected = projection(raw)
        packet += text(name) + blob(raw) + text(missing or "") + blob(expected)
        manifest.append(dict(name=name,archive_sha256=hashlib.sha256(raw).hexdigest(),missing=missing,projection_sha256=hashlib.sha256(expected).hexdigest()))
    root = ROOT / "tests/fixtures/electrical-file-fields"
    root.mkdir(parents=True,exist_ok=True)
    frozen = gzip.compress(packet,mtime=0)
    (root / "maps.bin.gz").write_bytes(frozen)
    report = dict(cases=manifest,fields_sha256=hashlib.sha256(SOURCE.read_bytes()).hexdigest(),raw_bytes=len(packet),raw_sha256=hashlib.sha256(packet).hexdigest(),gzip_sha256=hashlib.sha256(frozen).hexdigest())
    (root / "integrity.json").write_text(json.dumps(report,indent=2) + "\n",encoding="utf-8")
    print(dict(cases=len(fixtures),raw_bytes=len(packet),gzip_bytes=len(frozen)))


if __name__=="__main__":
    freeze()
