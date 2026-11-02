"""Complete independent Results resource requirements and sufficient-payload caps (#164)."""
from typing import cast
import struct
from native_carriers import digest
from results_evidence import FEATURE, results_baseline
from results_native import actual_results
from results_projection import results_fields
from results_packets import results_owned_cases, results_native_cases, persist_results_packets


def results_budget_case(level: int, version: int, raw: bytes) -> bytes:
    """Measure actual complete fields, declared list entries and raw text maxima; e.g. results_budget_case(1, 800044, raw)."""
    parsed = actual_results(raw, version, level)
    fields = results_fields(level, version, raw)
    spans = cast(list[dict[str, object]], parsed['spans'])
    texts = [span for span in spans if cast(str, span['label']).startswith('text') and span['label'] != 'text_list.count']
    maximum_text = max((struct.unpack_from('<I', raw, cast(int, span['offset']))[0] for span in texts), default=0)
    entries = len(texts) if level == 1 else 0
    return struct.pack('<III', level, version, len(raw)) + raw + struct.pack('<III', len(fields), entries, maximum_text)


def verify_results_resources() -> None:
    """Freeze every owned/native exact resource denominator; e.g. verify_results_resources()."""
    owned = b''.join(results_budget_case(*case) for case in results_owned_cases())
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-results-headers'
    summaries = persist_results_packets(folder, {'budgets.bin': owned})
    native_path = FEATURE.parents[2] / 'target/simulation-results-headers-native.bin'
    native = b''.join(results_budget_case(*case) for case in results_native_cases(native_path.read_bytes()))
    native_path.with_name('simulation-results-headers-native-budgets.bin').write_bytes(native)
    summaries['native-budgets'] = dict(bytes=len(native), sha256=digest(native))
    results_baseline('resource-reference.json', summaries)


def verify_results_category_cap() -> None:
    """Require full 100,000/100,001-entry payloads on opposite sides of C-763's cap; e.g. verify_results_category_cap()."""
    packet = bytearray()
    rows = []
    for count in (100000, 100001):
        raw = struct.pack('<ii', -2147483648, count) + b'\0\0\0\0' * count
        parsed = actual_results(raw, 800044, 1)
        accepted = count == 100000
        assert parsed['success'] == accepted, (count, parsed['success'])
        if accepted: assert parsed['end'] == len(raw) and len(cast(list[object], parsed['spans'])) == count + 2
        packet.extend(struct.pack('<IIII', 1, 800044, int(accepted), len(raw)) + raw)
        rows.append(dict(count=count, accepted=accepted, bytes=len(raw), sha256=digest(raw)))
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-results-headers'
    summaries = persist_results_packets(folder, {'category-cap.bin': bytes(packet)})
    results_baseline('category-cap-reference.json', dict(controls=rows, packets=summaries))
