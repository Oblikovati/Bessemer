"""Freeze actual classification/topology readers and complete paired native evidence (#118)."""
from pathlib import Path
from collections import Counter
import json
import struct
import sys
import tempfile
import unittest
import zlib

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'simulation-images'))
import freeze_images as shared
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from owned_elements import element, selected, legacy_cases, topology_cases
span, words, sha = shared.span, shared.words, shared.sha


def report(name: str, value: object) -> None:
    """Require explicit review intent before replacing reference facts."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def classification(raw: bytes) -> bytes:
    """Check actual complete-record retention and serialize every declared numeric view."""
    import simulationelementkind as reader
    result = reader.read(raw)
    assert result['raw'] == raw and result['engineering_name'] is None and not result['field_semantics_complete']
    return words([result[key] for key in ('selector_family', 'accessor_node_count', 'selector_variant',
                                         'stored_code', 'selected_code', 'selected_signed')])


def topology(raw: bytes, bound: int | None) -> bytes:
    """Keep exact consumer slot order and ordered invalid IDs, never geometry claims."""
    import simulationtopology as reader
    result = reader.solid_ten_node(raw, node_count=bound)
    assert result['raw'] == raw and not result['averaging_executed'] and not result['geometry_complete']
    expected = words(result['corner_node_ids'])
    for edge in result['edge_nodes']:
        expected += words([edge['slot'], edge['node_id'], *edge['endpoint_slots'], *edge['endpoint_node_ids']])
    return expected + words([len(result['invalid_node_ids']), *result['invalid_node_ids'], int(result['upper_bound_checked'])])


def freeze_selectors() -> None:
    """Exhaust all 256 cubed unsigned selector inputs against independent closed forms."""
    import simulationelementkind as reader
    raw = bytearray(element(0, 0, 0))
    codes = bytearray()
    for index in range(256 ** 3):
        family, count, variant = index >> 16, (index >> 8) & 255, index & 255
        raw[20], raw[23], raw[24] = family, count, variant
        actual = reader.read(raw)['selected_code']
        assert actual == selected(family, count, variant), (family, count, variant)
        codes.append(255 if actual == 0xffffffff else actual)
    packet = zlib.compress(b'SEL1' + codes, 9)
    fixture = ROOT / 'tests/fixtures/simulation-elements/selectors.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(packet)
    assert fixture.read_bytes() == packet
    report('selector-reference.json', dict(cases=len(codes), sentinel_encoding=255, fixture_sha256=sha(packet), codes_sha256=sha(codes)))


def freeze_owned() -> None:
    """Freeze independent full-record replacements and every source topology/predicate view."""
    legacy, cases = legacy_cases(), topology_cases()
    beam, beam_count = owned_beam_packet()
    packet = b'ELO1' + owned_legacy_packet(legacy) + owned_topology_packet(cases) + beam
    fixture = ROOT / 'tests/fixtures/simulation-elements/owned.bin'
    compressed = zlib.compress(packet, 9)
    if '--write' in sys.argv:
        fixture.write_bytes(compressed)
    assert fixture.read_bytes() == compressed
    report('owned-reference.json', dict(legacy_cases=len(legacy), topology_cases=len(cases), beam_cases=beam_count, fixture_sha256=sha(compressed)))


def owned_legacy_packet(cases: list[tuple[bytes, int, int, bytes]]) -> bytes:
    """Bind every literal two-byte replacement to the actual explicit-context projection."""
    import simulationelementkind as kinds
    packet = words([len(cases)])
    for raw, version, analysis, golden in cases:
        parsed = kinds.project_legacy(raw, version=version, analysis_code=analysis)
        assert parsed['raw'] == golden and parsed['engineering_name'] is None and not parsed['native_io_executed']
        packet += span(raw) + words([version, analysis]) + span(golden) + span(classification(raw))
    return packet


def owned_topology_packet(cases: list[tuple[bytes, int | None]]) -> bytes:
    """Preserve all bound-presence states and their actual named-consumer associations."""
    packet = words([len(cases)])
    for raw, bound in cases:
        packet += span(raw) + words([int(bound is not None), bound or 0]) + span(topology(raw, bound))
    return packet


def owned_beam_packet() -> tuple[bytes, int]:
    """Keep the stored-code predicate independent of activity, count and selector output."""
    import simulationtopology as shapes
    predicates = [element(f, n, v, stored=s, activity=a) for f in (0, 2, 4) for n in (0, 2, 255)
                  for v in (0, 255) for s in (0, 37, 38, 65535) for a in (0, 1, 255)]
    packet = words([len(predicates)])
    for raw in predicates:
        expected = raw[20] == 2 and struct.unpack_from('<H', raw, 26)[0] != 37
        assert shapes.beam_first_record_predicate(raw) == expected
        packet += span(raw) + words([int(expected)])
    return packet, len(predicates)


def inspect_native(master_raw: bytes, gen_raw: bytes) -> tuple[bytes, dict[str, object]]:
    """Resolve native rows through complete actual MAS/GEN context before projecting fields."""
    import simulationmaster
    import simulationgen
    import simulationtopology
    parsed = simulationgen.read(gen_raw, master=simulationmaster.read(master_raw))
    elements = parsed['elements']['raw']
    bound = parsed['prefix']['node_count']
    rows = [elements[i:i+140] for i in range(0, len(elements), 140)]
    expected, histogram, associations, eligible = inspected_rows(rows, bound)
    mismatch = sum(n for (stored, chosen), n in histogram.items() if stored != chosen)
    beam = bool(rows) and simulationtopology.beam_first_record_predicate(rows[0])
    return words([len(rows)]) + expected + words([int(beam)]), dict(elements=len(rows), node_count=bound, differing_stored_codes=mismatch,
        topology_records=eligible, associations=eligible*6, associations_sha256=sha(associations), first_record_beam=beam,
        histogram=[dict(stored=s, selected=c, count=n) for (s,c),n in sorted(histogram.items())])


def inspected_rows(rows: list[bytes], bound: int) -> tuple[bytes, Counter[tuple[int, int]], bytes, int]:
    """Project all complete stored rows before any aggregate corpus facts are derived."""
    expected = bytearray()
    histogram = Counter()
    associations = bytearray()
    eligible = 0
    for raw in rows:
        kind = classification(raw)
        values = struct.unpack('<6I', kind)
        histogram[(values[3], values[4])] += 1
        active = raw[0] != 0 and raw[20] == 4 and raw[23] == 10
        expected += kind + words([int(active)])
        if active:
            projection = topology(raw, bound)
            expected += span(projection)
            associations += projection
            eligible += 1
    return bytes(expected), histogram, bytes(associations), eligible


def freeze_native() -> None:
    """Retain all three installed complete pairs and their exact inspected field projections."""
    packets, facts = [], []
    projected = bytearray(b'ELP1')
    for source, carrier, master, gen in shared.native_pairs():
        expected, counts = inspect_native(master, gen)
        projected += flattened_projection(expected)
        facts.append(dict(path=source['path'], carrier_sha256=sha(carrier), master_sha256=sha(master), gen_sha256=sha(gen), oracle_sha256=sha(expected), **counts))
        packets.append(span(master) + span(gen) + span(expected))
    report('native-reference.json', dict(files=len(facts), records=facts))
    if '--verify-rust' in sys.argv:
        actual = (ROOT / 'target/simulation-elements-native-projections.bin').read_bytes()
        assert actual == projected
        report('rust-projection-reference.json', dict(bytes=len(actual), sha256=sha(actual), records=25106, associations=87852))
    if '--export' in sys.argv:
        packet = b'ELN1' + words([len(packets)]) + b''.join(packets)
        (ROOT / 'target/simulation-elements-native.bin').write_bytes(packet)
        report('native-probe-reference.json', dict(bytes=len(packet), sha256=sha(packet)))


def flattened_projection(oracle: bytes) -> bytes:
    """Flatten actual reader projections in the independently emitted Rust packet order."""
    count = struct.unpack_from('<I', oracle)[0]
    offset = 4
    parts = []
    for _ in range(count):
        parts.append(oracle[offset:offset+24])
        present = struct.unpack_from('<I', oracle, offset+24)[0]
        offset += 28
        if present:
            size = struct.unpack_from('<I', oracle, offset)[0]
            parts.append(oracle[offset+4:offset+4+size])
            offset += 4+size
    assert offset+4 == len(oracle)
    return b''.join(parts)


def schema() -> None:
    """Retain both complete consumer clauses without adding engineering interpretations."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_text().replace('\r\n', '\n')
    raw = (text[text.index('### 14.112 '):text.index('### 14.114 ')].rstrip() + '\n').encode()
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('schema-reference.json', dict(issue=118, clauses=['14.112', '14.113'], sha256=sha(raw)))


def run() -> bool:
    """Import only complete copied actual readers with recorded hashes and verified origins."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-elements-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        shared.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_selectors, freeze_owned, freeze_native, schema))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cfb', 'simulationmaster', 'simulationgen', 'simulationelementkind', 'simulationtopology'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
