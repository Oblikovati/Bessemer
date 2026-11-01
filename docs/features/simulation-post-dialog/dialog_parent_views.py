"""Complete raw-preserving parent range and deformation views with native provenance (#166)."""
from collections import Counter
from itertools import product
from typing import cast
import importlib
import json
import struct
from native_carriers import RESEARCH, SOURCE, digest
from dialog_evidence import FEATURE, dialog_baseline, dialog_reports
from dialog_operation_packets import accepted_dialog_native_cases
from dialog_bindings import dialog_images, dialog_export
from dialog_field_bindings import verify_field_leaf
from dialog_owned import dialog_owned_cases

PARENT_REPORTS = ('cosworks-legend-range-fields-native-evidence.json', 'cosworks-deform-scale-fields-native-evidence.json')
HISTORICAL_COLOR = ('f11bf2049dbd4bad26a58b496377c5e0c3bd5fb6284fd79f797574225762f643',
                    '8281bc7d37a518f518244940e9b5c09f4b673143f9cc2f132c77a6c9205d0df2')
CURRENT_COLOR = '97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d'


def dialog_parent_reports() -> list[dict[str, object]]:
    """Keep both complete historical reports intact; e.g. dialog_parent_reports()."""
    reports = [cast(dict[str, object], json.loads((RESEARCH / name).read_bytes())) for name in PARENT_REPORTS]
    for name, report, evidence in zip(PARENT_REPORTS, reports, ('C-874', 'C-875'), strict=True):
        assert report['evidence'] == evidence and report['target_executed'] is False, evidence
        assert report['native_prefixes'] == 224 and report['coverage_credit'] == 0, evidence
        dialog_baseline(name, report)
    return reports


def verify_dialog_parent_sources() -> None:
    """Require exact consumed source pins and record historical replay-input drift explicitly; e.g. verify_dialog_parent_sources()."""
    transitions: list[dict[str, str]] = []
    for report, historical in zip(dialog_parent_reports(), HISTORICAL_COLOR, strict=True):
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
            assert relative.startswith('solidworks/research/'), original
            actual = digest((SOURCE / relative).read_bytes())
            if actual == expected: continue
            assert relative == 'solidworks/research/cosworks-post-color-fields-native-evidence.json', relative
            assert (expected, actual) == (historical, CURRENT_COLOR), (expected, actual)
            transitions.append(dict(evidence=cast(str, report['evidence']), path=relative, original=expected, current=actual))
    assert len(transitions) == 2, transitions
    dialog_baseline('parent-source-adoption-reference.json', dict(bindings=transitions,
        reason='Historical color replay inputs are not executed; every parent row is independently replayed from complete native input bytes'))


def dialog_range_view(raw: bytes) -> dict[str, object]:
    """Model exact-one range selection while retaining the original code and all scalar bits; e.g. dialog_range_view(raw)."""
    helper = importlib.import_module('cosworks_legend_range_fields')
    flags = helper.decode_state(struct.unpack_from('<i', raw, 56)[0])
    view = helper.select_range(*(raw[offset:offset + 4] for offset in (68, 72, 76, 80)),
        minimum_flag=flags['user_minimum_flag'], maximum_flag=flags['user_maximum_flag'])
    return dict(state_offset=56, stored_state_raw_hex=raw[56:60].hex(), state=flags,
        reencoded_combination_view=helper.combination(flags['user_minimum_flag'], flags['user_maximum_flag']),
        range_wire_offsets=dict(baseline_minimum=68, baseline_maximum=72, user_minimum=76, user_maximum=80), range_view=view)


def dialog_scale_view(raw: bytes) -> dict[str, object]:
    """Keep unknown selectors and select the literal-one fallback without geometry claims; e.g. dialog_scale_view(raw)."""
    helper = importlib.import_module('cosworks_deform_scale_fields')
    selector = struct.unpack_from('<i', raw, 52)[0]
    return dict(selector_wire_offset=52, selector_raw_hex=raw[52:56].hex(),
        scale_wire_offsets=dict(scale_528=60, scale_52c=64), view=helper.select_scale(selector, raw[60:64], raw[64:68]))


def verify_dialog_parent_native() -> None:
    """Replay every original prefix-view value from independently recovered complete native inputs; e.g. verify_dialog_parent_native()."""
    packet = (FEATURE.parents[2] / 'target/simulation-post-dialog-native.bin').read_bytes()
    cases = [(version, raw) for family, version, raw in accepted_dialog_native_cases(packet) if family == 0]
    assert len(cases) == 224, len(cases)
    available = {(digest(raw), version): raw for version, raw in cases}
    originals = cast(list[dict[str, object]], dialog_reports()[4]['rows'])
    identities = Counter((row['path'], row['stream'], row['sha256'], attempt['version'])
        for row in originals for attempt in cast(list[dict[str, object]], row['attempts']) if attempt['success'])
    for report, project in zip(dialog_parent_reports(), (dialog_range_view, dialog_scale_view), strict=True):
        rows = cast(list[dict[str, object]], report['rows'])
        assert Counter((row['path'], row['stream'], row['sha256'], row['selected_version']) for row in rows) == identities
        assert Counter((row['sha256'], row['selected_version']) for row in rows) == Counter((digest(raw), version) for version, raw in cases)
        for row in rows:
            raw = available[(cast(str, row['sha256']), cast(int, row['selected_version']))]
            assert row['scope'] == 'prefix_only', row['scope']
            actual = project(raw)
            assert {key: row[key] for key in actual} == actual, row['stream']


def verify_dialog_parent_bindings() -> None:
    """Bind complete accessor bytes, exact export names and the constant against hashed installed images; e.g. verify_dialog_parent_bindings()."""
    images = dialog_images()
    try:
        image = images['dgxpost.dll']
        for report in dialog_parent_reports():
            assert digest(image.__data__) == report['module_sha256'], report['evidence']
            reader = json.loads((RESEARCH / 'cosworks-post-dialog-reader-probe.json').read_bytes())[0]
            start, end = int(reader['address'], 16), int(reader['end'], 16)
            assert digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)) == report['reader_sha256']
            functions = cast(list[dict[str, str]], report.get('functions', [report.get('method')]))
            for function in functions:
                verify_field_leaf(image, function)
                name = function['export'].encode()
                assert dialog_export(image, name, int(function['address'], 16)) == name, name
            if 'constant' not in report: continue
            constant = cast(dict[str, str], report['constant'])
            assert image.get_data(int(constant['address'], 16) - image.OPTIONAL_HEADER.ImageBase, 4).hex() == constant['raw_hex']
    finally:
        for image in images.values(): image.close()


def verify_dialog_parent_owned() -> None:
    """Check synthetic branch selection against literal raw slices, including unknown selectors and nonfinite bits; e.g. verify_dialog_parent_owned()."""
    rows: list[dict[str, object]] = []
    for family, version, _, raw in dialog_owned_cases():
        if family != 0: continue
        range_view, scale_view = dialog_range_view(raw), dialog_scale_view(raw)
        assert_dialog_parent_bits(raw, range_view, scale_view)
        rows.append(dict(version=version, sha256=digest(raw), range=range_view, scale=scale_view))
    assert len(rows) == 225, len(rows)
    dialog_baseline('owned-parent-views-reference.json', rows)


def assert_dialog_parent_bits(raw: bytes, range_view: dict[str, object], scale_view: dict[str, object]) -> None:
    """Use independent literal offsets and branches rather than helper-derived expected values; e.g. assert_dialog_parent_bits(raw, range_view, scale_view)."""
    code, selector = struct.unpack_from('<i', raw, 56)[0], struct.unpack_from('<i', raw, 52)[0]
    flags = dict(stored_combination=code, user_minimum_flag=int(code == 1 or code == 2), user_maximum_flag=int(code == 1 or code == 3))
    assert range_view['state'] == flags, (code, range_view)
    ranges = cast(dict[str, dict[str, object]], range_view['range_view'])
    for name, offset in (('baseline_minimum', 68), ('baseline_maximum', 72), ('user_minimum', 76), ('user_maximum', 80)):
        assert ranges[name]['raw_hex'] == raw[offset:offset + 4].hex(), name
    minimum = raw[76:80] if code in (1, 2) else raw[68:72]
    maximum = raw[80:84] if code in (1, 3) else raw[72:76]
    assert (ranges['selected_minimum']['raw_hex'], ranges['selected_maximum']['raw_hex']) == (minimum.hex(), maximum.hex())
    scales = cast(dict[str, object], scale_view['view'])
    selected = raw[60:64] if selector == 0 else raw[64:68] if selector == 1 else b'\x00\x00\x80\x3f'
    assert cast(dict[str, object], scales['selected_scale'])['raw_hex'] == selected.hex(), selector


def verify_dialog_parent_branches() -> None:
    """Exercise every stored code and selector branch with exact zero/subnormal/nonfinite bit patterns; e.g. verify_dialog_parent_branches()."""
    from dialog_literal_values import FLOAT32_PATTERNS
    raw = bytearray(84)
    codes = (0, 1, 2, 3, -1, -2147483648, 2147483647)
    selectors = (0, 1, 2, -1, -2147483648, 2147483647)
    for code, selector, rotation in product(codes, selectors, range(len(FLOAT32_PATTERNS))):
        struct.pack_into('<ii', raw, 52, selector, code)
        for index in range(6):
            struct.pack_into('<I', raw, 60 + 4 * index, FLOAT32_PATTERNS[(rotation + index) % len(FLOAT32_PATTERNS)])
        literal = bytes(raw)
        assert_dialog_parent_bits(literal, dialog_range_view(literal), dialog_scale_view(literal))
