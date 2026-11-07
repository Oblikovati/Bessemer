"""Observe old short widths without changing flag-forced polygon line properties."""
from earlier_environment_patches import convert as convert_environment
import dl_ui


def width_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, restore_type: int, flag: int = 0) -> object:
        start = archive.p
        result = original(archive,restore_type,flag)
        if version<0xf41 and flag==0:
            assert restore_type==2 and archive.p==start+6
            patches.append((start+2,archive.p,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    original = dl_ui.read_utLineWidth_c
    dl_ui.read_utLineWidth_c = width_reader(original,version,patches)
    try:return convert_environment(document,path,raw,version,edited,patches)
    finally:dl_ui.read_utLineWidth_c = original
