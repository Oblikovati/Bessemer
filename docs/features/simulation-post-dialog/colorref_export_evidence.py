"""Original GetCOLORREF conditional model and complete installed bindings (#166)."""
import json
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from dialog_field_bindings import verify_field_leaf
from colorref_import_evidence import colorref_import_reports


def colorref_export_report() -> dict[str, object]:
    """Retain the complete original projection report without rewriting historical qualifications; e.g. colorref_export_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-colorref-view-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-861' and report['target_executed'] is False, report['evidence']
    assert report['coverage_credit'] == 0, report['coverage_credit']
    dialog_baseline('colorref-export-c861-reference.json', report)
    return report


def verify_colorref_export_sources() -> None:
    """Require every original source or the exact later color-report transition; e.g. verify_colorref_export_sources()."""
    report = colorref_export_report()
    _, latest = colorref_import_reports()
    pins = {path.replace('\\', '/').split('/inventor-interface/')[-1]: value
            for path, value in cast(dict[str, str], latest['sources']).items()}
    transitions: list[dict[str, str]] = []
    for path, expected in cast(dict[str, str], report['sources']).items():
        relative = path.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), path
        current = digest((SOURCE / relative).read_bytes())
        if current == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, current) == ('a1a672e78546d9dbddc7ee55b6315916003c8776dfe7acfd7ea1907495ccb67c', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, current)
        assert pins[relative] == current, current
        transitions.append(dict(path=relative, original=expected, current=current))
    assert len(transitions) == 1, transitions
    dialog_baseline('colorref-export-source-transitions.json', dict(transitions=transitions))


def verify_colorref_export_bindings() -> None:
    """Bind the full named converter and exact binary32 factor without native execution; e.g. verify_colorref_export_bindings()."""
    report = colorref_export_report()
    images = dialog_images()
    try:
        image = images['dspogl.dll']
        assert digest(image.__data__) == report['module_sha256'], report['module_sha256']
        method = cast(dict[str, str], report['method'])
        verify_field_leaf(image, method)
        dialog_export(image, method['export'].encode(), int(method['address'], 16))
        scale = cast(dict[str, str], report['scale'])
        assert scale['raw_hex'] == '00007f43', scale
        assert image.get_data(int(scale['address'], 16) - image.OPTIONAL_HEADER.ImageBase, 4).hex() == scale['raw_hex']
    finally:
        for image in images.values(): image.close()
