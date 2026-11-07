"""Independent tangent-list omission and populated variants for explicit file context."""
from pathlib import Path
from generate_middle_buckets import complete_root
from generate_earlier_buckets import encoded as earlier_root
import bucket_vector_wire as wire


def tangent_lists(original: object, file_version: int) -> object:
    def encode(name: str, dimensions: int, empty: bool) -> bytes:
        if name not in ("tan_v","tan_h"):
            return original(name,dimensions,empty)
        if file_version<0x224:
            return b""
        if empty:
            return wire.poly_list([],True)
        return wire.poly_list([wire.poly(1,dimensions,9 if name=="tan_v" else 3,False,False)])
    return encode


def bucket_root(document: object, path: Path, raw: bytes, version: int, file_version: int, scenario: int) -> bytes:
    original = wire.main_list
    wire.main_list = tangent_lists(original,file_version)
    try:
        encoder = earlier_root if version<16000 else complete_root
        return encoder(document,path,raw,version,scenario)
    finally:
        wire.main_list = original


def entry_budget(version: int, file_version: int, scenario: int) -> int:
    if scenario==2:
        return 45
    result = 86 if scenario>=16 else 91
    result -= (version<=0x2faa)+2*(version<=0x2789)
    return result+((8 if scenario>=16 else 10) if file_version>=0x224 else 0)
