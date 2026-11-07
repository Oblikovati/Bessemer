"""Independent graph resource denominators and fully supplied character-count caps (#165)."""
from typing import cast
import importlib
import struct
from native_carriers import digest
from graph_evidence import FEATURE, READER, graph_baseline
from graph_literals import graph_blocks
from graph_owned import graph_owned_cases
from graph_packets import graph_native_cases, persist_graph_packets
from graph_projection import graph_fields


def graph_budget_case(version: int, raw: bytes) -> bytes:
    """Measure full physical fields, declared characters and maximum text payload bytes; e.g. graph_budget_case(890110, raw)."""
    parsed = importlib.import_module(READER).parse(raw, version)
    fields = graph_fields(version, raw)
    strings = cast(list[dict[str, object]], parsed['strings'])
    counts = [cast(int, row['count']) for row in strings]
    return (struct.pack('<II', version, len(raw)) + raw
            + struct.pack('<III', len(fields), sum(counts), 2 * max(counts, default=0)))


def verify_graph_resources() -> None:
    """Lock every owned/native exact resource requirement, including empty zeros; e.g. verify_graph_resources()."""
    owned = b''.join(graph_budget_case(version, raw) for version, _, _, raw in graph_owned_cases())
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-post-graph'
    summaries = persist_graph_packets(folder, {'budgets.bin': owned})
    native_path = FEATURE.parents[2] / 'target/simulation-post-graph-native.bin'
    native = b''.join(graph_budget_case(*case) for case in graph_native_cases(native_path.read_bytes()))
    native_path.with_name('simulation-post-graph-native-budgets.bin').write_bytes(native)
    summaries['native-budgets'] = dict(bytes=len(native), sha256=digest(native))
    graph_baseline('resource-reference.json', summaries)


def graph_cap_image(label: str, count: int) -> bytes:
    """Provide every declared UTF-16 code unit and the full graph suffix; e.g. graph_cap_image('textb0', 1024)."""
    blocks = graph_blocks(890110, 2, True)
    replacements = {label + '.character_count': struct.pack('<i', count),
                    label + '.utf16': b'\0\xd8' * max(0, count)}
    return b''.join(replacements.get(name, fragment) for name, fragment in blocks)


def verify_graph_character_caps() -> None:
    """Exercise all four real string positions at 1024/1025 with complete payloads; e.g. verify_graph_character_caps()."""
    packet = bytearray()
    rows: list[dict[str, object]] = []
    for label in ('textb0', 'textb8', 'textc0', 'textd0'):
        for count in (-1, 1024, 1025):
            raw = graph_cap_image(label, count)
            parsed = importlib.import_module(READER).parse(raw, 890110)
            accepted = count == 1024
            assert parsed['success'] == accepted, (label, count, parsed['success'])
            fields = len(graph_fields(890110, raw)) if accepted else 0
            assert not accepted or fields == 79, (label, count, fields)
            packet.extend(struct.pack('<III', 890110, int(accepted), len(raw)) + raw)
            rows.append(dict(label=label, count=count, accepted=accepted, fields=fields, bytes=len(raw), sha256=digest(raw)))
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-post-graph'
    summaries = persist_graph_packets(folder, {'character-caps.bin': bytes(packet)})
    graph_baseline('character-caps-reference.json', dict(controls=rows, packets=summaries))
