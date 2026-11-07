"""Observe old pane strings before replacing primitive-only display item lists."""
import struct
from pre_facet_wire import prototype,observed,converted,items
from pre_facet_external import external_root
from generate_version_range import apply_patches
from generate_unsigned_versions import reference
from generate_transform_vectors import doubles,compact
from generate_named_vectors import cstring
from generate_component_vectors import integers
from generate_item_vectors import record
import dl_ui


def capture_pane_text(original: object, spans: list[tuple[int,int]]) -> object:
    def read(archive: object) -> object:
        cstring_reader = archive.cstring
        def capture() -> str:
            start = archive.p
            result = cstring_reader()
            spans.append((start,archive.p))
            return result
        archive.cstring = capture
        try:return original(archive)
        finally:archive.cstring = cstring_reader
    return read


def pane_spans(raw: bytes, kind: str) -> list[tuple[int,int]]:
    original,spans = dl_ui.read_uiSplitterInfo_c,[]
    dl_ui.read_uiSplitterInfo_c = capture_pane_text(original,spans)
    try:reference(raw,3196,kind,())
    finally:dl_ui.read_uiSplitterInfo_c = original
    assert len(spans)==3,f"pane strings {spans}; expected three owned panes"
    return spans


def display_items(version: int, edited: bool) -> bytes:
    earlier = items(version,edited)
    flag = integers(-2147483648+int(edited)) if version<=0xbf8 else bytes([255-int(edited)])
    line = doubles(6)+flag
    symbol = doubles(3)
    if version>0xbbd:symbol += integers(-7)+compact(2,0x40)
    symbol += doubles(2)+cstring("symbol 鋼",True)+cstring("secondary")
    return struct.pack("<H",4)+earlier[2:]+record(4,line)+record(19,symbol)


def root_bytes(version: int, kind: str, scenario: int) -> tuple[bytes,tuple[str,...]]:
    raw,identities = external_root(version,scenario==4) if scenario in (3,4) else (converted(version,kind,scenario),())
    original = prototype(kind,scenario-16 if scenario>=16 else None)
    spans = observed(original,kind)
    old_items = items(version,scenario in (1,4))
    start = spans["items"][0]
    assert raw[start:start+len(old_items)]==old_items
    patches = [(start,start+len(old_items),display_items(version,scenario in (1,4)))]
    if version<=0xbda:
        strings = pane_spans(original,kind)
        assert all(last<=start for _,last in strings)
        patches += [(first,last,b"") for first,last in strings]
    return apply_patches(raw,patches),identities
