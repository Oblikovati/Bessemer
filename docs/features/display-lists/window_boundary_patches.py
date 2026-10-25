"""Earlier environment, body-string and units members of complete display roots."""
from lower_range_patches import string_capture,tail_capture
from grouped_component_patches import convert as convert_grouped
from mfcobjects import ObjectArchive
import dl_doc


def environment_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        result = original(archive,schema)
        if version<0x1ff1:patches.append((archive.p-4,archive.p,b""))
        return result
    return read


def body_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, schema: int) -> object:
        saved,spans = archive.cstring,[]
        archive.cstring = string_capture(saved,archive,spans)
        try:result = original(archive,schema)
        finally:archive.cstring = saved
        if version<=0x2017:patches.append((*spans[-1],b""))
        return result
    return read


def unit_reader(original: object, version: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        if version<=0x1fb2:patches.append((start,archive.p,b""))
        return result
    return read


def convert(document: object, path: object, raw: bytes, version: int, edited: bool, patches: list[tuple[int,int,bytes]]) -> bytes:
    names = ["uiUserModelEnv_c","uoTempBodyTessData_c"]
    originals = [ObjectArchive.readers[name] for name in names]
    readers = [environment_reader(originals[0],version,patches),body_reader(originals[1],version,patches)]
    original_annotation = dl_doc.read_AnnotationViewTransform
    dl_doc.read_AnnotationViewTransform = tail_capture(original_annotation,version<=0x1fa8,8,patches)
    original_units = dl_doc.read_UnitsofMeasurement
    dl_doc.read_UnitsofMeasurement = unit_reader(original_units,version,patches)
    try:
        for name,reader in zip(names,readers,strict=True):ObjectArchive.readers[name] = reader
        return convert_grouped(document,path,raw,version,edited,patches)
    finally:
        dl_doc.read_AnnotationViewTransform = original_annotation
        dl_doc.read_UnitsofMeasurement = original_units
        for name,reader in zip(names,originals,strict=True):ObjectArchive.readers[name] = reader
