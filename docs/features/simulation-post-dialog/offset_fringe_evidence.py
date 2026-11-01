"""Original C-876/C-877 methods and explicitly qualified full-replay report transitions (#166)."""
import json
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline, dialog_reports
from dialog_bindings import dialog_images
from dialog_field_bindings import verify_field_leaf

REPORTS = ('cosworks-need-offset-native-evidence.json', 'cosworks-fringe-option-native-evidence.json')
HISTORICAL_REPLAYS = ('7fbf4227ae555bbadd13726f4a6b0ad07bf726c14a9d98c8163ae84d1d875a25',
                      '0b36b795192ec4ca71af0f19134858d697dcb879f20b4840af7b12a1ea67784c')
CURRENT_REPLAY = 'eb608cf058f1763c2bbcaf96c9cb800b9215ef94e8b91438e0d6d22af187a4bb'


def offset_fringe_reports() -> list[dict[str, object]]:
    """Retain every original row, native denominator and missing-state qualification; e.g. offset_fringe_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in REPORTS]
    for name, report, evidence, count in zip(REPORTS, reports, ('C-876', 'C-877'), (224, 208), strict=True):
        assert report['evidence'] == evidence and report['target_executed'] is False, evidence
        assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == count, evidence
        dialog_baseline('offset-fringe-' + name, report)
    return reports


def verify_offset_fringe_sources() -> None:
    """Require exact source pins and qualify only the two complete replay-report transitions; e.g. verify_offset_fringe_sources()."""
    current = dialog_reports()[4]
    transitions: list[dict[str, str]] = []
    for report, historical in zip(offset_fringe_reports(), HISTORICAL_REPLAYS, strict=True):
        assert report['repeated_native_summary'] == current['summary'], report['evidence']
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/research/'), original
            actual = digest((SOURCE / relative).read_bytes())
            if actual == expected: continue
            assert relative == 'solidworks/research/cosworks-post-dialog-native-evidence.json', relative
            assert (expected, actual) == (historical, CURRENT_REPLAY), (expected, actual)
            transitions.append(dict(evidence=cast(str, report['evidence']), path=relative, original=expected, current=actual))
    assert len(transitions) == 2, transitions
    dialog_baseline('offset-fringe-source-transitions.json', dict(transitions=transitions,
        reason='Original complete-native summaries match; every original wire input and modeled result is independently replayed from complete native bytes.'))


def verify_offset_fringe_bindings() -> None:
    """Match both full named leaves and their exact exports to the installed provider; e.g. verify_offset_fringe_bindings()."""
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        for report in offset_fringe_reports():
            assert digest(image.__data__) == report['module_sha256'], report['module_sha256']
            method = cast(dict[str, str], report['method'])
            verify_field_leaf(image, method | {'name': method['export']})
    finally:
        for image in images.values(): image.close()
