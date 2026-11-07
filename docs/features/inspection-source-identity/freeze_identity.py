"""Owned compatibility-checksum inputs independent of any Bessemer implementation (#195)."""
from pathlib import Path
import hashlib
import struct
import xml.etree.ElementTree as ET

RFC_VECTORS: tuple[tuple[bytes, str], ...] = (
    (b'', 'd41d8cd98f00b204e9800998ecf8427e'),
    (b'a', '0cc175b9c0f1b6a831c399e269772661'),
    (b'abc', '900150983cd24fb0d6963f7d28e17f72'),
    (b'message digest', 'f96b697d7cb7938d525a2f31aaf161d0'),
    (b'abcdefghijklmnopqrstuvwxyz', 'c3fcd3d76192e4007dfb496cca67e13b'),
    (b'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789', 'd174ab98d277d9f5a5611c2c9f419d9f'),
    (b'1234567890' * 8, '57edf4a22be3c955ac49da2e2107b67a'),
)
LARGE_LENGTHS = (255, 256, 257, 511, 512, 513, 1023, 1024, 1025,
                 4095, 4096, 4097, 65535, 65536, 65537)
FIXTURES = Path(__file__).resolve().parents[3] / 'tests/fixtures/inspection-source-identity'


def patterned_input(length: int) -> bytes:
    """Exercise binary values at every byte/block position; e.g. patterned_input(65)."""
    return bytes((index * 73 + length * 19 + (index >> 3)) & 255 for index in range(length))


def owned_inputs() -> list[tuple[str, bytes]]:
    """Build reproducible boundary and degenerate inputs; e.g. owned_inputs()[0]."""
    cases = [(f'rfc-{index}', raw) for index, (raw, _) in enumerate(RFC_VECTORS)]
    cases.extend((f'pattern-{length}', patterned_input(length)) for length in range(130))
    cases.extend((f'pattern-{length}', patterned_input(length)) for length in LARGE_LENGTHS)
    for length in (55, 56, 63, 64, 65, 127, 128, 129):
        cases.extend(((f'zeros-{length}', bytes(length)), (f'ones-{length}', b'\xff' * length)))
    cases.extend((('ascending', bytes(range(256))), ('descending', bytes(reversed(range(256)))),
                  ('four-byte-cycles', bytes(range(256)) * 4), ('million-a', b'a' * 1_000_000)))
    return cases


def build_packet() -> bytes:
    """Freeze hashlib expectations with explicit framing; e.g. build_packet()[:4]."""
    cases = owned_inputs()
    packet = bytearray(b'ISMD' + struct.pack('<I', len(cases)))
    for label, raw in cases:
        name = label.encode()
        packet.extend(struct.pack('<II', len(name), len(raw)))
        packet.extend(hashlib.md5(raw).digest())
        packet.extend(name)
        packet.extend(raw)
    return bytes(packet)


def manifest_rows() -> str:
    """Expose independently reviewable payload locations; e.g. manifest_rows().splitlines()[1]."""
    rows = ['index\tlabel\tpayload-offset\tpayload-length\tmd5\tsha256']
    offset = 8
    for index, (label, raw) in enumerate(owned_inputs()):
        offset += 24 + len(label.encode())
        fields = (str(index), label, str(offset), str(len(raw)),
                  hashlib.md5(raw).hexdigest(), hashlib.sha256(raw).hexdigest())
        rows.append('\t'.join(fields))
        offset += len(raw)
    return '\n'.join(rows) + '\n'


def metadata_inputs() -> list[tuple[str, str | None, str | None]]:
    """Distinguish absent, empty and exact stored spelling; e.g. metadata_inputs()[0]."""
    checksum = RFC_VECTORS[2][1]
    sizes = (None, '', '3', '03', '+3', '3.0', ' 3', '3 ', '٣',
             '18446744073709551616', '\t3', '3\n')
    checksums = (None, '', checksum, checksum.upper(), checksum + ' ',
                 checksum[:-1], 'not-a-hash', '0' * 32)
    cases = [(f'size-{index}', value, checksum) for index, value in enumerate(sizes)]
    cases.extend((f'checksum-{index}', '3', value) for index, value in enumerate(checksums))
    cases.extend((('both-absent', None, None), ('both-empty', '', '')))
    return cases


def metadata_xml(size: str | None, checksum: str | None) -> bytes:
    """Build a scoped document with an unopened path; e.g. metadata_xml('3', None)."""
    root = ET.Element('{http://www.extensiblecad.com}SOLIDWORKSINSPECTION')
    container = ET.SubElement(ET.SubElement(root, 'INSPECTION_SHEET'), 'DOCUMENT')
    document = ET.SubElement(container, 'DOCUMENT', Uid='D', SourcePath='Z:/never-open/from-stored-path.pdf')
    if size is not None:
        document.set('ModelSize', size)
    if checksum is not None:
        document.set('ModelMD5HashData', checksum)
    return ET.tostring(root, encoding='utf-8')


def metadata_packet() -> bytes:
    """Freeze exact-text comparison expectations; e.g. metadata_packet()[:4]."""
    cases = metadata_inputs()
    packet = bytearray(b'ISMC' + struct.pack('<I', len(cases)))
    for label, size, checksum in cases:
        xml = metadata_xml(size, checksum)
        name = label.encode()
        flags = bytes((size == '3', checksum == RFC_VECTORS[2][1]))
        packet.extend(struct.pack('<II', len(name), len(xml)) + flags + name + xml)
    return bytes(packet)


if __name__ == '__main__':
    FIXTURES.mkdir(parents=True, exist_ok=True)
    (FIXTURES / 'vectors.bin').write_bytes(build_packet())
    (FIXTURES / 'vectors.tsv').write_text(manifest_rows(), encoding='utf-8', newline='\n')
    (FIXTURES / 'metadata.bin').write_bytes(metadata_packet())
