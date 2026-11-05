"""Freeze or verify the independently constructed HOV1 storage corpus (#229)."""
import argparse
import hashlib
import json
from pathlib import Path
from writer_pages import fixture_packet, image_matrix

ROOT = Path(__file__).resolve().parents[3]
PACKET = ROOT / 'tests/fixtures/database-overflow/cases.bin'
REFERENCE = Path(__file__).with_name('fixture-reference.json')


def reference_record(packet: bytes) -> dict[str, object]:
    """Describe source scope and complete fixture identity; e.g. reference_record(fixture_packet())."""
    images = image_matrix()
    return {
        'issue': 229, 'format': 'HOV1', 'cases': len(images),
        'bytes': len(packet), 'sha256': hashlib.sha256(packet).hexdigest(),
        'cursor_wrapped_cases': sum(image.cursor_wrapped for image in images),
        'extents': sorted({image.extent for image in images}),
        'basis': 'Static installed writer14600 and caller17340; owned Python construction, no native execution',
        'native_producer_acceptance': False,
        'extent_65536_scope': 'Conditional word-truncation arithmetic only; runtime reachability unproved',
    }


def freeze_or_verify(write: bool) -> None:
    """Require explicit writes and otherwise compare every byte; e.g. freeze_or_verify(False)."""
    packet = fixture_packet()
    reference = reference_record(packet)
    if write:
        PACKET.parent.mkdir(parents=True, exist_ok=True)
        PACKET.write_bytes(packet)
        REFERENCE.write_text(json.dumps(reference, indent=2) + '\n', encoding='utf-8')
        return
    assert PACKET.read_bytes() == packet, f'{PACKET}: expected exact independent HOV1 packet'
    assert json.loads(REFERENCE.read_text(encoding='utf-8')) == reference
    print(json.dumps(reference))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--write', action='store_true')
    freeze_or_verify(parser.parse_args().write)
