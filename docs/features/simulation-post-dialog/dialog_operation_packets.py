"""Complete owned/native dialog operation packets with explicit helper/caller qualifiers (#166)."""
from pathlib import Path
from collections.abc import Iterator
import struct
import sys
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import dialog_projection, dialog_history


def accepted_dialog_native_cases(packet: bytes) -> Iterator[tuple[int, int, bytes]]:
    """Preserve accepted helper arguments while keeping all failure/caller flags in the input packet; e.g. list(accepted_dialog_native_cases(packet))."""
    offset = 0
    while offset < len(packet):
        family, version, accepted, _, size = struct.unpack_from('<IIIII', packet, offset)
        offset += 20
        raw = packet[offset:offset + size]
        assert len(raw) == size, (offset, size, len(packet))
        offset += size
        if accepted: yield family, version, raw
    assert offset == len(packet), (offset, len(packet))


def persist_dialog_packets(folder: Path, packets: dict[str, bytes]) -> dict[str, object]:
    """Require complete immutable operation expectations outside explicit initialization; e.g. persist_dialog_packets(folder, packets)."""
    folder.mkdir(exist_ok=True)
    summaries: dict[str, object] = {}
    for name, raw in packets.items():
        path = folder / name
        if '--write' in sys.argv: path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected complete unchanged dialog operation packet'
        summaries[name] = dict(bytes=len(raw), sha256=digest(raw))
    return summaries


def verify_dialog_operation_packets() -> None:
    """Freeze every owned complete input, typed field and ordered edit-history expectation; e.g. verify_dialog_operation_packets()."""
    cases = [(family, version, raw) for family, version, _, raw in dialog_owned_cases()]
    inputs = b''.join(struct.pack('<IIIII', family, version, 1, int(version >= 700335 if family == 0 else version < 700335), len(raw)) + raw
                      for family, version, raw in cases)
    projection = b''.join(dialog_projection(*case) for case in cases)
    history = b''.join(dialog_history(*case) for case in cases)
    folder = FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog'
    summaries = persist_dialog_packets(folder, {'owned.bin': inputs, 'projection.bin': projection, 'history.bin': history})
    summaries.update(dialog_native_operation_packets())
    dialog_baseline('operation-packets-reference.json', summaries)


def dialog_native_operation_packets() -> dict[str, object]:
    """Export all original successful helpers separately from failed/unresolved complete inputs; e.g. dialog_native_operation_packets()."""
    folder = FEATURE.parents[2] / 'target'
    packet = (folder / 'simulation-post-dialog-native.bin').read_bytes()
    cases = list(accepted_dialog_native_cases(packet))
    assert len(cases) == 396, len(cases)
    exports = {'projection': b''.join(dialog_projection(*case) for case in cases),
               'history': b''.join(dialog_history(*case) for case in cases)}
    for name, raw in exports.items(): (folder / f'simulation-post-dialog-native-{name}.bin').write_bytes(raw)
    return {'native-' + name: dict(bytes=len(raw), sha256=digest(raw)) for name, raw in exports.items()}
