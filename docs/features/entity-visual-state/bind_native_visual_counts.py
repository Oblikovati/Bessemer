"""Bind fresh Rust physical visual groups to every unchanged C-1959 carrier (#181)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import re
import sys

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'dimension-stored-state'))
from bind_native_dimension_counts import retained_dimension_packet, dimension_packet_case, dimension_stream_key


def native_visual_outcomes(path: Path) -> list[tuple[int, int, int]]:
    """Reject incomplete, duplicate or unordered native result rows; e.g. native_visual_outcomes(path)."""
    text = path.read_text()
    rows = [tuple(map(int, match)) for match in re.findall(r'^VISUAL_NATIVE\t(\d+)\t(\d+)\t(\d+)$', text, re.M)]
    assert len(rows) == 906, f'{len(rows)}; expected all906 complete native visual outcomes'
    assert [row[0] for row in rows] == list(range(906)), 'native indices; expected each original packet index exactly once in order'
    assert 'test result: ok. 1 passed; 0 failed;' in text, 'native result; expected terminal complete one-test success'
    return cast(list[tuple[int, int, int]], rows)


def bind_visual_carrier(packet: bytes, position: int, expected_rows: dict[tuple[str, str], dict[str, object]], counts: tuple[int, int, int]) -> tuple[dict[str, object], int]:
    """Bind complete stream bytes and physical counts without adopting old byte accounting; e.g. bind_visual_carrier(packet, position, rows, counts)."""
    version, label, features, position = dimension_packet_case(packet, position)
    expected = expected_rows[dimension_stream_key(label)]
    assert label.startswith(f'{counts[0]}: '), f'{label}; expected packet index{counts[0]}'
    digest = hashlib.sha256(features).hexdigest()
    assert digest == expected['sha256'] and len(features) == expected['end'], f'{label}/{digest}/{len(features)}; expected complete original stream {expected}'
    assert counts[2] == expected['visual_records'], f'{label}/{counts[2]}; expected original physical read count{expected["visual_records"]}'
    return dict(index=counts[0], version=version, path=expected['path'], stream=expected['stream'], sha256=digest,
        configuration_visuals=counts[1], feature_visuals=counts[2]), position


def verify_native_visual_bindings() -> None:
    """Record all906 identity/count bindings beside immutable original evidence; e.g. verify_native_visual_bindings()."""
    report_path = FEATURE / 'reports/part-entity-visual-rgb-replay-evidence.json'
    report_raw = report_path.read_bytes()
    pins = cast(dict[str, str], json.loads((FEATURE / 'report-pins.json').read_bytes()))
    assert hashlib.sha256(report_raw).hexdigest() == pins[report_path.name], f'{report_path}; expected the immutable C-1959 report'
    report = cast(dict[str, object], json.loads(report_raw))
    expected_rows = {(cast(str, row['path']).replace('\\', '/').casefold(), cast(str, row['stream'])): row
        for row in cast(list[dict[str, object]], report['files'])}
    assert len(expected_rows) == 906, f'{len(expected_rows)}; expected906 unique original visual streams'
    outcomes = native_visual_outcomes(ROOT / 'target/entity-visual-native-carriers.log')
    packet, position = retained_dimension_packet(), 8
    rows: list[dict[str, object]] = []
    for counts in outcomes:
        row, position = bind_visual_carrier(packet, position, expected_rows, counts)
        rows.append(row)
    assert position == len(packet) and len(rows) == 906, f'{position}/{len(packet)}/{len(rows)}; expected the complete906-case packet'
    assert sum(cast(int, row['feature_visuals']) for row in rows) == 127773, 'visual total; expected127773 original physical reads'
    result = dict(original_evidence='C-1959', cases=906, source_report_sha256=pins[report_path.name], coverage_credit=0,
        configuration_visuals=sum(cast(int, row['configuration_visuals']) for row in rows), feature_visuals=127773, rows=rows)
    (FEATURE / 'native-bindings-verification.json').write_text(json.dumps(result, indent=2) + '\n', newline='\n')


if __name__ == '__main__':
    verify_native_visual_bindings()
