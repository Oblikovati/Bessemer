"""Observe polygon line properties before their visibility word entered the stream."""
from legacy_mode_wire import mode_root
from legacy_region_reference import region_reference
from generate_component_vectors import integers
from generate_version_range import apply_patches
import dl_ui


def visibility_reader(original: object, spans: list[tuple[int,int]]) -> object:
    def read(archive: object, flag: int = 0) -> dict[str,object]:
        result = original(archive,flag)
        assert flag==0,f"line-property caller flag {flag}; expected 0 in display buckets"
        assert result["visibility"]==5 and archive.b[archive.p-4:archive.p]==integers(5),result
        spans.append((archive.p-4,archive.p))
        return result
    return read


def visibility_root(version: int, kind: str, scenario: int, cached: bool = False) -> tuple[bytes,tuple[str,...]]:
    assert 2650<=version<=2711,f"visibility fixture version {version}; expected 2650..2711"
    raw,identities = mode_root(2711,kind,scenario,cached)
    original,spans = dl_ui.read_moLineViz_c,[]
    dl_ui.read_moLineViz_c = visibility_reader(original,spans)
    try:region_reference(raw,2711,kind,identities)
    finally:dl_ui.read_moLineViz_c = original
    expected = 3 if cached or (scenario>=32 and scenario%16!=2) else 0
    assert len(spans)==expected,(version,kind,scenario,spans)
    edited = scenario==4 or scenario%16==1
    replacement = integers(-2147483648+int(edited)) if version>=0xa80 else b""
    return apply_patches(raw,[(first,last,replacement) for first,last in spans]),identities
