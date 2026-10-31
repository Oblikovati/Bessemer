"""Complete ShellManager header/group reference foundation without native execution (#156)."""
from pathlib import Path
from typing import cast
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for name in ('simulation-binary-thermal', 'model-archives'):
    sys.path.insert(0, str(FEATURE.parent / name))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import SOURCE, RESEARCH, digest, recover_sample, pinned_auxiliaries
from owned_shell import owned_shell_groups, owned_shell_headers
from shell_literal_oracles import shell_literal_spans, shell_literal_edit, shell_literal_history, shell_header_spans, shell_header_edit

REPORTS = ('cosworks-shell-manager-native-evidence.json', 'cosworks-shell-group-native-evidence.json')


def shell_baseline(name: str, value: object) -> None:
    """Require complete immutable evidence outside explicit initialization; e.g. shell_baseline('foundation.json', report)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def shell_reports() -> list[dict[str, object]]:
    """Retain both complete checkpoints without replacing the older empty-group boundary; e.g. shell_reports()."""
    reports = [json.loads((RESEARCH / name).read_bytes()) for name in REPORTS]
    assert [report['evidence'] for report in reports] == ['C-741', 'C-747']
    assert all(not report['target_executed'] for report in reports)
    shell_baseline('initial-reference.json', reports)
    return cast(list[dict[str, object]], reports)


def verify_shell_sources() -> None:
    """Require every original reader/decompilation hash and full clause; e.g. verify_shell_sources()."""
    for report in shell_reports():
        for original, expected in cast(dict[str, str], report['sources']).items():
            relative = original.replace('\\', '/').split('/inventor-interface/', 1)[1]
            assert digest((SOURCE / relative).read_bytes()) == expected, relative
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    for section in ('14.363', '14.368'):
        start = text.index('### ' + section + ' ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        path = FEATURE / f'schema-{section}.md'
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw


def verify_shell_image(image: pefile.PE, report: dict[str, object]) -> None:
    """Read installed function/threshold/literal bytes without loading the DLL; e.g. verify_shell_image(image, report)."""
    base = image.OPTIONAL_HEADER.ImageBase
    for function in cast(list[dict[str, str]], report['functions']):
        start, end = int(function['address'], 16), int(function['end'], 16)
        assert digest(image.get_data(start - base, end - start)) == function['raw_sha256']
    for key in ('version_constants', 'literals'):
        for binding in cast(list[dict[str, str]], report[key]):
            expected = bytes.fromhex(binding['raw_hex'])
            assert image.get_data(int(binding['address'], 16) - base, len(expected)) == expected


def verify_shell_bindings() -> None:
    """Require the exact installed image for both original static checkpoints; e.g. verify_shell_bindings()."""
    raw = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation/cosworks.dll').read_bytes()
    image = pefile.PE(data=raw, fast_load=True)
    try:
        for report in shell_reports():
            assert digest(raw) == report['module_sha256']
            verify_shell_image(image, report)
    finally:
        image.close()


def shell_attempt(raw: bytes, version: int, kind: str, group_records: bool) -> dict[str, object]:
    """Keep each original checkpoint's parser boundary explicit; e.g. shell_attempt(raw, 890016, 'ShellManagerGroup', True)."""
    from cosworks_shell_manager_layout import parse as manager
    from cosworks_shell_group_layout import parse as group
    parsed = group(raw, version, kind) if group_records else manager(raw, version, kind)
    return dict(version=version, **parsed)


def verify_shell_owned() -> None:
    """Match independently constructed signed and populated CString/double values; e.g. verify_shell_owned()."""
    from cosworks_shell_group_layout import parse
    headers, groups = [], []
    for version, words, raw in owned_shell_headers():
        parsed = shell_attempt(raw, version, 'Property', False)
        assert parsed['success'] and parsed['values'] == words and parsed['end'] == len(raw)
        headers.append(dict(version=version, raw_hex=raw.hex(), expected=parsed))
    for case in owned_shell_groups():
        raw = case['raw']
        parsed = parse(raw, case['version'], wire_profile=case['wire_profile'])
        assert parsed['success'] and parsed['end'] == len(raw) and parsed['records'] == case['records']
        assert b''.join(bytes.fromhex(span['raw_hex']) for span in parsed['spans']) == raw
        assert parsed['reconstruction_sha256'] == digest(raw)
        groups.append(dict(version=case['version'], wire_profile=case['wire_profile'], pattern=case['pattern'], raw_hex=raw.hex(), expected=parsed))
    shell_baseline('owned-reference.json', dict(headers=headers, groups=groups))


def verify_shell_literal_oracles() -> None:
    """Compare every primitive boundary and independent edited record against the actual reader; e.g. verify_shell_literal_oracles()."""
    from cosworks_shell_group_layout import parse
    oracles: list[dict[str, object]] = []
    for case in owned_shell_groups():
        original = parse(case['raw'], case['version'], wire_profile=case['wire_profile'])
        assert original['spans'] == shell_literal_spans(case)
        raw, expected_records, ordinal = shell_literal_edit(case)
        edited = parse(raw, case['version'], wire_profile=case['wire_profile'])
        assert edited['success'] and edited['records'] == expected_records and edited['end'] == len(raw)
        assert edited['reconstruction_sha256'] == digest(raw)
        oracles.append(dict(version=case['version'], wire_profile=case['wire_profile'], pattern=case['pattern'],
                       ordinal=ordinal, original_spans=original['spans'], expected_edited=edited,
                       history=shell_literal_history(case, ordinal)))
    shell_baseline('literal-oracle-reference.json', oracles)


def verify_shell_header_oracles() -> None:
    """Pin every header field and complete edited body independently of the actual parser; e.g. verify_shell_header_oracles()."""
    oracles: list[dict[str, object]] = []
    for version, words, original in owned_shell_headers():
        spans = shell_header_spans(words)
        assert b''.join(bytes.fromhex(cast(str, span['raw_hex'])) for span in spans) == original
        raw, expected_words, history = shell_header_edit(words)
        parsed = shell_attempt(raw, version, 'Property', False)
        assert parsed['success'] and parsed['values'] == expected_words and parsed['end'] == len(raw)
        assert parsed['reconstruction_sha256'] == digest(raw)
        assert raw[4:] == original[4:]
        oracles.append(dict(version=version, original_spans=spans, expected_edited=parsed, history=history))
    shell_baseline('header-oracle-reference.json', oracles)


def replay_shell_row(row: dict[str, object], streams: dict[str, bytes], group_records: bool) -> bytes:
    """Preserve all original candidate outcomes and complete payloads; e.g. replay_shell_row(row, streams, True)."""
    stream = cast(str, row['stream'])
    raw = streams[stream]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    kind = stream.rsplit('/', 1)[-1]
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = shell_attempt(raw, version, kind, group_records)
        assert parsed == expected, (stream, version)
        packet.extend(struct.pack('<IIII', int(group_records), version, int(cast(bool, parsed['success'])), len(raw)) + raw)
    return bytes(packet)


def replay_shell_native(pin: Path) -> None:
    """Recover each envelope once while retaining old/new group checkpoints separately; e.g. replay_shell_native(pin)."""
    reports = shell_reports()
    all_rows = [row for report in reports for row in cast(list[dict[str, object]], report['rows'])]
    packets = [bytearray(), bytearray()]
    for document in sorted({cast(str, row['path']) for row in all_rows}):
        streams = recover_sample(document, pin)
        for packet, fragment in zip(packets, shell_document_fragments(document, streams, reports), strict=True):
            packet.extend(fragment)
    summaries = []
    for index, (report, packet) in enumerate(zip(reports, packets, strict=True)):
        (ROOT / f'target/simulation-shell-native-{index}.bin').write_bytes(packet)
        rows = cast(list[dict[str, object]], report['rows'])
        attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
        summaries.append(dict(evidence=report['evidence'], streams=len(rows), logical_bytes=sum(cast(int, row['bytes']) for row in rows),
                         arguments=len(attempts), accepted=sum(cast(bool, attempt['success']) for attempt in attempts), packet_bytes=len(packet), packet_sha256=digest(packet)))
    shell_baseline('native-reference.json', summaries)


def shell_document_fragments(document: str, streams: dict[str, bytes], reports: list[dict[str, object]]) -> list[bytes]:
    """Keep the old empty-group checkpoint distinct from the populated grammar; e.g. shell_document_fragments(document, streams, reports)."""
    fragments: list[bytes] = []
    for index, report in enumerate(reports):
        selected = [row for row in cast(list[dict[str, object]], report['rows']) if row['path'] == document]
        fragments.append(b''.join(replay_shell_row(row, streams, index == 1) for row in selected))
    return fragments


def run_shell_checks(pin: Path) -> unittest.TestResult:
    """Keep all six original tests beside fresh native/hash/binding checks; e.g. run_shell_checks(pin)."""
    from functools import partial, update_wrapper
    import test_cosworks_shell_manager
    import test_cosworks_shell_group
    replay = update_wrapper(partial(replay_shell_native, pin), replay_shell_native)
    checks = (verify_shell_sources, verify_shell_bindings, verify_shell_owned, verify_shell_literal_oracles, verify_shell_header_oracles, replay)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for module in (test_cosworks_shell_manager, test_cosworks_shell_group):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(module))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Isolate all reference imports on an immutable copy; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-shell-manager-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_shell_checks(pin)
        readers = ('cosworks_shell_manager_layout', 'cosworks_shell_group_layout', 'swarchive', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
