"""Original C-872/C-873 rendering-decision evidence with exact source transitions (#166)."""
import json
import pefile
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_bindings import dialog_images, dialog_export
from colorref_import_evidence import colorref_import_reports
from dialog_field_bindings import verify_field_leaf

RENDER_REPORTS = ('cosworks-legend-render-gate-native-evidence.json', 'cosworks-need-model-tess-native-evidence.json')
COLOR_TRANSITIONS = {'C-872': '3efb4ca03307aa6f8dc7205c49e435bf3588f9f8d8ed65b66af39d3b90f3546e',
                     'C-873': '748b9255fd995c96485f97476d694971e663dfed04796636127784cc7134d877'}


def render_policy_reports() -> list[dict[str, object]]:
    """Keep the full original rows and runtime/version exclusions, without invoking audit writers; e.g. render_policy_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in RENDER_REPORTS]
    for name, report, evidence, count in zip(RENDER_REPORTS, reports, ('C-872', 'C-873'), (276, 160), strict=True):
        assert report['evidence'] == evidence and report['target_executed'] is False, evidence
        assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == count, evidence
        dialog_baseline('render-policy-' + name, report)
    return reports


def verify_render_policy_sources() -> None:
    """Accept only the two exact later color-census transitions qualified by C-906; e.g. verify_render_policy_sources()."""
    _, latest = colorref_import_reports()
    pins = {path.replace('\\', '/').split('/inventor-interface/')[-1]: value for path, value in cast(dict[str, str], latest['sources']).items()}
    transitions: list[dict[str, str]] = []
    for report in render_policy_reports():
        for path, expected in cast(dict[str, str], report['sources']).items():
            relative = path.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/research/'), path
            actual = digest((SOURCE / relative).read_bytes())
            if actual == expected: continue
            assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
            assert expected == COLOR_TRANSITIONS[cast(str, report['evidence'])], (expected, report['evidence'])
            assert actual == pins[relative] == '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d', actual
            transitions.append(dict(evidence=cast(str, report['evidence']), path=relative, original=expected, current=actual))
    assert len(transitions) == 2, transitions
    dialog_baseline('render-policy-source-transitions.json', dict(transitions=transitions))


def verify_render_policy_bindings() -> None:
    """Bind complete named method bytes, exports, dialog reader and the hidden-parts getter; e.g. verify_render_policy_bindings()."""
    reports = render_policy_reports()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        for report, export in zip(reports, (b'?Layer0_RenderBody@CDspLegend@@', b'?NeedModelTess@CcwPostDlgParam@@'), strict=True):
            assert digest(image.__data__) == report['module_sha256'], report['module_sha256']
            method = cast(dict[str, str], report['method'])
            start, end = int(method['address'], 16), int(method['end'], 16)
            assert digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)) == method['sha256'], method
            dialog_export(image, export, start)
        reader = json.loads((RESEARCH / 'cosworks-post-dialog-reader-probe.json').read_bytes())[0]
        start, end = int(reader['address'], 16), int(reader['end'], 16)
        assert digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)) == reports[1]['reader_sha256']
        verify_render_hidden_getter(image)
    finally:
        for image in images.values(): image.close()


def verify_render_hidden_getter(image: pefile.PE) -> None:
    """Keep the exact hidden-option getter tied to its retained signed-word field binding; e.g. verify_render_hidden_getter(image)."""
    report = json.loads((RESEARCH / 'cosworks-post-style-alpha-fields-native-evidence.json').read_bytes())
    leaves = [leaf for leaf in report['leaves'] if leaf['address'] == '0x180044540']
    assert len(leaves) == 1 and leaves[0]['object_offset'] == '0x1ec', leaves
    verify_field_leaf(image, leaves[0])
    dialog_export(image, leaves[0]['export'].encode(), 0x180044540)
