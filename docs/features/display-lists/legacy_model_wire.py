"""Complete component and part models before atom lists and paired metadata arrays."""
import struct
from legacy_environment_wire import environment_root
from legacy_edge_reference import edge_reference
from generate_version_range import apply_patches
from mfcobjects import ObjectArchive
import dl_tess

VERSIONS = [2130,2131,2139,2148,2149,2277]


def body_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        assert archive.b[archive.p-4:archive.p]==struct.pack("<i",result["unknown_2c"])
        patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def part_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        first,second = result["unknown_30"],result["unknown_48"]
        assert len(first)==len(second)
        length = 4+8*len(first)
        raw = archive.b[archive.p-length:archive.p]
        count, = struct.unpack_from("<i",raw)
        assert count==len(first) if first else count<=0
        assert raw[4:]==struct.pack("<"+"i"*(len(first)+len(second)),*first,*second)
        patches.append((archive.p-length,archive.p,b""))
        return result
    return read


def component_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        values = result["gobbled_up_atom_ids"]
        length = 4+4*len(values)
        raw = archive.b[archive.p-length:archive.p]
        count, = struct.unpack_from("<i",raw)
        assert count==len(values) if values else count<=0
        assert raw[4:]==struct.pack("<"+"i"*len(values),*values)
        patches.append((archive.p-length,archive.p,b""))
        return result
    return read


def model_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = environment_root(2277,kind,scenario)
    originals = ObjectArchive.readers["uoTempBodyTessData_c"],dl_tess.read_uoTempPartTessData,dl_tess.read_moViewerComponentData
    patches = []
    if version<=0x852:
        ObjectArchive.readers["uoTempBodyTessData_c"] = body_reader(originals[0],patches)
        dl_tess.read_uoTempPartTessData = part_reader(originals[1],patches)
    if version<=0x864:dl_tess.read_moViewerComponentData = component_reader(originals[2],patches)
    try:edge_reference(raw,2277,kind,identities)
    finally:ObjectArchive.readers["uoTempBodyTessData_c"],dl_tess.read_uoTempPartTessData,dl_tess.read_moViewerComponentData = originals
    return apply_patches(raw,patches),identities
