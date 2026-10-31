"""Complete owned/native shell field and history packets without first-success dispatch (#159)."""
from pathlib import Path
from typing import cast
from native_carriers import recover_sample, digest
from owned_shell_property import owned_shell_properties
from shell_property_evidence import shell_baseline, shell_reports, FEATURE
from shell_property_projection import shell_property_projection
from shell_property_history import shell_property_history

ROOT = FEATURE.parents[2]


def shell_packet_pair(kind: int, version: int, raw: bytes, presence: list[bool] | None) -> tuple[bytes, bytes]:
    """Require an actual complete selected context before projecting both operations; e.g. shell_packet_pair(0, 891106, raw, presence)."""
    from cosworks_shell_property_layout import parse
    from cosworks_embedded_const_surface import archive_reader
    parsed = parse(raw, version, 'Shell', archive_reader=archive_reader, layer_presence=presence)
    assert parsed['success'] and parsed['end'] == len(raw), parsed
    return (shell_property_projection(kind, version, raw, parsed, presence),
            shell_property_history(kind, version, raw, parsed, presence))


def verify_owned_shell_packets() -> None:
    """Retain all independent contexts as CI-owned full primitive/history expectations; e.g. verify_owned_shell_packets()."""
    import sys
    packets = [bytearray(), bytearray()]
    cases = owned_shell_properties()
    for case in cases:
        pair = shell_packet_pair(case['kind'], case['version'], case['raw'], case['layer_presence'])
        for packet, fragment in zip(packets, pair, strict=True): packet.extend(fragment)
    references: dict[str, object] = {}
    for name, raw in zip(('projection', 'history'), packets, strict=True):
        path = ROOT / 'tests/fixtures/simulation-shell-property' / f'{name}.bin'
        if '--write' in sys.argv:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(raw)
        assert path.read_bytes() == raw
        references[name] = dict(bytes=len(raw), sha256=digest(raw))
    shell_baseline('owned-packets-reference.json', dict(cases=len(cases), packets=references))


def native_shell_packet_row(kind: int, row: dict[str, object], streams: dict[str, bytes]) -> tuple[bytes, bytes, int]:
    """Keep every accepted original argument without dropping duplicate-version evidence; e.g. native_shell_packet_row(0, row, streams)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
    packets = [bytearray(), bytearray()]
    count = 0
    for attempt in cast(list[dict[str, object]], row['attempts']):
        if not attempt['success']: continue
        pair = shell_packet_pair(kind, cast(int, attempt['version']), raw, None)
        for packet, fragment in zip(packets, pair, strict=True): packet.extend(fragment)
        count += 1
    return bytes(packets[0]), bytes(packets[1]), count


def verify_native_shell_packets(pin: Path) -> None:
    """Freshly recover all original native identities and accepted field/history arguments; e.g. verify_native_shell_packets(pin)."""
    rows = [(kind, row) for kind, report in enumerate(shell_reports()[:2]) for row in cast(list[dict[str, object]], report['rows'])]
    packets = [bytearray(), bytearray()]
    count = 0
    for document in sorted({cast(str, row['path']) for _, row in rows}):
        streams = recover_sample(document, pin)
        for kind, row in (item for item in rows if item[1]['path'] == document):
            fields, history, accepted = native_shell_packet_row(kind, row, streams)
            packets[0].extend(fields)
            packets[1].extend(history)
            count += accepted
    assert count == 251, count
    references: dict[str, object] = {}
    for name, raw in zip(('projection', 'history'), packets, strict=True):
        (ROOT / f'target/simulation-shell-property-native-{name}.bin').write_bytes(raw)
        references[name] = dict(bytes=len(raw), sha256=digest(raw))
    shell_baseline('native-packets-reference.json', dict(accepted_arguments=count, packets=references))
