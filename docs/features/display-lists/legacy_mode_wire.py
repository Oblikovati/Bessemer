"""Observe complete viewer records before omitting or widening their display-mode tail."""
from legacy_region_wire import region_root
from legacy_region_reference import region_reference
from generate_component_vectors import integers
from generate_version_range import apply_patches
import dl_tess


def mode_reader(original: object, spans: list[tuple[int,int]]) -> object:
    def read(archive: object) -> dict[str,object]:
        result = original(archive)
        words = result["display_mode_data"]
        first,last = archive.p-4*len(words),archive.p
        assert words==[6] and archive.b[first:last]==integers(*words),(words,archive.b[first:last])
        spans.append((first,last))
        return result
    return read


def mode_tail(version: int, scenario: int, cached: bool) -> bytes:
    if version<=0xaa4:return b""
    if not cached and scenario not in (3,4) and scenario<64:return integers(6)
    edited = scenario==4 or scenario%16==1
    return integers(-2147483648+int(edited),2147483647,-2147483648)


def mode_root(version: int, kind: str, scenario: int, cached: bool = False) -> tuple[bytes,tuple[str,...]]:
    assert 2711<=version<=2725,f"mode fixture version {version}; expected 2711..2725"
    raw,identities = region_root(2725,kind,scenario,cached)
    original,spans = dl_tess.read_moViewerComponentData,[]
    dl_tess.read_moViewerComponentData = mode_reader(original,spans)
    try:root = region_reference(raw,2725,kind,identities)
    finally:dl_tess.read_moViewerComponentData = original
    expected = int(kind=="assembly" and root["has_shaded_data"]!=0)
    assert len(spans)==expected,(kind,scenario,spans)
    tail = mode_tail(version,scenario,cached)
    return apply_patches(raw,[(first,last,tail) for first,last in spans]),identities
