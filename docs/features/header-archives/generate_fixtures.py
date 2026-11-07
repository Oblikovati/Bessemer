"""Independent clause 11.8 vectors; run with the reference solidworks directory on PYTHONPATH."""
from pathlib import Path
import struct
import modelheader

ROOT = Path(__file__).resolve().parents[3]
DESTINATION = ROOT / 'tests/fixtures/headers'


class FixtureArchive:
    def __init__(self):
        self.output = bytearray()
        self.classes = {}
        self.next_index = 1

    def pack(self, shape, *values):
        self.output.extend(struct.pack('<' + shape, *values))

    def text(self, value):
        encoded = value.encode('latin1')
        assert len(encoded) < 255
        self.pack('B', len(encoded))
        self.output.extend(encoded)

    def object(self, name):
        if name in self.classes:
            self.pack('H', self.classes[name] | 0x8000)
        else:
            self.pack('HHH', 0xffff, 0, len(name))
            self.output.extend(name.encode('ascii'))
            self.classes[name] = self.next_index
            self.next_index += 1
        index = self.next_index
        self.next_index += 1
        return index

    def null(self):
        self.pack('H', 0)

    def strings(self, texts):
        self.object('su_CStringArray')
        self.pack('H', len(texts))
        for text in texts:
            self.text(text)

    def external(self, version):
        self.object('moExtObject_c')
        for text in ('C:\\fixture\\', 'fixture.SLDPRT'):
            if version >= 0xfa7:
                self.object('moCStringHandle_c')
            self.text(text)
        self.pack('HBI', 0x1234, 0x56, 0xfedcba98)
        if 0x1427 < version < 0x146c:
            self.pack('B', 0xab)
        for threshold, text in ((0x1fb3, 'unknown'), (0x23d9, 'imported'), (0x2f82, 'pdm')):
            if version > threshold:
                self.text(text)
        if version >= 0x3330:
            self.pack('B', 0xcd)
        self.pack('BIii', 0xfe, 0xffffffff, -1, -2)
        self.pack('i', -3)
        self.text('Default')
        self.pack('ii', -4, -5)
        if version >= 0xfd2:
            self.pack('I', 0x80000001)
        if version > 0x3f12:
            self.pack('i', -6)


def header(version, embedded=None):
    archive = FixtureArchive()
    archive.object('moHeader_c')
    if version > 0x371b:
        archive.pack('i', -7)
    if embedded is None:
        archive.null()
    else:
        archive.strings(['fixture', modelheader.VERSION_MARKER + str(embedded)])
    body = version if embedded is None else embedded & 0xffffffff
    archive.strings(['Company'])
    archive.null()
    if body >= 0xf1:
        archive.pack('I', 0xaabbccdd)
    archive.pack('i', -24)
    archive.null()
    if body > 0x96:
        archive.pack('I', 102)
    if body > 0xd8:
        archive.external(body) if body >= 0xf18 else archive.null()
    for threshold, shape, value in ((0x30b, 'I', 101), (0x4e4, 'i', -58), (0x500, 'i', -68),
                                    (0x505, 'I', 123), (0x530, 'i', -60)):
        if body > threshold:
            archive.pack(shape, value)
    for threshold, shape, value in ((0x859, 'i', -1), (0x91e, 'i', -2), (0xaa1, 'H', 0),
                                    (0xb8c, 'i', 1), (0xcf6, 'I', 0xfedcba98),
                                    (0xe5c, 'I', 88), (0xf3a, 'H', 0), (0x128f, 'i', -9),
                                    (0x1303, 'i', -64)):
        if version > threshold:
            archive.pack(shape, value)
    if version > 0x130e:
        archive.pack('i10Q', -1, *range(0x3ff0000000000000, 0x3ff000000000000a))
    if version > 0x2783:
        archive.pack('3i', -1, 0, 1)
    if version > 0x2bc2:
        archive.pack('I', 0xffffffff)
    if version > 0x2fa3:
        archive.pack('6i', -1, -2, -3, -4, -5, -6)
    if version > 0x336f:
        archive.pack('i', -8)
    if version > 0x3b0a:
        archive.pack('6i', -7, -8, -9, -10, -11, -12)
    for threshold, value in ((0x3b1f, -13), (0x3db8, -14), (0x3f01, -15)):
        if version > threshold:
            archive.pack('i', value)
    return bytes(archive.output)


def generate():
    thresholds = [0x96, 0xd8, 0xf0, 0x30b, 0x4e4, 0x500, 0x505, 0x530, 0x859, 0x91e,
                  0xaa1, 0xb8c, 0xcf6, 0xe5c, 0xf17, 0xf3a, 0xfa6, 0xfd1, 0x128f,
                  0x1303, 0x130e, 0x1427, 0x146b, 0x1fb3, 0x23d9, 0x2783, 0x2bc2,
                  0x2f82, 0x2fa3, 0x332f, 0x336f, 0x371b, 0x3b0a, 0x3b1f, 0x3db8, 0x3f01, 0x3f12]
    cases = [(f'version-{v:04x}', v, None) for t in thresholds for v in (t, t + 1)]
    cases += [('override-old', 19000, 0x96), ('override-modern', 9000, 19000), ('override-signed', 19000, -1)]
    rust = ['pub const CASES: &[(&str, u32, &[u8])] = &[']
    for name, version, embedded in cases:
        payload = header(version, embedded)
        decoded = modelheader.read_data(payload, version)
        assert decoded['next_id'] == -24
        (DESTINATION / (name + '.bin')).write_bytes(payload)
        rust.append(f'    ("{name}", {version}, include_bytes!("../fixtures/headers/{name}.bin")),')
    rust.append('];\n')
    (ROOT / 'tests/support/header_cases.rs').write_text('\n'.join(rust), encoding='utf-8', newline='\n')
    print(f'Generated and reference-verified {len(cases)} header vectors')


if __name__ == '__main__':
    generate()
