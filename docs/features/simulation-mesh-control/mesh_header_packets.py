"""Complete independent MeshControl header field and edited-history packets (#161)."""
from pathlib import Path
from typing import cast
import sys
from native_carriers import digest, recover_sample
from owned_mesh_headers import mesh_header_spans, owned_mesh_headers
from mesh_control_evidence import FEATURE, mesh_baseline, mesh_reports
from shell_header_packets import header_case_packets

ROOT = FEATURE.parents[2]


def require_mesh_header_words(profile: int, version: int, raw: bytes, words: list[int]) -> None:
    """Check writer literals with a declared compatible-reader control; e.g. require_mesh_header_words(1, 0, raw, words)."""
    from cosworks_mesh_control_header import parse
    assert profile in (0, 1), f'{profile}; expected reader 0 or current writer 1'
    control = version if profile == 0 else 840013
    parsed = parse(raw, control)
    assert parsed['success'] and parsed['values'] == words and parsed['end'] == len(raw), (profile, version, parsed)
    assert parsed['reconstruction_sha256'] == digest(raw)
    destinations = [0x08] + ([0x50] if control > 800027 else []) + ([0x60] if control >= 840013 else [])
    assert parsed['destinations'] == [hex(value) for value in destinations]


def mesh_header_case_packets(profile: int, version: int, raw: bytes, words: list[int]) -> tuple[bytes, bytes]:
    """Reuse exact signed-header framing with independent Mesh destinations and acceptance; e.g. mesh_header_case_packets(0, version, raw, words)."""
    return header_case_packets(profile, version, raw, words, acceptance=require_mesh_header_words,
                               span_factory=mesh_header_spans, edited_path='word08')


def verify_owned_mesh_header_packets() -> None:
    """Freeze all 80 independent complete field/span/edit/history contexts; e.g. verify_owned_mesh_header_packets()."""
    packets = [bytearray(), bytearray()]
    cases = owned_mesh_headers()
    for case in cases:
        fragments = mesh_header_case_packets(case['profile'], case['version'], case['raw'], case['words'])
        for packet, fragment in zip(packets, fragments, strict=True): packet.extend(fragment)
    folder = ROOT / 'tests/fixtures/simulation-mesh-control'
    references: dict[str, object] = {}
    for name, raw in zip(('projection', 'history'), packets, strict=True):
        path = folder / f'header-{name}.bin'
        if '--write' in sys.argv:
            folder.mkdir(parents=True, exist_ok=True)
            path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected complete unchanged owned packet'
        references[name] = dict(bytes=len(raw), sha256=digest(raw))
    mesh_baseline('header-owned-packets.json', dict(cases=len(cases), packets=references,
                  writer_scope='Static unconditional three-word layout, using an explicit compatible-reader control; no native writer execution.'))


def native_mesh_header_row(row: dict[str, object], streams: dict[str, bytes]) -> tuple[bytes, bytes]:
    """Retain every original success and failure before projecting complete operations; e.g. native_mesh_header_row(row, streams)."""
    from cosworks_mesh_control_header import parse
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    packets = [bytearray(), bytearray()]
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = parse(raw, version)
        assert dict(version=version, **parsed) == expected, (row['stream'], version)
        if not parsed['success']: continue
        fragments = mesh_header_case_packets(0, version, raw, cast(list[int], parsed['values']))
        for packet, fragment in zip(packets, fragments, strict=True): packet.extend(fragment)
    return bytes(packets[0]), bytes(packets[1])


def replay_native_mesh_header_packets(pin: Path) -> None:
    """Recover and project all 361 accepted header arguments without dropping the failures; e.g. replay_native_mesh_header_packets(pin)."""
    rows = cast(list[dict[str, object]], mesh_reports()[0]['rows'])
    packets = [bytearray(), bytearray()]
    for document in sorted({cast(str, row['path']) for row in rows}):
        streams = recover_sample(document, pin)
        fragments = [native_mesh_header_row(row, streams) for row in rows if row['path'] == document]
        for index, packet in enumerate(packets): packet.extend(b''.join(fragment[index] for fragment in fragments))
    references: dict[str, object] = {}
    for name, raw in zip(('projection', 'history'), packets, strict=True):
        (ROOT / f'target/simulation-mesh-control-native-header-{name}.bin').write_bytes(raw)
        references[name] = dict(bytes=len(raw), sha256=digest(raw))
    mesh_baseline('header-native-packets.json', dict(accepted_arguments=361, original_arguments=392, rejected=31, packets=references))
