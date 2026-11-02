"""Actual complete Results grammar outcomes for targeted physical mutations (#164)."""
import json
import struct
from typing import cast
from native_carriers import digest
from results_evidence import FEATURE, results_baseline
from results_native import actual_results
from results_packets import results_owned_cases, persist_results_packets


def results_mutations(raw: bytes, parsed: dict[str, object]) -> list[bytes]:
    """Mutate each real count/text frame with sufficient retained suffix bytes; e.g. results_mutations(raw, parsed)."""
    candidates: set[bytes] = set()
    for span in cast(list[dict[str, object]], parsed['spans']):
        label, start, end = cast(str, span['label']), cast(int, span['offset']), cast(int, span['end'])
        if label == 'text_list.count' or label.startswith('text'):
            for word in (0, 1, 2, 3, 4, 16, 100000, 100001, 1048577, 4294967295):
                candidates.add(raw[:start] + struct.pack('<I', word) + raw[start + 4:])
        if label.startswith('text') and label != 'text_list.count' and end > start + 4:
            candidates.add(raw[:end - 1] + bytes([raw[end - 1] ^ 255]) + raw[end:])
    candidates.discard(raw)
    return sorted(candidates)


def verify_results_malformed() -> None:
    """Retain every actual acceptance decision and full-outcome digest; e.g. verify_results_malformed()."""
    packet = bytearray()
    outcomes: list[dict[str, object]] = []
    for level, version, raw in results_owned_cases():
        parsed = actual_results(raw, version, level)
        for mutated in results_mutations(raw, parsed):
            result = actual_results(mutated, version, level)
            packet.extend(struct.pack('<IIII', level, version, int(cast(bool, result['success'])), len(mutated)) + mutated)
            outcomes.append(dict(level=level, version=version, bytes=len(mutated), sha256=digest(mutated), result=result))
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-results-headers'
    summaries = persist_results_packets(folder, {'malformed.bin': bytes(packet)})
    results_baseline('malformed-reference.json', dict(cases=len(outcomes),
        accepted=sum(cast(bool, cast(dict[str, object], row['result'])['success']) for row in outcomes),
        complete_outcomes_sha256=digest(json.dumps(outcomes, sort_keys=True).encode()), packets=summaries))


def verify_results_extents() -> None:
    """Reject every owned proper prefix and appended byte through actual readers; e.g. verify_results_extents()."""
    prefixes = 0
    contexts = 0
    for level, version, raw in results_owned_cases():
        for end in range(len(raw)):
            assert not actual_results(raw[:end], version, level)['success'], (level, version, end)
            prefixes += 1
        assert not actual_results(raw + b'\0', version, level)['success'], (level, version)
        contexts += 1
    assert (prefixes, contexts) == (7961, 660), (prefixes, contexts)
