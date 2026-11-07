"""Complete C-878/C-879/C-880 evidence, native bindings and exact report transitions (#166)."""
import json
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline, dialog_reports
from dialog_bindings import dialog_images, dialog_export, dialog_import

VECTOR_REPORTS = ('cosworks-vector-direction-branch-native-evidence.json',
    'cosworks-component-vector-tables-evidence.json', 'cosworks-component-vector-policy-native-evidence.json')
OLD_REPLAYS = {'C-878': 'd679aabfee88349b87a2d4d31e242c89eee65f3ab77dc4168325cf4cb6526547',
               'C-880': '563c58f3bafa9cdf9502ce3e83b0471ddf129b6459a9f0755c9f6f67a0feb466'}


def vector_reports() -> list[dict[str, object]]:
    """Preserve complete original records and all static/runtime qualifications; e.g. vector_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in VECTOR_REPORTS]
    for name, report, evidence in zip(VECTOR_REPORTS, reports, ('C-878', 'C-879', 'C-880'), strict=True):
        assert report['evidence'] == evidence and report['target_executed'] is False, evidence
        assert report['coverage_credit'] == 0, evidence
        dialog_baseline('vector-' + name, report)
    assert len(cast(list[object], reports[0]['rows'])) == reports[2]['native_records'] == 224
    assert (reports[1]['table_count'], reports[1]['table_entries'], reports[1]['unique_tails'], reports[1]['unique_outputs']) == (23, 262, 11, 9)
    return reports


def verify_vector_sources() -> None:
    """Require all original code/table pins and qualify only exact complete-replay report changes; e.g. verify_vector_sources()."""
    reports = vector_reports()
    assert reports[0]['repeated_native_summary'] == dialog_reports()[4]['summary']
    transitions: list[dict[str, str]] = []
    for report in reports:
        for path, expected in cast(dict[str, str], report['sources']).items():
            relative = path.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/research/'), path
            actual = digest((SOURCE / relative).read_bytes())
            if actual == expected: continue
            assert relative == 'solidworks/research/cosworks-post-dialog-native-evidence.json', relative
            assert expected == OLD_REPLAYS[cast(str, report['evidence'])], expected
            assert actual == 'eb608cf058f1763c2bbcaf96c9cb800b9215ef94e8b91438e0d6d22af187a4bb', actual
            transitions.append(dict(evidence=cast(str, report['evidence']), path=relative, original=expected, current=actual))
    assert len(transitions) == 2, transitions
    dialog_baseline('vector-source-transitions.json', dict(transitions=transitions,
        reason='Complete original native inputs and results are independently replayed without executing historical audit writers.'))


def verify_vector_bindings() -> None:
    """Bind the full local/provider methods and the exact four-argument imported symbol; e.g. verify_vector_bindings()."""
    local, table, _ = vector_reports()
    images = dialog_images()
    try:
        caller, provider = images['dgxpost.dll'], images['cwpostcommon.dll']
        assert digest(caller.__data__) == local['module_sha256']
        assert digest(provider.__data__) == table['module_sha256']
        for image, method in ((caller, local['method']), (provider, table['code'])):
            leaf = cast(dict[str, str], method)
            start, end = int(leaf['address'], 16), int(leaf['end'], 16)
            assert digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)) == leaf['sha256'], leaf['address']
        dialog_export(caller, b'?GetVectorDir@CcwPostDlgParam@@', 0x1800907b0)
        symbol = b'?dgxPostCommonGetComponentVectorDir@@YAXHHHHQEAM@Z'
        dialog_export(provider, symbol, 0x18002ebb0)
        dialog_import(caller, b'cwpostcommon.dll', symbol, 0x1802cc3b8)
    finally:
        for image in images.values(): image.close()
