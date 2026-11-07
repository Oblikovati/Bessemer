"""Root-owned item lists with independently checked framing and unchanged object identities."""
import struct
from legacy_curvature_wire import early_curvature_root
from legacy_component_reference import component_reference
from legacy_model_viz_wire import model_viz_object_reader
from generate_item_vectors import record
from generate_component_vectors import integers
from generate_transform_vectors import doubles
from mfcobjects import ObjectArchive
import dl_doc

VERSIONS = [767,768,772,773]


def item_span_reader(original: object, spans: list[tuple[int,int,object]]) -> object:
    def read(archive: object) -> object:
        first,before = archive.p,len(archive.map)
        result = original(archive)
        assert len(archive.map)==before
        spans.append((first,archive.p,result))
        return result
    return read


def view_start_reader(original: object, positions: list[int]) -> object:
    def read(archive: object) -> object:
        result = original(archive)
        positions.append(archive.p)
        return result
    return read


def observed_root_items(raw: bytes, kind: str, identities: tuple[str,...]) -> tuple[dict[str,object],list[tuple[int,int,object]],list[tuple[int,int,int,int,object]],int]:
    saved = dl_doc.read_display_item_list,ObjectArchive.object,dl_doc.read_AnnotationViewTransform
    lists,objects,views = [],[],[]
    dl_doc.read_display_item_list = item_span_reader(saved[0],lists)
    ObjectArchive.object = model_viz_object_reader(saved[1],objects)
    dl_doc.read_AnnotationViewTransform = view_start_reader(saved[2],views)
    try:root = component_reference(raw,773,kind,identities)
    finally:dl_doc.read_display_item_list,ObjectArchive.object,dl_doc.read_AnnotationViewTransform = saved
    assert len(root["views"])==1 and len(views)==1
    return root,lists,objects,views[0]


def selected_global_items(raw: bytes, first: int, last: int, scenario: int) -> bytes:
    if scenario in (164,165):return struct.pack("<HI",0xffff,0)
    if scenario not in (176,177):return raw[first:last]
    payload = doubles(3)+integers(23 if scenario==177 else -1,2)
    return struct.pack("<HI",0xffff,1)+record(1,payload)


def old_view_sequence(raw: bytes, start: int, end: int, objects: list[tuple[int,int,int,int,object]], root: dict[str,object], scenario: int) -> tuple[int,bytes]:
    if scenario in (160,161):return 0,b""
    first = raw[start:end]
    if scenario not in (162,163):return 1,first
    bucket = root["views"][0]["view_bucket"]
    tag = min((row for row in objects if row[4] is bucket),key=lambda row:row[0])
    schema, = struct.unpack_from("<H",raw,tag[0])
    assert schema==0xffff
    identity = tag[2]+1
    assert tag[3]>identity
    return 2,first+raw[start:tag[0]]+struct.pack("<H",identity)


def source_profile(scenario: int) -> int:
    if scenario in (176,177):return 0
    if scenario in (162,163):return 32+scenario%2
    return scenario%2 if scenario>=160 else scenario


def early_root_item_root(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    profile = source_profile(scenario)
    raw,identities = early_curvature_root(773,kind,profile)
    if version>=773:return raw,identities
    root,lists,objects,start = observed_root_items(raw,kind,identities)
    first,last,_ = next(row for row in lists if row[2] is root["views"][0]["display_items"])
    environment = min(row[0] for row in objects if row[4] is root["user_model_env"])
    assert environment==88 and last+4<=len(raw)
    items = selected_global_items(raw,first,last,scenario)
    count,views = old_view_sequence(raw,start,first,objects,root,scenario)
    prefix = raw[:4]+integers(count)+raw[8:environment]+items+raw[environment:start]
    tail = integers(0) if scenario in (160,161,162,163) else raw[last:]
    return prefix+views+tail,identities
