"""Bind 64-bit MFC map framing and code units to actual reference reads (#99)."""
from functools import partial, update_wrapper
from pathlib import Path
from types import FrameType
from typing import cast
import hashlib
import json
import struct
import sys
import tempfile
import unittest
from typing import Protocol

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from primitive_inventory import PrimitiveRead, ReferenceCursor, observe_primitives, verify_primitive_coverage


class ConfigurationDocument(Protocol):
    def names(self) -> list[str]: ...
    def read(self, name: str) -> bytes: ...


class SelectedConfigurationDocument:
    """Replace only the owned configuration while keeping its real header and directory."""
    def __init__(self, original: ConfigurationDocument, configuration: bytes) -> None:
        self.original = original
        self.configuration = configuration

    def names(self) -> list[str]:
        return self.original.names()

    def read(self, name: str) -> bytes:
        return self.configuration if name == 'Contents/Config-0' else self.original.read(name)


def map_text(value: str, wide: bool, width: int) -> bytes:
    """Encode independent byte/code-unit strings without Unicode replacement."""
    raw = value.encode('utf-16-le', errors='surrogatepass') if wide else value.encode('latin1')
    count = len(raw) // (2 if wide else 1)
    prefix = {1: lambda: struct.pack('<B', count), 3: lambda: struct.pack('<BH', 255, count),
              7: lambda: struct.pack('<BHI', 255, 65535, count),
              15: lambda: struct.pack('<BHIQ', 255, 65535, 0xffffffff, count)}[width]()
    return (b'\xff\xfe\xff' if wide else b'') + prefix + raw


def object_tag(name: str) -> bytes:
    """Reserve the real class/object slots through normal framing; e.g. object_tag(name)."""
    return struct.pack('<HHH', 65535, 1, len(name)) + name.encode('ascii')


def map_cases() -> list[tuple[int, bytes]]:
    """Cross collection/text escape widths, encodings, duplicate keys and empty values."""
    result = []
    for count_width in (2, 6, 14):
        count = {2: struct.pack('<H', 2), 6: struct.pack('<HI', 65535, 2),
                 14: struct.pack('<HIQ', 65535, 0xffffffff, 2)}[count_width]
        for width in (1, 3, 7, 15):
            for wide in (False, True):
                key, value = ('\ud800A', 'Ω😀\udfff') if wide else ('\xffA', 'one\x00two')
                entries = map_text(key, wide, width) + map_text(value, wide, width)
                entries += map_text(key, wide, width) + map_text('', not wide, width)
                raw = object_tag('su_CMapStringToString') + count + entries
                result.append((0, raw))
    result.append((0, object_tag('su_CMapStringToString') + struct.pack('<HIQ', 65535, 0xffffffff, 0)))
    result.append((1, object_tag('moBendNoteFormat_c') + b'\x03fmt' + result[0][1]))
    return result


class StringReadSpans:
    """Observe the real string64 return metadata to project its raw code-unit spans."""
    def __init__(self, raw: bytes) -> None:
        import mfcmaps
        self.raw = raw
        self.code = mfcmaps.string64.__code__
        self.spans: list[tuple[int, int, int]] = []

    def capture(self, frame: FrameType, event: str, value: object) -> None:
        """Record only successful actual string reads from this fixture's cursor."""
        if event != 'return' or frame.f_code is not self.code or value is None:
            return
        cursor = cast(ReferenceCursor, frame.f_locals['a'])
        if cursor.b != self.raw:
            return
        _text, encoding = cast(tuple[str, dict[str, int | bool]], value)
        self.spans.append((frame.f_locals['start'] + encoding['prefix_bytes'], cursor.p,
                           2 if encoding['wide'] else 1))


def observe_map(kind: int, raw: bytes) -> tuple[bytes, dict[str, object]]:
    """Use real registered serializers and preserve the original primitive inventory."""
    import mfcmaps, sharedmodeltail
    from mfcobjects import ObjectArchive
    from swarchive import Archive
    cursor = ObjectArchive(raw, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.readers = dict(cursor.readers, su_CMapStringToString=mfcmaps.string_map,
                          moBendNoteFormat_c=sharedmodeltail.bend_format)
    observer = StringReadSpans(raw)
    previous = sys.getprofile()
    try:
        sys.setprofile(observer.capture)
        with observe_primitives(Archive, raw) as reads:
            cursor.object()
    finally:
        sys.setprofile(previous)
    assert cursor.p == len(raw)
    spans = [r for r in reads if r.end > r.start]
    verify_primitive_coverage(raw, spans)
    projected = projected_oracle(raw, spans, observer.spans)
    original = ''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\n' for r in spans).encode()
    frame = struct.pack('<BIIII', kind, len(raw), len(projected), len(original), len(cursor.map)) + raw + projected + original
    return frame, dict(kind=kind, bytes=len(raw), reads=len(spans), projected_reads=len(projected.splitlines()),
                       slots=len(cursor.map), sha256=hashlib.sha256(raw).hexdigest(), string_spans=observer.spans)


def projected_oracle(raw: bytes, reads: list[PrimitiveRead], strings: list[tuple[int, int, int]]) -> bytes:
    """Expand only Source-identified string payloads into exact endian code units."""
    result = []
    widths = {(start, end): width for start, end, width in strings}
    for row in reads:
        width = widths.get((row.start, row.end)) if row.kind == 'raw' else None
        if width is None:
            result.append(f'{row.start}\t{row.end}\t{row.kind}\t{row.raw_hex}\n')
            continue
        assert (row.end - row.start) % width == 0
        for start in range(row.start, row.end, width):
            result.append(f'{start}\t{start + width}\t{"B" if width == 1 else "H"}\t{raw[start:start + width].hex()}\n')
    return ''.join(result).encode()


def freeze_maps(write: bool) -> None:
    """Regenerate exact map graphs plus both original and projected Source inventories."""
    cases = map_cases()
    observations = [observe_map(*case) for case in cases]
    goldens = {}
    for index, label, replacement in [(0, 'latin1', 254), (1, 'utf16', 0xd801)]:
        kind, raw = cases[index]
        start, _end, width = observations[index][1]['string_spans'][0]
        edited = raw[:start] + replacement.to_bytes(width, 'little') + raw[start + width:]
        observations.append(observe_map(kind, edited))
        goldens[label + '.bin'] = raw
        goldens[label + '-edited.bin'] = edited
    packet = b'MS64' + struct.pack('<I', len(observations)) + b''.join(frame for frame, _ in observations)
    path = ROOT / 'tests/fixtures/assembly-configurations/string-maps.bin'
    path.parent.mkdir(exist_ok=True)
    if write:
        path.write_bytes(packet)
    else:
        assert path.read_bytes() == packet, 'map fixtures changed; expected exact Source regeneration'
    for name, payload in goldens.items():
        golden = path.parent / ('map-' + name)
        if write:
            golden.write_bytes(payload)
        else:
            assert golden.read_bytes() == payload, f'{name}; expected independent edit bytes'
    freeze_owned_configuration(goldens, write)
    report = dict(cases=[facts for _, facts in observations], packet_sha256=hashlib.sha256(packet).hexdigest(),
                  raw_string_projection='Actual string64 return metadata; original primitive spans retained in the packet')
    (FEATURE / 'string-map-reference.json').write_bytes((json.dumps(report, indent=2) + '\n').encode())


def inserted_configuration(original: ConfigurationDocument, string_map: bytes) -> bytes:
    """Insert a real bend-format graph at its observed nullable pointer and relocate later IDs."""
    from mfcobjects import ObjectArchive
    import modelprefix
    sys.path.insert(0, str(FEATURE.parent / 'manager-scalars'))
    sys.path.insert(0, str(FEATURE.parent / 'resolved-features'))
    from manager_tag_observation import ManagerTagObservation, relocated_tail
    raw = original.read('Contents/Config-0')
    observer = ManagerTagObservation(raw, ObjectArchive.object.__code__)
    selected: list[tuple[int, int]] = []
    def capture(frame: FrameType, event: str, value: object) -> None:
        observer.capture(frame, event, value)
        if event == 'call' and frame.f_code is observer.code and frame.f_locals.get('expect') == 'moBendNoteFormat_c':
            cursor = frame.f_locals['self']
            if cursor.b == raw:
                selected.append((cursor.p, len(cursor.map)))
    previous = sys.getprofile()
    try:
        sys.setprofile(capture)
        root, cursor = modelprefix.read(original, through_part_resolved=True)
    finally:
        sys.setprofile(previous)
    assert cursor.p == len(raw) and len(selected) == 1
    position, first_slot = selected[0]
    assert raw[position:position + 2] == bytes(2)
    inserted = object_tag('moBendNoteFormat_c') + b'\x03fmt' + string_map
    changed = relocated_tail(raw, observer.events, position, first_slot, 4)
    changed = bytearray(changed[:position] + inserted + changed[position + 2:])
    size_offset = root['lightweight_end']
    size = struct.unpack_from('<i', raw, size_offset)[0] + len(inserted) - 2
    struct.pack_into('<i', changed, size_offset, size)
    return bytes(changed)


def freeze_owned_configuration(goldens: dict[str, bytes], write: bool) -> None:
    """Bind the map to a complete real part configuration and an independent key edit."""
    from swcontainer import Document
    import modelprefix
    original = Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    facts = []
    for name in ('latin1.bin', 'latin1-edited.bin'):
        raw = inserted_configuration(original, goldens[name])
        document = SelectedConfigurationDocument(original, raw)
        root, cursor = modelprefix.read(document, through_part_resolved=True)
        assert cursor.p == len(raw) and root['_class'] == 'moPart_c'
        path = ROOT / 'tests/fixtures/assembly-configurations' / ('owned-map-' + name)
        if write:
            path.write_bytes(raw)
        else:
            assert path.read_bytes() == raw, f'{path.name}; expected unchanged complete Source configuration'
        facts.append(dict(name=path.name, bytes=len(raw), map_slots=len(cursor.map), sha256=hashlib.sha256(raw).hexdigest()))
    (FEATURE / 'owned-string-map-reference.json').write_bytes((json.dumps(facts, indent=2) + '\n').encode())


def main() -> None:
    """Run fixture replay with immutable readers and record their exact origins."""
    with tempfile.TemporaryDirectory(prefix='bessemer-string-maps-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        replay = update_wrapper(partial(freeze_maps, '--write' in sys.argv), freeze_maps)
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(replay))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE, ('mfcmaps', 'sharedmodeltail'), ('mfcobjects', 'swarchive'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'string-map-source-snapshot.json')


if __name__ == '__main__':
    main()
