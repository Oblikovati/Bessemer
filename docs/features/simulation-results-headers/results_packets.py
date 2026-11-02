"""Owned input and complete field packets for Results hierarchy operations (#164)."""
from collections.abc import Iterator
from pathlib import Path
from itertools import product
import struct
import sys
from native_carriers import digest
from results_evidence import FEATURE, results_baseline
from results_literals import results_root_blocks, results_category_blocks, results_plot_blocks
from results_owned import RESULTS_VERSIONS
from results_projection import results_projection
from results_history import results_history


def results_owned_cases() -> Iterator[tuple[int, int, bytes]]:
    """Enumerate the persisted owned profile order; e.g. list(results_owned_cases())."""
    builders = (results_root_blocks, results_category_blocks, results_plot_blocks)
    for level, version, rotation, null_only in product(range(3), RESULTS_VERSIONS, range(5), (False, True)):
        yield level, version, b''.join(fragment for _, fragment in builders[level](version, rotation, null_only))


def results_native_cases(packet: bytes) -> Iterator[tuple[int, int, bytes]]:
    """Keep every accepted native argument without dropping failed input controls; e.g. list(results_native_cases(packet))."""
    position = 0
    while position < len(packet):
        level, version, accepted, size = struct.unpack_from('<IIII', packet, position)
        position += 16
        raw = packet[position:position + size]
        assert len(raw) == size, (position, size, len(packet))
        position += size
        if accepted: yield level, version, raw
    assert position == len(packet), (position, len(packet))


def verify_results_packets() -> None:
    """Lock every owned/native complete input and primitive projection; e.g. verify_results_packets()."""
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-results-headers'
    folder.mkdir(exist_ok=True)
    cases = list(results_owned_cases())
    inputs = b''.join(struct.pack('<IIII', level, version, 1, len(raw)) + raw for level, version, raw in cases)
    projection = b''.join(results_projection(level, version, raw) for level, version, raw in cases)
    native_path = FEATURE.parents[2] / 'target/simulation-results-headers-native.bin'
    history = b''.join(results_history(level, version, raw) for level, version, raw in cases)
    packets = {'owned.bin': inputs, 'projection.bin': projection, 'history.bin': history}
    summaries = persist_results_packets(folder, packets)
    summaries.update(results_native_packets(native_path))
    results_baseline('operation-packets-reference.json', summaries)


def persist_results_packets(folder: Path, packets: dict[str, bytes]) -> dict[str, object]:
    """Freeze complete owned packet bytes only on explicit initialization; e.g. persist_results_packets(folder, packets)."""
    summaries: dict[str, object] = {}
    for name, raw in packets.items():
        path = folder / name
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected complete unchanged Results packet'
        summaries[name] = dict(bytes=len(raw), sha256=digest(raw))
    return summaries


def results_native_packets(native_path: Path) -> dict[str, object]:
    """Export every complete native field and edited-history context; e.g. results_native_packets(native_path)."""
    cases = list(results_native_cases(native_path.read_bytes()))
    packets = {'projection': b''.join(results_projection(*case) for case in cases),
               'history': b''.join(results_history(*case) for case in cases)}
    for name, raw in packets.items():
        native_path.with_name(f'simulation-results-headers-native-{name}.bin').write_bytes(raw)
    return {f'native-{name}': dict(bytes=len(raw), sha256=digest(raw)) for name, raw in packets.items()}
