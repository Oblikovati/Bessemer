"""Complete old roots with raw line words, omitted pane strings and symbol-plane gates."""
import struct
import hashlib
from pathlib import Path
from legacy_pane_wire import root_bytes
from generate_unsigned_versions import reference,traces,compact_manifest
from generate_pre_facet_roots import scalars
from generate_earlier_range import write_group
from generate_earliest_components import write_documents
from functools import partial
import dl_doc

VERSIONS = [2999,0xbbd,0xbbe,0xbda,0xbdb,0xbf8,0xbf9]


def raw_line_reader(original: object) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        if archive.version>0xbf8:return result
        assert archive.p-start==52
        word, = struct.unpack_from("<i",archive.b,start+48)
        assert result["unknown_30"]==int(word!=0)
        result["unknown_30"] = word
        return result
    return read


def physical_reference(raw: bytes, version: int, kind: str, identities: tuple[str,...]) -> dict[str,object]:
    original = dl_doc.ITEM_TYPES[4]
    dl_doc.ITEM_TYPES[4] = (original[0],raw_line_reader(original[1]))
    try:return reference(raw,version,kind,identities)
    finally:dl_doc.ITEM_TYPES[4] = original


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw,identities = root_bytes(version,kind,scenario)
    root = physical_reference(raw,version,kind,identities)
    expected,scalar = traces(root,kind),scalars(root,kind)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw
    framed += struct.pack("<I",len(expected))+expected+struct.pack("<I",len(scalar))+scalar
    row = {"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(expected.splitlines())+len(scalar.splitlines())}
    if identities:row["external_ids"] = list(identities)
    return framed,row


if __name__=="__main__":
    cases = [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(3)]
    cases += [generate(v,"assembly",s) for v in VERSIONS for s in (3,4)]
    cases += [generate(2999,k,16+light) for k in ["part","assembly"] for light in range(5)]
    print(write_group("legacy-pane-roots",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("legacy-pane-roots.json"))
    write_documents(partial(generate,scenario=0),2999,"legacy-pane")
