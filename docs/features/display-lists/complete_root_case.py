"""Frame independently parsed complete roots without changing their physical context."""
import hashlib
import struct
from generate_unsigned_versions import traces
from generate_pre_facet_roots import scalars


def frame_case(version: int, kind: str, scenario: int, raw: bytes, identities: tuple[str,...], root: dict[str,object], extra_scalars: bytes = b"") -> tuple[bytes,dict[str,object]]:
    expected,scalar = traces(root,kind),scalars(root,kind)+extra_scalars
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw
    framed += struct.pack("<I",len(expected))+expected+struct.pack("<I",len(scalar))+scalar
    row = {"version":version,"kind":kind,"scenario":scenario,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(expected.splitlines())+len(scalar.splitlines())}
    if identities:row["external_ids"] = list(identities)
    return framed,row
