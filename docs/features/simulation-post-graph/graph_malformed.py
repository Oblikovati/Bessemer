"""Actual bounded graph count mutations and exhaustive owned extent rejection (#165)."""
from typing import cast
import importlib
import json
import struct
from native_carriers import digest
from graph_evidence import FEATURE, READER, graph_baseline
from graph_owned import graph_owned_cases
from graph_packets import persist_graph_packets


def graph_mutations(raw: bytes, spans: list[dict[str, object]]) -> list[bytes]:
    """Mutate actual character-count words and arbitrary raw code units; e.g. graph_mutations(raw, spans)."""
    candidates: set[bytes] = set()
    for span in spans:
        label, start, end = cast(str, span['label']), cast(int, span['offset']), cast(int, span['end'])
        if label.endswith('.character_count'):
            for word in (0, 1, 2, 3, 1023, 1024, 1025, 2147483647, 4294967295):
                candidates.add(raw[:start] + struct.pack('<I', word) + raw[end:])
        if label.endswith('.utf16') and end > start:
            candidates.add(raw[:end - 1] + bytes([raw[end - 1] ^ 255]) + raw[end:])
    candidates.discard(raw)
    return sorted(candidates)


def verify_graph_malformed() -> None:
    """Retain all actual successful reinterpretations and full-outcome hashes; e.g. verify_graph_malformed()."""
    packet = bytearray()
    outcomes: list[dict[str, object]] = []
    parse = importlib.import_module(READER).parse
    for version, _, _, raw in graph_owned_cases():
        parsed = parse(raw, version)
        for mutated in graph_mutations(raw, parsed['spans']):
            result = parse(mutated, version)
            packet.extend(struct.pack('<III', version, int(result['success']), len(mutated)) + mutated)
            outcomes.append(dict(version=version, bytes=len(mutated), sha256=digest(mutated), result=result))
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-post-graph'
    packets = persist_graph_packets(folder, {'malformed.bin': bytes(packet)})
    graph_baseline('malformed-reference.json', dict(cases=len(outcomes),
        accepted=sum(cast(bool, cast(dict[str, object], row['result'])['success']) for row in outcomes),
        complete_outcomes_sha256=digest(json.dumps(outcomes, sort_keys=True).encode()), packets=packets))


def verify_graph_extents() -> None:
    """Reject every proper prefix and appended byte through the actual reader; e.g. verify_graph_extents()."""
    parse = importlib.import_module(READER).parse
    prefixes = 0
    contexts = 0
    for version, _, _, raw in graph_owned_cases():
        for end in range(len(raw)):
            assert not parse(raw[:end], version)['success'], (version, end)
            prefixes += 1
        assert not parse(raw + b'\0', version)['success'], version
        contexts += 1
    assert (prefixes, contexts) == (90630, 270), (prefixes, contexts)
