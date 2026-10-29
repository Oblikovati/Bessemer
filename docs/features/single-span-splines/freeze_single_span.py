"""Complete exact-mask single-span collections through the actual reader (#54)."""
from pathlib import Path
import struct,sys,json,hashlib
ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'docs/features/model-archives'))
sys.path.insert(0,str(ROOT/'docs/features/trim-edge-identifiers'))
from freeze_trim_edges import ObjectFixtureFamily,owned,REFERENCE
from fillet_fixture_bytes import WireMap
from sketch_tail_fixture_bytes import ordinary
from sketch_spline_fixture_bytes import LISTS,collection,prefix,arrays
from spline_array_fixture_bytes import interpolation,secondary
from freeze_views import view_oracle
from oracle_projection import prune_compound_kernel
from mfcobjects import ObjectArchive
import sketchprefix


def single_span_wire(name: str | None, rich: bool, flags: int, mode: int, extra: bool) -> bytes:
    """Insert only the true-callback secondary record; e.g. single_span_wire(None,False,1,0,True)."""
    raw=collection(name,rich,flags,mode,extra,mask=0x80)
    if extra and not (rich and not flags&1):
        at=6+len(prefix(rich,flags,name,None,name is None))+len(arrays(rich,flags))+len(interpolation(mode))+8
        raw=raw[:at]+secondary(0 if flags&1 else 3)+raw[at:]
    return raw


def observe(version: int, raw: bytes) -> tuple[bytes,int]:
    """Retain all stored fields and require exact EOF without prototype stubs."""
    source=ObjectArchive(raw,version);source.feature_context='ordinary_external_xml';source.single_span_splines=True
    sketchprefix.install_scoped_readers(source);out=sketchprefix.read_splines(source,{})
    for item in out['splines']:
        assert item.pop('single_span_mode')==0
        item.pop('interpolation_count')
    prune_compound_kernel(out,set());assert source.p==len(raw)
    oracle=view_oracle('root',out)
    return struct.pack('<BIII',0,version,len(raw),len(oracle))+raw+oracle,len(oracle.splitlines())


def fixture_rows() -> list[tuple[bytes,int]]:
    """Cross inherited bodies, flags, interpolation modes and both callback paths."""
    return [observe(v,single_span_wire(name,rich,flags,mode,extra))
            for v in (0x27cc,19000) for name in LISTS for rich in (False,True)
            for flags in (0,1,4,5,64) for mode in range(5) for extra in (False,True)]


def owned_sketch(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Replace the eighth direct collection in a complete sketch; e.g. owned_sketch(0,19000,2)."""
    mapping=WireMap();mapping.slot=first_slot
    def body() -> bytes:
        raw=ordinary(mapping,version,False)
        splines=single_span_wire(None,False,(0,1,4,5,64,4)[style],style%5,style not in (0,4))
        if edited:splines=splines[:-4]+struct.pack('<i',12345)
        assert raw[:32]==bytes(32)
        return raw[:28]+splines+raw[32:]
    raw=mapping.new('sgSketch',body)
    return raw,mapping.slot+1


def generate() -> None:
    """Save independently observed collections; e.g. generate() inside the immutable pin."""
    rows=fixture_rows();raw=b''.join(b for b,_ in rows);folder=ROOT/'tests/fixtures/single-span-splines';folder.mkdir(exist_ok=True)
    (folder/'cases.bin').write_bytes(raw)
    original=single_span_wire(None,False,4,0,True);edited=original[:-4]+struct.pack('<i',12345)
    observe(19000,original);observe(19000,edited)
    (folder/'original.bin').write_bytes(original);(folder/'edited.bin').write_bytes(edited)
    family=ObjectFixtureFamily(build=owned_sketch)
    saved=[owned(REFERENCE,folder,0,style,False,f'owned-{style}',family) for style in range(6)]
    saved.append(owned(REFERENCE,folder,0,2,True,'owned-2-edited',family))
    report=dict(owned=saved,cases=len(rows),fields=sum(n for _,n in rows),sha256=hashlib.sha256(raw).hexdigest(),excluded_metadata=['derived single_span_mode and interpolation_count'])
    Path(__file__).with_name('reference.json').write_text(json.dumps(report,indent=2)+'\n')
    print(report)
