"""Original constructor and later alpha-preserving assignment evidence, with explicit source transitions (#166)."""
import json
import pefile
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from dialog_field_bindings import verify_field_leaf

REPORTS = (
    ('cosworks-colorref-import-view-native-evidence.json', 'colorref-import-c862-reference.json', 'C-862'),
    ('cosworks-colorref-assignment-evidence.json', 'colorref-assignment-c906-reference.json', 'C-906'),
)
ADOPTIONS = {
    'solidworks/research/cosworks_colorref_import_view.py': ('2911a13b4150ab95f3506a563e482cad7dff0c2fe69b751b9602d500fc0992db', '2f20a1942112c9cb700912529d402dacd84eb2c6ecd86df4090346a0421cc32b'),
    'solidworks/research/cosworks-post-color-fields-native-evidence.json': ('1d296569880aa715cf2a33301d586df68b4fc8d6d0de65398557ffbc2e83317e', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'),
}


def colorref_import_reports() -> tuple[dict[str, object], dict[str, object]]:
    """Retain complete original reports without replaying mutating audit entry points; e.g. colorref_import_reports()."""
    reports: list[dict[str, object]] = []
    for source, target, evidence in REPORTS:
        report = cast(dict[str, object], json.loads((RESEARCH / source).read_bytes()))
        assert report['evidence'] == evidence and report['target_executed'] is False, evidence
        assert report['coverage_credit'] == 0, evidence
        dialog_baseline(target, report)
        reports.append(report)
    return reports[0], reports[1]


def verify_colorref_import_sources() -> None:
    """Require latest assignment pins and only its two exact constructor-era transitions; e.g. verify_colorref_import_sources()."""
    original, assignment = colorref_import_reports()
    latest = {path.replace('\\', '/').split('/inventor-interface/')[-1]: value
              for path, value in cast(dict[str, str], assignment['sources']).items()}
    observed: list[dict[str, str]] = []
    for report in (assignment, original):
        for path, expected in cast(dict[str, str], report['sources']).items():
            relative = path.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/research/'), path
            current = digest((SOURCE / relative).read_bytes())
            if current == expected: continue
            assert report is original and ADOPTIONS.get(relative) == (expected, current), (relative, expected, current)
            assert latest[relative] == current, (path, current)
            observed.append(dict(path=relative, original=expected, current=current))
    assert len(observed) == 2, observed
    dialog_baseline('colorref-import-source-transitions.json', dict(transitions=observed))


def verify_colorref_import_bindings() -> None:
    """Bind complete constructor/assignment bytes, exports and the exact inverse scale; e.g. verify_colorref_import_bindings()."""
    original, assignment = colorref_import_reports()
    images = dialog_images()
    try:
        image = images['dspogl.dll']
        actual = digest(image.__data__)
        assert actual == original['module_sha256'] == assignment['provider_sha256'], actual
        constructor = cast(dict[str, str], original['constructor'])
        verify_field_leaf(image, constructor)
        dialog_export(image, constructor['export'].encode(), int(constructor['address'], 16))
        verify_assignment_leaf(image, assignment)
        scale = cast(dict[str, str], original['inverse_scale'])
        assert scale['raw_hex'] == assignment['scale_raw_hex'] == '8180803b', scale
        assert image.get_data(int(scale['address'], 16) - image.OPTIONAL_HEADER.ImageBase, 4).hex() == scale['raw_hex']
    finally:
        for image in images.values(): image.close()


def verify_assignment_leaf(image: pefile.PE, report: dict[str, object]) -> None:
    """Keep the assignment overload distinct from constructor alpha initialization; e.g. verify_assignment_leaf(image, report)."""
    raw = bytes.fromhex(cast(str, report['raw_hex']))
    leaf = dict(address=cast(str, report['start']), end=cast(str, report['end']),
        raw_hex=raw.hex(), sha256=digest(raw))
    verify_field_leaf(image, leaf)
    dialog_export(image, cast(str, report['export']).encode(), int(leaf['address'], 16))
    assert report['destination_store_offsets'] == [0, 4, 8], report['destination_store_offsets']
