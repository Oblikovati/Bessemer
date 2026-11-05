"""Owned logical expectations for exact SQLite userdata TEXT assignment."""
from __future__ import annotations
import gzip
import hashlib
import json
from pathlib import Path
import sqlite3
import struct
import sys
from typing import TypedDict

ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT / "docs/features/electrical-records"))
from freeze_graphs import zip_graph
from freeze_fixtures import SqlText,blob,text,value


class WriteCase(TypedDict):
    name:str
    image:bytes
    replacement:str
    success:bool


def fixture_image(variant:str)->bytes:
    with sqlite3.connect(":memory:") as database:
        database.execute("PRAGMA page_size=1024")
        database.execute(userdata_schema(variant))
        insert_fixture_rows(database,variant)
        add_fixture_relations(database,variant)
        database.commit()
        return database.serialize()


def userdata_schema(variant:str)->str:
    affinity="TEXT" if variant=="text-key" else "INTEGER"
    columns=["use_objectid "+affinity,"use_objectno "+affinity]
    columns += [f"use_data{index} "+("INTEGER" if variant=="integer-slot" and index==0 else "TEXT") for index in range(20)]
    columns += ["unknown BLOB"]
    if variant=="check":columns += ["CHECK(use_data0 != 'forbidden')"]
    if variant=="cascade":columns += ["UNIQUE(use_data0)"]
    if variant=="generated":columns += ["derived TEXT GENERATED ALWAYS AS (use_data0 || '!') STORED"]
    if variant=="without-rowid":columns += ["PRIMARY KEY(use_objectid,use_objectno)"]
    return "CREATE TABLE tew_userdata("+",".join(columns)+")" + (" WITHOUT ROWID" if variant=="without-rowid" else "")


def insert_fixture_rows(database:sqlite3.Connection,variant:str)->None:
    first=None if variant=="null" else (b"\0\xff" if variant=="blob" else "old")
    row=[7,0,first]+[""]*19+[b"opaque\0\xff"]
    names=["use_objectid","use_objectno"]+[f"use_data{i}" for i in range(20)]+["unknown"]
    sql="INSERT INTO tew_userdata("+",".join(names)+") VALUES("+",".join("?" for _ in row)+")"
    database.execute(sql,row)
    if variant=="duplicate":database.execute(sql,row)


def add_fixture_relations(database:sqlite3.Connection,variant:str)->None:
    database.execute('CREATE TABLE "odd""table"(unknown BLOB, nullable)')
    database.execute('INSERT INTO "odd""table" VALUES(?,?)',(b"untouched\0\xff",None))
    database.execute("CREATE VIEW unchanged_view AS SELECT * FROM tew_userdata")
    if variant=="trigger":database.execute("CREATE TRIGGER side_effect AFTER UPDATE ON tew_userdata BEGIN DELETE FROM 'odd\"table'; END")
    if variant=="generated":return
    if variant=="virtual":database.execute("CREATE VIRTUAL TABLE virtual_data USING fts5(content)")
    if variant=="foreign-key":
        database.execute("CREATE TABLE parent(id INTEGER PRIMARY KEY)")
        database.execute("CREATE TABLE child(id REFERENCES parent)")
        database.execute("INSERT INTO child VALUES(99)")
    if variant=="cascade":
        database.execute("CREATE TABLE child(v TEXT REFERENCES tew_userdata(use_data0) ON UPDATE CASCADE)")
        database.execute("INSERT INTO child VALUES('old')")


def logical_snapshot(image:bytes)->bytes:
    with sqlite3.connect(":memory:") as database:
        database.deserialize(image);database.text_factory=SqlText
        database.execute("PRAGMA query_only=ON");database.execute("PRAGMA trusted_schema=OFF")
        tables=database.execute("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").fetchall()
        encoded=struct.pack("<I",len(tables))
        for (name,) in tables:
            spelling=name.decode("utf-8");identifier='"'+spelling.replace('"','""')+'"'
            cursor=database.execute("SELECT * FROM "+identifier)
            names=[column[0] for column in cursor.description];records=cursor.fetchall()
            encoded += text(spelling)+struct.pack("<I",len(names))+b"".join(text(column) for column in names)+struct.pack("<I",len(records))
            encoded += b"".join(value(cell) for row in records for cell in row)
        return encoded


def expected_image(image:bytes,replacement:str)->bytes:
    with sqlite3.connect(":memory:") as database:
        database.deserialize(image)
        database.execute("PRAGMA foreign_keys=ON")
        database.execute("UPDATE tew_userdata SET use_data0=? WHERE use_objectid=7 AND use_objectno=0",(replacement,))
        database.commit()
        return database.serialize()


def cases()->list[WriteCase]:
    accepted=["ordinary","text-key","null","blob","without-rowid"]
    result=[WriteCase(name=name,image=fixture_image(name),replacement="new\0µ",success=True) for name in accepted]
    result += [WriteCase(name="no-op",image=fixture_image("ordinary"),replacement="old",success=True)]
    result += [WriteCase(name="growth",image=fixture_image("ordinary"),replacement="µ"*4096,success=True)]
    rejected=[("duplicate","new"),("integer-slot","17"),("check","forbidden"),("trigger","new"),("generated","new"),("virtual","new"),("foreign-key","new"),("cascade","new")]
    result += [WriteCase(name=name,image=fixture_image(name),replacement=replacement,success=False) for name,replacement in rejected]
    return result


def freeze()->None:
    entries=cases();packet=b"EUW1"+struct.pack("<I",len(entries));manifest=[]
    for entry in entries:
        expected=logical_snapshot(expected_image(entry["image"],entry["replacement"])) if entry["success"] else b""
        archive=zip_graph([("Database/tew_project_data.sqlite",entry["image"]),("opaque.bin",b"untouched\0\xff")])
        packet += text(entry["name"])+blob(archive)+text(entry["replacement"])+bytes([entry["success"]])+blob(expected)
        manifest.append(dict(name=entry["name"],success=entry["success"],archive_sha256=hashlib.sha256(archive).hexdigest(),expected_sha256=hashlib.sha256(expected).hexdigest()))
    root=ROOT / "tests/fixtures/electrical-userdata-write";root.mkdir(parents=True,exist_ok=True)
    frozen=gzip.compress(packet,mtime=0);(root / "owned.bin.gz").write_bytes(frozen)
    (root / "integrity.json").write_text(json.dumps(dict(cases=manifest,raw_bytes=len(packet),raw_sha256=hashlib.sha256(packet).hexdigest(),gzip_sha256=hashlib.sha256(frozen).hexdigest()),indent=2)+"\n",encoding="utf-8")
    print(dict(cases=len(entries),raw_bytes=len(packet),gzip_bytes=len(frozen)))


if __name__=="__main__":freeze()
