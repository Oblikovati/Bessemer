"""Owned/native complete graph field and ordered-history packet locks (#165)."""
from collections.abc import Iterator
from pathlib import Path
import struct
import sys
from native_carriers import digest
from graph_evidence import FEATURE, graph_baseline
from graph_owned import graph_owned_cases
from graph_projection import graph_projection, graph_history


def graph_native_cases(packet: bytes) -> Iterator[tuple[int, bytes]]:
    """Retain every accepted original argument in packet order; e.g. list(graph_native_cases(packet))."""
    position = 0
    while position < len(packet):
        version, accepted, size = struct.unpack_from('<III', packet, position)
        position += 12
        raw = packet[position:position + size]
        assert len(raw) == size, (position, size, len(packet))
        position += size
        if accepted: yield version, raw
    assert position == len(packet), (position, len(packet))


def persist_graph_packets(folder: Path, packets: dict[str, bytes]) -> dict[str, object]:
    """Require complete fixture equality outside explicit initialization; e.g. persist_graph_packets(folder, packets)."""
    summaries: dict[str, object] = {}
    folder.mkdir(exist_ok=True)
    for name, raw in packets.items():
        path = folder / name
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected complete unchanged graph operation packet'
        summaries[name] = dict(bytes=len(raw), sha256=digest(raw))
    return summaries


def verify_graph_packets() -> None:
    """Freeze all owned inputs and complete owned/native field/history oracles; e.g. verify_graph_packets()."""
    owned = [(version, raw) for version, _, _, raw in graph_owned_cases()]
    packets = {'owned.bin': b''.join(struct.pack('<III', version, 1, len(raw)) + raw for version, raw in owned),
               'projection.bin': b''.join(graph_projection(*case) for case in owned),
               'history.bin': b''.join(graph_history(*case) for case in owned)}
    summaries = persist_graph_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-graph', packets)
    native_path = FEATURE.parents[2] / 'target/simulation-post-graph-native.bin'
    native = list(graph_native_cases(native_path.read_bytes()))
    exports = {'projection': b''.join(graph_projection(*case) for case in native),
               'history': b''.join(graph_history(*case) for case in native)}
    for name, raw in exports.items():
        native_path.with_name(f'simulation-post-graph-native-{name}.bin').write_bytes(raw)
        summaries['native-' + name] = dict(bytes=len(raw), sha256=digest(raw))
    graph_baseline('operation-packets-reference.json', summaries)
