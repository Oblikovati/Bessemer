"""Complete C-884 local predicate evidence and its narrow runtime qualifications (#166)."""
import json
import struct
from typing import cast
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import dialog_baseline
from dialog_bindings import dialog_images
from dialog_field_bindings import verify_field_leaf


def local_render_report() -> dict[str, object]:
    """Retain every original explicit native input without promoting the method name into global safety; e.g. local_render_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-render-safety-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-884' and report['target_executed'] is False
    assert report['coverage_credit'] == 0 and len(cast(list[object], report['rows'])) == 276
    dialog_baseline('local-render-c884-reference.json', report)
    return report


def verify_local_render_sources() -> None:
    """Require exact code/probe pins and only the qualified later native-color report transition; e.g. verify_local_render_sources()."""
    transitions: list[dict[str, str]] = []
    for original, expected in cast(dict[str, str], local_render_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/research/'), original
        actual = digest((SOURCE / relative).read_bytes())
        if actual == expected: continue
        assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
        assert (expected, actual) == ('f11bf2049dbd4bad26a58b496377c5e0c3bd5fb6284fd79f797574225762f643', '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'), (expected, actual)
        transitions.append(dict(path=relative, original=expected, current=actual))
    assert len(transitions) == 1, transitions
    dialog_baseline('local-render-source-transitions.json', dict(transitions=transitions,
        reason='All original native identities, exact input words and offsets are independently checked from complete bytes.'))


def verify_local_render_bindings() -> None:
    """Bind both return paths, the exact export and the reader's style-subobject call; e.g. verify_local_render_bindings()."""
    report = local_render_report()
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        assert digest(image.__data__) == report['module_sha256']
        method = cast(dict[str, str], report['method'])
        verify_field_leaf(image, method | {'address': method['start'], 'name': method['export']})
        assert image.get_data(0x957df, 7) == bytes.fromhex('498d8f40020000')
        call = image.get_data(0x957ec, 5)
        assert call[:1] == b'\xe8' and 0x1800957f1 + struct.unpack_from('<i', call, 1)[0] == 0x180044ae0
    finally:
        for image in images.values(): image.close()
