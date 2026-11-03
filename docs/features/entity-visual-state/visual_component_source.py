"""Original complete display-list Source acceptance for owned component seeds (#181)."""
from pathlib import Path
from typing import NamedTuple, Protocol, cast
import importlib
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage
from visual_component_seeds import ComponentVisualSeed


class ComponentSourceArchive(Protocol):
    p: int
    version: int
    map: list[object]


class CapturedComponentSeed(NamedTuple):
    root: dict[str, object]
    archive: ComponentSourceArchive
    physical: list[PrimitiveRead]


def capture_component_visual_seed(root: Path, seed: ComponentVisualSeed) -> CapturedComponentSeed:
    """Read a complete root with its real document/version context and original readers; e.g. capture_component_visual_seed(root, seed)."""
    extension = 'SLDPRT' if seed.kind == 'part' else 'SLDASM'
    path = root / 'tests/fixtures' / f'blank-{seed.kind}.{extension}'
    document = importlib.import_module('swcontainer').Document(str(path))
    display = importlib.import_module('displaylists')
    source = importlib.import_module('swarchive')
    with observe_primitives(source.Archive, seed.raw) as reads:
        archive = display.archive_for(document, str(path), seed.raw)
        assert archive.version == seed.version, f'{archive.version}; expected the owned display version {seed.version} from document names'
        parsed = display.display_lists(archive, seed.kind)
    assert archive.p == len(seed.raw), f'{archive.p}/{len(seed.raw)}; expected complete original-reader consumption'
    physical = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(seed.raw, physical)
    return CapturedComponentSeed(cast(dict[str, object], parsed), cast(ComponentSourceArchive, archive), physical)


def component_seed_visual_spans(captured: CapturedComponentSeed) -> list[int]:
    """Locate each physical shared-codec record through its original color read; e.g. component_seed_visual_spans(captured)."""
    return [read.start - 5 for read in captured.physical if read.kind == 'I'
        and (read.source_file, read.source_function) == ('dl_light.py', 'read_moEntVisProp_c')]


def component_seed_properties(captured: CapturedComponentSeed) -> list[dict[str, object]]:
    """Keep physical property definitions distinct from holder aliases; e.g. component_seed_properties(captured)."""
    return [cast(dict[str, object], entry) for entry in captured.archive.map
        if isinstance(entry, dict) and entry.get('_class') == 'moComponentProperty']


def verify_component_seed_aliases(seed: ComponentVisualSeed, captured: CapturedComponentSeed) -> None:
    """Require the actual nested-holder cycle and shared property identity; e.g. verify_component_seed_aliases(seed, captured)."""
    holders = [cast(dict[str, object], entry) for entry in captured.archive.map
        if isinstance(entry, dict) and entry.get('_class') == 'moComponentPropertyHolder']
    expected = 0 if seed.scenario < 16 else 2 if seed.scenario == 21 else 1
    assert len(holders) == expected, f'{len(holders)}; expected{expected} complete holder definitions for scenario{seed.scenario}'
    if seed.scenario == 17:
        assert holders[0]['property'] is None, f'{holders[0]["property"]}; expected the original null property'
    if seed.scenario != 21:
        return
    outer, inner = holders
    cycle = outer['unknown_8'] is inner, inner['unknown_8'] is outer
    assert cycle == (True, True), f'{cycle}; expected(True,True) for the original outer/inner reference cycle'
    shared = outer['property'] is inner['property']
    assert shared, f'{shared}; expected True for the original shared physical property'
