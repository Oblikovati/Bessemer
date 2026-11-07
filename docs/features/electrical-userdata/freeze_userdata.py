"""Freeze complete two-part SQL and lexical userdata expectations independently."""
from __future__ import annotations
import gzip
import hashlib
import io
import json
from pathlib import Path
import sqlite3
import struct
import sys
from typing import cast

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "docs/features/electrical-records"))
from freeze_graphs import literal_archive, zip_graph, sql_image
from electrical import userdata_by_key, userdata_for_object, userdata_id_transition, sqlite_rows
from freeze_fixtures import SqlValue, blob, text, value

Record = dict[str, SqlValue]
COLUMNS = ["use_objectid", "use_objectno"] + [f"use_data{index}" for index in range(20)]
OWNED_KEYS = [(-1,0),(0,0),(7,0),(7,-1),(26,-1),(26,0),(2147483647,-2147483648),(-2147483648,2147483647),(7,999)]


def encode_rows(records: list[Record], columns: list[str]) -> bytes:
    assert all(set(record) == set(columns) for record in records)
    header = struct.pack("<I",len(columns)) + b"".join(text(name) for name in columns)
    return header + struct.pack("<I",len(records)) + b"".join(value(record[name]) for record in records for name in columns)


def owned_userdata(affinity: str, keys: list[tuple[SqlValue, SqlValue, str]]) -> bytes:
    with sqlite3.connect(":memory:") as connection:
        declarations = ["use_objectid " + affinity, "use_objectno " + affinity] + COLUMNS[2:] + ["unknown"]
        connection.execute("CREATE TABLE tew_userdata(" + ",".join(declarations) + ")")
        for object_id, object_no, label in keys:
            cells = [object_id,object_no,label,None,b"\0\xff",1.25] + [""] * 16 + [b"opaque"]
            connection.execute("INSERT INTO tew_userdata VALUES(" + ",".join("?" for _ in cells) + ")", cells)
        for table in ("tew_file","tew_location","tew_function","tew_bundle","tew_version"):
            connection.execute(f"CREATE TABLE {table}(unknown)")
        connection.commit()
        assert connection.execute("SELECT count(*) FROM tew_userdata").fetchone() == (len(keys),)
        return connection.serialize()


def owned_cases() -> list[tuple[str,bytes,list[tuple[int,int]]]]:
    definitions = [("INTEGER",[(-1,0,"invalid-id"),(7,0,"first"),(7,0,"second"),(0,0,"zero"),(-2147483648,2147483647,"min"),(2147483647,-2147483648,"max"),("7","-1","coerced"),(None,0,"null")]),
        ("TEXT",[("26","-1","primary"),("26","0","secondary"),(None,"0","null"),("026","0","leading"),("26.0","0","decimal")]),
        ("",[(7,0,"integer"),(7.0,0.0,"real"),("7","0","text"),(b"7",b"0","blob"),(7,1,"other-number")])]
    definitions[0][1].extend((7,0,f"duplicate-{index}") for index in range(32))
    catalog = sql_image("CREATE TABLE tew_project(pro_directory); INSERT INTO tew_project VALUES('2'); CREATE TABLE tew_version(v);")
    return [("owned-" + (affinity or "NONE"), zip_graph([("Database/tew_app_project.sqlite",catalog), ("Projects/2/Database/tew_project_data.sqlite",owned_userdata(affinity,keys))]), OWNED_KEYS) for affinity, keys in definitions]


def native_cases() -> list[tuple[str,bytes,list[tuple[int,int]]]]:
    report = json.loads((ROOT.parent / "inventor-interface/solidworks/research/electrical-userdata-query-native-evidence.json").read_text(encoding="utf-8"))
    records = {record["sha256"]:record for record in report["archives"]}
    cases = []
    for path in sorted((ROOT / "tests/fixtures/electrical-projects").glob("*.tewzip")):
        raw = path.read_bytes()
        lookups = records[hashlib.sha256(raw).hexdigest()]["lookups"]
        keys = [(lookup["object_id"],lookup["object_no"]) for lookup in lookups]
        keys += [(object_id,999) for object_id in sorted({key[0] for key in keys})]
        cases.append((path.name,raw,keys))
    return cases


def case_projection(raw: bytes, keys: list[tuple[int,int]]) -> bytes:
    entries = literal_archive(raw)["entries"]
    database = next(entry["data"] for entry in entries if entry["name"] == "Projects/2/Database/tew_project_data.sqlite")
    assert isinstance(database,bytes)
    encoded = struct.pack("<I",len(keys))
    for object_id, object_no in keys:
        rows = cast(list[Record],userdata_by_key(database,object_id,object_no))
        encoded += struct.pack("<ii",object_id,object_no) + blob(encode_rows(rows,COLUMNS))
    records = cast(list[Record],sqlite_rows(database,"tew_userdata"))
    columns = list(records[0])
    ids = sorted({key[0] for key in keys})
    encoded += struct.pack("<I",len(ids))
    for object_id in ids:
        rows = cast(list[Record],userdata_for_object(records,object_id))
        encoded += struct.pack("<i",object_id) + blob(encode_rows(rows,columns))
    return encoded


def identity_cases() -> bytes:
    bounds = [-2147483648,-2147483647,-2,-1,0,1,7,2147483646,2147483647]
    encoded = struct.pack("<I",len(bounds)**2)
    for current_id in bounds:
        for loaded_id in bounds:
            next_id, success = userdata_id_transition(current_id,loaded_id)
            encoded += struct.pack("<iii?",current_id,loaded_id,next_id,success)
    return encoded


def freeze_userdata() -> None:
    cases = owned_cases() + native_cases()
    packet = b"EUQ1" + struct.pack("<I",len(cases))
    manifest = []
    for name, raw, keys in cases:
        expected = case_projection(raw,keys)
        packet += text(name) + blob(raw) + blob(expected)
        manifest.append(dict(name=name,queries=len(keys),archive_sha256=hashlib.sha256(raw).hexdigest(),projection_sha256=hashlib.sha256(expected).hexdigest()))
    packet += identity_cases()
    root = ROOT / "tests/fixtures/electrical-userdata"
    root.mkdir(parents=True,exist_ok=True)
    frozen = gzip.compress(packet,mtime=0)
    (root / "queries.bin.gz").write_bytes(frozen)
    (root / "integrity.json").write_text(json.dumps(dict(cases=manifest,identity_cases=81,raw_bytes=len(packet),raw_sha256=hashlib.sha256(packet).hexdigest(),gzip_sha256=hashlib.sha256(frozen).hexdigest()),indent=2) + "\n",encoding="utf-8")
    print(dict(cases=len(cases),queries=sum(len(keys) for _,_,keys in cases),raw_bytes=len(packet),gzip_bytes=len(frozen)))


if __name__ == "__main__":
    freeze_userdata()
