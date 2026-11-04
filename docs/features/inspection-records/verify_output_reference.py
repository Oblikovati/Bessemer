"""Check actual Rust outputs with original Source and independent Expat byte spans (#193)."""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
from typing import BinaryIO, Iterator, NamedTuple, TypedDict
import copy
import json
import os
import re
import struct
import unittest
import xml.etree.ElementTree as ET
from xml.parsers.expat import ParserCreate
import inspectionrecords

WORKSPACE = Path(__file__).resolve().parents[3]
FIXTURES = (
    'inspection-records/ordered.xml', 'inspection-records/load-projections.xml',
    'standalone-inspection/historical.xml', 'standalone-inspection/current-0.xml',
    'standalone-inspection/current-1.xml', 'inspection-records/single-quotes.xml',
)
ROOT_NAME = 'http://www.extensiblecad.com}SOLIDWORKSINSPECTION'
SHEET_PATH = (ROOT_NAME, 'INSPECTION_SHEET')
GROUP_PATH = SHEET_PATH + ('PackageOptions', 'IXCharacteristics', 'IXGroupProxy')
CHARACTERISTIC_PATH = GROUP_PATH + ('IXCharacteristic',)
RECORD_PATHS = frozenset((SHEET_PATH, SHEET_PATH + ('DOCUMENT', 'DOCUMENT'),
                         GROUP_PATH, CHARACTERISTIC_PATH,
                         CHARACTERISTIC_PATH + ('Measurements', 'Measurement'),
                         CHARACTERISTIC_PATH + ('CustomProperties', 'CustomProperty')))
START_TAG = re.compile(rb'''<(?:[^'">]|"[^"]*"|'[^']*')*>''', re.S)
ATTRIBUTE = re.compile(rb'''([^\s=/>]+)\s*=\s*(["'])(.*?)\2''', re.S)
REPLACEMENT = 'Bessemer Ω<&"\'\n\t\r'
ENCODED_REPLACEMENTS = {
    b'"': 'Bessemer Ω&lt;&amp;&quot;\'&#10;&#9;&#13;'.encode(),
    b"'": 'Bessemer Ω&lt;&amp;"&apos;&#10;&#9;&#13;'.encode(),
}


class ElementView(TypedDict):
    element: ET.Element


class GroupView(ElementView):
    characteristics: list[ElementView]


class SheetView(ElementView):
    documents: list[ElementView]
    groups: list[GroupView]


class InspectionView(TypedDict):
    xml_bytes: bytes
    tree: ET.Element
    sheets: list[SheetView]


class AttributePin(NamedTuple):
    slot: int
    local: str
    start: int
    end: int
    quote: bytes


class ExpatRecordSpans:
    def __init__(self, raw: bytes) -> None:
        self.raw = raw
        self.stack: list[str] = []
        self.rows: list[list[AttributePin]] = []
        self.parser = ParserCreate(namespace_separator='}')
        self.parser.StartElementHandler = self.start
        self.parser.EndElementHandler = self.end
        self.parser.Parse(raw, True)

    def start(self, name: str, attributes: dict[str, str]) -> None:
        self.stack.append(name)
        if tuple(self.stack) in RECORD_PATHS:
            self.rows.append(self.attribute_pins(self.parser.CurrentByteIndex))

    def end(self, name: str) -> None:
        actual = self.stack.pop()
        assert actual == name, f'closing element {name}; expected {actual}'

    def attribute_pins(self, position: int) -> list[AttributePin]:
        tag = START_TAG.match(self.raw, position)
        assert tag is not None, f'byte {position}; expected complete Expat-selected start tag'
        ordinary = [match for match in ATTRIBUTE.finditer(tag.group())
                    if match[1] != b'xmlns' and not match[1].startswith(b'xmlns:')]
        return [AttributePin(slot, match[1].decode(), position + match.start(3),
                             position + match.end(3), match[2])
                for slot, match in enumerate(ordinary) if b':' not in match[1]]


def group_elements(group: GroupView) -> list[ET.Element]:
    elements = [group['element']]
    for characteristic in group['characteristics']:
        element = characteristic['element']
        elements.append(element)
        elements.extend(element.findall('Measurements/Measurement'))
        elements.extend(element.findall('CustomProperties/CustomProperty'))
    return elements


def sheet_elements(sheet: SheetView) -> list[ET.Element]:
    elements = [sheet['element']]
    elements.extend(document['element'] for document in sheet['documents'])
    for group in sheet['groups']:
        elements.extend(group_elements(group))
    return elements


@dataclass(frozen=True)
class SourceCase:
    raw: bytes
    view: InspectionView
    elements: list[ET.Element]
    positions: dict[int, int]
    pins: list[list[AttributePin]]


def source_case(relative: str) -> SourceCase:
    raw = (WORKSPACE / 'tests/fixtures' / relative).read_bytes()
    view: InspectionView = inspectionrecords.read(raw)
    positions = {id(element): index for index, element in enumerate(view['tree'].iter())}
    elements = [element for sheet in view['sheets'] for element in sheet_elements(sheet)]
    elements.sort(key=lambda element: positions[id(element)])
    pins = ExpatRecordSpans(raw).rows
    assert len(elements) == len(pins), f'{relative}; expected Source/Expat record counts to agree'
    assert len({id(element) for element in elements}) == len(elements)
    return SourceCase(raw, view, elements, positions, pins)


def read_exact(stream: BinaryIO, length: int) -> bytes:
    raw = stream.read(length)
    assert len(raw) == length, f'{len(raw)} bytes; expected {length} complete packet bytes'
    return raw


def packet_rows(path: Path, expected_count: int) -> Iterator[tuple[tuple[int, int, int], bytes]]:
    with path.open('rb') as stream:
        assert read_exact(stream, 4) == b'IRX1'
        count, = struct.unpack('<I', read_exact(stream, 4))
        assert count == expected_count, f'{count} outputs; expected {expected_count}'
        for _ in range(count):
            case, record, attribute, length = struct.unpack('<IIII', read_exact(stream, 16))
            assert length <= 1_000_000, f'XML length {length}; expected at most 1000000'
            yield (case, record, attribute), read_exact(stream, length)
        assert stream.read(1) == b'', 'packet suffix; expected exact EOF'


def expected_pins(cases: list[SourceCase]) -> dict[tuple[int, int, int], AttributePin]:
    return {(case_index, record_index, pin.slot): pin
            for case_index, case in enumerate(cases)
            for record_index, pins in enumerate(case.pins) for pin in pins}


def check_output(case: SourceCase, record_index: int, pin: AttributePin, xml: bytes) -> None:
    expected_raw = case.raw[:pin.start] + ENCODED_REPLACEMENTS[pin.quote] + case.raw[pin.end:]
    assert xml == expected_raw, f'record {record_index} attribute {pin.local}; expected exact independent byte patch'
    actual: InspectionView = inspectionrecords.read(xml)
    expected = copy.deepcopy(case.view['tree'])
    position = case.positions[id(case.elements[record_index])]
    selected = list(expected.iter())[position]
    assert list(selected.attrib)[pin.slot] == pin.local
    selected.set(pin.local, REPLACEMENT)
    assert ET.tostring(actual['tree']) == ET.tostring(expected), 'Source graph differs; expected only selected attribute change'
    assert actual['xml_bytes'] == xml


def packet_pair() -> tuple[Path, Path]:
    return (Path(os.environ['BESSEMER_INSPECTION_EXPORT_FIRST']),
            Path(os.environ['BESSEMER_INSPECTION_EXPORT_SECOND']))


class OutputReferenceTests(unittest.TestCase):
    def test_every_actual_rust_output_matches_original_source_and_independent_byte_patch(self) -> None:
        cases = [source_case(relative) for relative in FIXTURES]
        pins = expected_pins(cases)
        self.assertEqual(len(pins), 1349)
        seen: set[tuple[int, int, int]] = set()
        for identity, xml in packet_rows(packet_pair()[0], len(pins)):
            self.assertNotIn(identity, seen)
            pin = pins[identity]
            case, record, _ = identity
            check_output(cases[case], record, pin, xml)
            seen.add(identity)
        self.assertEqual(seen, set(pins))
        print(json.dumps(dict(actual_rust_outputs=len(seen), original_source_reader='inspectionrecords.read', independent_oracle='Expat spans and literal escaped replacement')))

    def test_fresh_process_packets_are_byte_identical(self) -> None:
        first, second = packet_pair()
        with first.open('rb') as left, second.open('rb') as right:
            while True:
                block = left.read(1024 * 1024)
                self.assertEqual(block, right.read(len(block)))
                if not block:
                    self.assertEqual(right.read(1), b'')
                    break
