"""Collect every recorded visual interval and named linkage without changing its original scope (#181)."""
from pathlib import Path
from typing import TypedDict, cast
from visual_state_evidence import visual_reports


class VisualImageInputs(TypedDict):
    sha256: str
    spans: list[dict[str, str]]
    exports: dict[str, int]
    imports: list[dict[str, str]]
    normalizers: list[dict[str, object]]


def visual_image_slot(catalog: dict[str, VisualImageInputs], module: str, digest: str) -> VisualImageInputs:
    """Require all reports to bind one image revision; e.g. visual_image_slot(catalog, path, sha)."""
    path = str(Path(module))
    slot = catalog.setdefault(path, dict(sha256=digest, spans=[], exports={}, imports=[], normalizers=[]))
    assert slot['sha256'] == digest, f'{path}/{digest}; expected the common image pin {slot["sha256"]}'
    return slot


def add_visual_named_export(slot: VisualImageInputs, name: str, address: str) -> None:
    """Reject contradictory named bindings while retaining shared implementation addresses; e.g. add_visual_named_export(slot, name, address)."""
    value = int(address, 16)
    prior = slot['exports'].setdefault(name, value)
    assert prior == value, f'{name}/{address}; expected its original unique named entry {prior:x}'


def add_visual_interval(slot: VisualImageInputs, row: dict[str, object], evidence: str) -> None:
    """Keep every complete reported range, including overlapping capture extents; e.g. add_visual_interval(slot, row, evidence)."""
    selected = {key: cast(str, row[key]) for key in ('start', 'end', 'bytes', 'sha256')}
    selected['evidence'] = evidence
    slot['spans'].append(selected)
    names = cast(list[str], row.get('exports', []))
    if 'export' in row:
        names = [cast(str, row['export']), *names]
    for name in names:
        add_visual_named_export(slot, name, selected['start'])


def add_visual_report_rows(slot: VisualImageInputs, report: dict[str, object]) -> None:
    """Retain singular functions and full-word discovery unwind fragments; e.g. add_visual_report_rows(slot, report)."""
    evidence = cast(str, report['evidence'])
    rows = cast(list[dict[str, object]], report.get('functions', []))
    if 'function' in report:
        rows = [*rows, cast(dict[str, object], report['function'])]
    for row in rows:
        add_visual_interval(slot, row, evidence)
    for row in cast(list[dict[str, object]], report.get('rows', [])):
        add_visual_named_export(slot, cast(str, row['export']), cast(str, row['entry']))
        for fragment in cast(list[dict[str, object]], row['fragments']):
            add_visual_interval(slot, fragment, evidence)
    for row in cast(list[dict[str, str]], report.get('exports', [])):
        add_visual_named_export(slot, row['name'], row['address'])
    slot['imports'].extend(cast(list[dict[str, str]], report.get('imports', [])))


def add_visual_module(catalog: dict[str, VisualImageInputs], module: dict[str, object], evidence: str) -> None:
    """Preserve the graphics consumer/provider links and raw normalizer constant; e.g. add_visual_module(catalog, module, evidence)."""
    slot = visual_image_slot(catalog, cast(str, module['module']), cast(str, module['sha256']))
    for row in cast(list[dict[str, object]], module['fragments']):
        add_visual_interval(slot, row, evidence)
    for instruction, binding in cast(dict[str, dict[str, str]], module.get('call_bindings', {})).items():
        slot['imports'].append(dict(instruction=instruction, name=binding['symbol'], dll=binding['dll'], iat=binding['iat']))
    if 'normalizer' in module:
        slot['normalizers'].append(cast(dict[str, object], module['normalizer']))


def visual_native_catalog() -> dict[str, VisualImageInputs]:
    """Collect all three reported installed images with no inferred targets; e.g. visual_native_catalog()."""
    catalog: dict[str, VisualImageInputs] = {}
    for report in visual_reports():
        if 'module' in report:
            slot = visual_image_slot(catalog, cast(str, report['module']), cast(str, report['sha256']))
            add_visual_report_rows(slot, report)
        for module in cast(list[dict[str, object]], report.get('modules', [])):
            add_visual_module(catalog, module, cast(str, report['evidence']))
    assert len(catalog) == 3, f'{list(catalog)}; expected sldmodu, sldrgl4u and sldgfxu'
    return catalog
