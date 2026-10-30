"""Freeze complete OUT syntax and independent edits through actual readers (#114)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest
import zlib

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
sys.path.insert(0, str(FEATURE.parent / 'compound-files'))
import freeze_compound as compound
from owned_reports import owned_cases, transfer


def sha(raw: bytes) -> str:
    """Bind persisted input and expected bytes to one immutable baseline."""
    return hashlib.sha256(raw).hexdigest()


def words(values: list[int]) -> bytes:
    """Use explicit portable unsigned patterns for offsets and optional indices."""
    return b''.join(struct.pack('<I', value & 0xffffffff) for value in values)


def span(raw: bytes) -> bytes:
    """Frame complete byte regions without host-sized integers."""
    return words([len(raw)]) + raw


def report(name: str, value: object) -> None:
    """Require review intent before replacing baseline facts."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def oracle(parsed: dict) -> bytes:
    """Retain every line and every ordered field occurrence, including duplicate symbols."""
    raw = words([len(parsed['lines'])])
    for line in parsed['lines']:
        raw += words([line['stream_offset'], line.get('field_index', -1), line.get('legend_field_index', -1)])
        raw += span(line['content']) + span(line['ending'])
        raw += words([len(line['nul_positions']), *line['nul_positions']])
    raw += words([len(parsed['fields'])])
    for field in parsed['fields']:
        raw += field_oracle(field)
    raw += words([int(parsed['declared_unit_system'] == 'SI'), parsed['unit_declaration_line'] if parsed['unit_declaration_line'] is not None else -1])
    return raw


def field_oracle(field: dict) -> bytes:
    """Keep exact Decimal sign, digits and exponent without binary floating conversion."""
    raw = words([field['line_index'], field['stream_offset']])
    for key in ('label', 'symbol', 'numeric_lexeme'):
        raw += span(field[key].encode('ascii'))
    numeric = field['value'].as_tuple()
    raw += words([numeric.sign]) + span(bytes(numeric.digits)) + span(str(numeric.exponent).encode())
    raw += words([len(field['legends'])])
    for legend in field['legends']:
        raw += words([legend['line_index'], legend['stream_offset']])
        raw += span(str(legend['value']).encode()) + span(legend['description'].encode('ascii'))
    return raw


def inspect(raw: bytes) -> tuple[bytes, dict]:
    """Reconstruct through complete original line bytes before trusting projections."""
    import simulationout
    parsed = simulationout.read(raw)
    assert raw[4:] == b''.join(line['content'] + line['ending'] for line in parsed['lines'])
    assert parsed['end'] == len(raw)
    expected = oracle(parsed)
    facts = dict(bytes=len(raw), sha256=sha(raw), oracle_sha256=sha(expected), lines=len(parsed['lines']), fields=len(parsed['fields']), legends=sum(len(field['legends']) for field in parsed['fields']), nul_positions=[index for index, value in enumerate(raw) if value == 0 and index >= 4], declared_unit_system=parsed['declared_unit_system'])
    return expected, facts


def golden(raw: bytes, index: int = 0) -> bytes:
    """Replace one literal token independently, retaining every other report byte."""
    import simulationout
    parsed = simulationout.read(raw)
    if not parsed['fields']:
        return raw
    first = parsed['fields'][index]
    content = parsed['lines'][first['line_index']]['content']
    lexeme = first['numeric_lexeme'].encode('ascii')
    local = content.rfind(lexeme)
    assert local > content.rfind(b'=') >= 0
    start = first['stream_offset'] - 4 + local
    body = raw[4:]
    result = transfer(body[:start] + b'-2.500D+1' + body[start + len(lexeme):])
    inspect(result)
    return result


def field_goldens(raw: bytes) -> bytes:
    """Define an independent literal replacement for every ordered occurrence."""
    import simulationout
    count = len(simulationout.read(raw)['fields'])
    return words([count]) + b''.join(span(golden(raw, index)) for index in range(count))


def freeze_owned() -> None:
    """Pin complete independently built syntax cases and literal edit goldens."""
    packets, facts = [], []
    for label, raw in owned_cases():
        expected, record = inspect(raw)
        edited = golden(raw)
        packets.append(span(raw) + span(expected) + span(edited) + field_goldens(raw))
        facts.append(dict(label=label, **record, golden_sha256=sha(edited)))
    packet = zlib.compress(b'OUTO' + words([len(packets)]) + b''.join(packets), 9)
    path = ROOT / 'tests/fixtures/simulation-out/owned.bin'
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(packet)
    assert path.read_bytes() == packet
    report('owned-reference.json', dict(fixture_sha256=sha(packet), cases=facts))


def freeze_native() -> None:
    """Use actual complete carriers and real paired master/GEN context for totals."""
    from cfb import CFB
    import simulationmaster
    import simulationgen
    import simulationout
    catalog = json.loads((FEATURE.parent / 'compound-files/native-reference.json').read_text())['records']
    inputs = [item for item in catalog if any(stream['path'] == 'OUT' for stream in item['streams'])]
    assert len(inputs) == 1
    item = inputs[0]
    carrier = Path(item['path']).read_bytes()
    assert sha(carrier) == item['sha256']
    document = CFB(Path(item['path']))
    raw, mas, gen = document.read('OUT'), document.read('MAS'), document.read('GEN')
    expected, facts = inspect(raw)
    parsed = simulationout.read(raw)
    paired = simulationgen.read(gen, master=simulationmaster.read(mas))
    totals = {field['symbol']: int(field['value']) for field in parsed['fields'] if field['symbol'] in ('NUME', 'NUMNP')}
    assert totals == dict(NUME=paired['prefix']['element_count'], NUMNP=paired['prefix']['node_count'])
    edited = golden(raw)
    report('native-reference.json', dict(path=item['path'], carrier_sha256=sha(carrier), master_sha256=sha(mas), gen_sha256=sha(gen), totals=totals, **facts, golden_sha256=sha(edited)))
    if '--export' in sys.argv:
        packet = b'OUTN' + words([1]) + b''.join(map(span, (carrier, raw, mas, gen, expected, edited))) + field_goldens(raw)
        (ROOT / 'target/simulation-out-native.bin').write_bytes(packet)
        report('native-probe-reference.json', dict(bytes=len(packet), sha256=sha(packet)))


def schema() -> None:
    """Pin the complete report clause separately from binary mesh semantics."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_text().replace('\r\n', '\n')
    begin = text.index('### 14.106 ')
    end = text.index('### 14.107 ', begin)
    raw = (text[begin:end].rstrip() + '\n').encode()
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('schema-reference.json', dict(issue=114, clauses=['14.106'], sha256=sha(raw)))


def run() -> bool:
    """Freeze the actual Source modules before any reference import occurs."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-out-source-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_owned, freeze_native, schema))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cfb', 'simulationtransfer', 'simulationout', 'simulationmaster', 'simulationgen'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
