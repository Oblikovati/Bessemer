"""Complete unshaded roots with component properties in real bucket CObject keys."""
import struct
import hashlib
from generate_earliest_visuals import generate as shaded_root
from generate_shaded_vectors import ObjectBuilder,displaylists
from generate_earliest_scene_roots import raw_nan_reader
from component_property_wire import body,entries
from generate_component_vectors import integers
from generate_named_vectors import cstring
from generate_transform_vectors import compact
from swarchive import Archive
import bucket_corpus
import dl_doc
from earlier_bucket_wire import polygon_encoder


def bucket(builder: ObjectBuilder, version: int, scenario: int) -> bytes:
    class_name = "moComponentPropertyHolder" if scenario>=16 else "moComponentProperty"
    raw = bytes(24)+integers(0,0)+bytes(8)+integers(0,0)
    raw += bytes(11*4)+bytes(4)+integers(0)+bytes(2)
    raw += integers(1)+builder.object(class_name,lambda state:body(state,version,class_name,scenario%16))
    value = integers(1)+property_poly(builder,version) if version<0xf41 else bytes(4)
    raw += value+bytes(3*4)
    return raw+bytes(2*4)+bytes(8*4)+bytes(2)


def prefix(document: object, path: object, raw: bytes, version: int, kind: str) -> tuple[bytes,int]:
    original,slots = dl_doc.read_AnnotationViewTransform,[]
    def capture(archive: object) -> object:
        result = original(archive)
        slots.append((archive.p,len(archive.map)))
        return result
    dl_doc.read_AnnotationViewTransform = capture
    try:
        archive = displaylists.archive_for(document,str(path),raw)
        archive.version,archive.doc_kind,archive.face_tess_comp_ids = version,kind,set()
        displaylists.display_lists(archive,kind)
    finally:dl_doc.read_AnnotationViewTransform = original
    assert archive.p==len(raw) and len(slots)==1
    end,index = slots[0]
    return raw[:end],index


def reference(raw: bytes, document: object, path: object, version: int, kind: str) -> tuple[bytes,int]:
    original = Archive.f32
    Archive.f32 = raw_nan_reader(original)
    try:
        archive = displaylists.archive_for(document,str(path),raw)
        archive.version,archive.doc_kind = version,kind
        root = displaylists.display_lists(archive,kind)
    finally:Archive.f32 = original
    assert archive.p==len(raw) and root["has_shaded_data"]==0
    trace = bucket_corpus.trace(root)
    rows = [line.split(b"=",1) for line in trace.splitlines()]
    return b"".join(key.replace(b".weight.",b".")+b"="+value+b"\n" for key,value in rows),sum(archive.classes.values())


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    from middle_range_source import source
    framed,row = shaded_root(version,kind)
    path,document,_ = source(kind)
    start,index = prefix(document,path,framed[10:10+row["bytes"]],version,kind)
    builder = ObjectBuilder(index)
    view = integers(-1)+compact(0,0)+compact(0,0)+cstring("property bucket")+integers(1,0,0)
    raw = start+view+builder.object("uiViewBucket_c",lambda state:bucket(state,version,scenario))+bytes(2)+integers(0)
    scalars,objects = reference(raw,document,path,version,kind)
    class_name = "moComponentPropertyHolder" if scenario>=16 else "moComponentProperty"
    # Forty prefix fonts, three panes, one view and one thread-map entry precede the property budget.
    row = {"version":version,"kind":kind,"scenario":scenario,"class_name":class_name,"bytes":len(raw),"sha256":hashlib.sha256(raw).hexdigest(),"oracle_fields":len(scalars.splitlines()),"aggregate_entries":45+entries(version,class_name,scenario%16)+4*int(version<0xf41),"objects":objects}
    expected,scalars = (scalars,b"") if version<0xf41 else (b"",scalars)
    framed = struct.pack("<IBBI",version,kind=="assembly",scenario,len(raw))+raw+struct.pack("<I",len(expected))+expected+struct.pack("<I",len(scalars))+scalars
    return framed,row



def property_poly(builder: ObjectBuilder, version: int) -> bytes:
    raw = polygon_encoder(builder,version)(1,3,kind=9,arc=False,line=False)
    line = cstring("legacy polygon")+struct.pack("<iii",-2147483648,-4,5)
    return raw[:28]+bytes([255])+line+raw[29:]
