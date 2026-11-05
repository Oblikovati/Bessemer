"""Bind every Rust native outcome to its unchanged C-1923 stream and identifier count (#179)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import re
import struct

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]


def native_dimension_outcomes(path: Path) -> list[tuple[int, int, int]]:
    """Reject partial or duplicate native outcome rows; e.g. native_dimension_outcomes(log)."""
    text = path.read_text()
    rows = [tuple(map(int, row)) for row in re.findall(r'^DIMENSION_NATIVE\t(\d+)\t(\d+)\t(\d+)$', text, re.M)]
    assert len(rows) == 906 and [row[0] for row in rows] == list(range(906)), len(rows)
    assert 'test result: ok. 1 passed; 0 failed; 0 ignored' in text, path
    return cast(list[tuple[int, int, int]], rows)


def retained_dimension_packet() -> bytes:
    """Require the complete established native carrier packet; e.g. retained_dimension_packet()."""
    raw = (ROOT / 'target/resolved-native-probe.bin').read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    assert digest == '4e8885cfedf8d4b1d14806fd2643f42f7d7fb4768e004530087df58b8a16797f', digest
    assert raw[:4] == b'BRF1' and struct.unpack_from('<I', raw, 4)[0] == 906, raw[:8]
    return raw


def dimension_packet_case(raw: bytes, offset: int) -> tuple[int, str, bytes, int]:
    """Keep packet order separate from historical report order; e.g. dimension_packet_case(packet, 8)."""
    version, *lengths = struct.unpack_from('<6I', raw, offset)
    offset += 24
    parts: list[bytes] = []
    for length in lengths:
        parts.append(raw[offset:offset + length])
        offset += length
    return version, parts[0].decode(), parts[4], offset


def dimension_stream_key(label: str) -> tuple[str, str]:
    """Normalize the pinned Windows sample identity once; e.g. dimension_stream_key(label)."""
    location, configuration = label.split(': ', 1)[1].rsplit('::', 1)
    root = 'C:/Users/Public/Documents/SOLIDWORKS/SOLIDWORKS 2026/'
    location = location.replace('\\', '/')
    assert location.startswith(root), f'{location}; expected a sample below{root}'
    return location[len(root):].casefold(), configuration + '-ResolvedFeatures'


def bind_dimension_carrier(raw: bytes, offset: int, expected_rows: dict[tuple[str, str], dict[str, object]], counts: tuple[int, int, int]) -> tuple[dict[str, object], int]:
    """Compare original identity, full bytes and per-stream promoted-word counts; e.g. bind_dimension_carrier(packet, offset, rows, counts)."""
    version, label, features, offset = dimension_packet_case(raw, offset)
    expected = expected_rows[dimension_stream_key(label)]
    assert label.startswith(f'{counts[0]}: '), label
    digest = hashlib.sha256(features).hexdigest()
    assert digest == expected['sha256'] and len(features) == expected['decoded_bytes'], label
    gain = cast(int, expected['decoded_gain'])
    assert gain % 4 == 0 and counts[2] == gain // 4, f'{label}/{counts[2]}; expected original C-1923 per-stream word count {gain // 4}'
    return dict(index=counts[0], version=version, path=expected['path'], stream=expected['stream'], sha256=digest,
                configuration_dimensions=counts[1], feature_dimensions=counts[2], original_promoted_identifier_bytes=gain), offset


def verify_native_dimension_bindings() -> None:
    """Record exact per-stream bindings without migrating original accounting; e.g. verify_native_dimension_bindings()."""
    report_path = FEATURE / 'reports/part-dimension-redundant-id-replay-evidence.json'
    report_raw = report_path.read_bytes()
    pins = json.loads((FEATURE / 'report-pins.json').read_bytes())
    assert hashlib.sha256(report_raw).hexdigest() == pins[report_path.name], report_path
    report = json.loads(report_raw)
    expected_rows = {(row['path'].replace('\\', '/').casefold(), row['stream']): row for row in report['files']}
    assert len(expected_rows) == 906, f'{len(expected_rows)}; expected906 unique original stream identities'
    outcomes = native_dimension_outcomes(ROOT / 'target/dimension-state-final-native.log')
    packet = retained_dimension_packet()
    rows: list[dict[str, object]] = []
    offset = 8
    for counts in outcomes:
        row, offset = bind_dimension_carrier(packet, offset, expected_rows, counts)
        rows.append(row)
    assert offset == len(packet) and len(rows) == 906, offset
    result = dict(original_evidence='C-1923', cases=len(rows), source_report_sha256=pins[report_path.name],
        feature_dimensions=sum(cast(int, row['feature_dimensions']) for row in rows),
        configuration_dimensions=sum(cast(int, row['configuration_dimensions']) for row in rows), rows=rows)
    (FEATURE / 'native-bindings-verification.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8', newline='\n')


if __name__ == '__main__':
    verify_native_dimension_bindings()
