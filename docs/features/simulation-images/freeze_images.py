"""Freeze complete paired Simulation images and documented mesh projections (#112)."""
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
from owned_images import owned_cases
from owned_carrier import compound_study, OTHER


def sha(raw: bytes) -> str:
    """Bind every input and expected projection to immutable bytes."""
    return hashlib.sha256(raw).hexdigest()


def words(values: list[int] | tuple[int, ...]) -> bytes:
    """Retain raw 32-bit patterns, including signed reference/count values."""
    return b''.join(struct.pack('<I', value & 0xffffffff) for value in values)


def span(raw: bytes) -> bytes:
    """Use bounded portable packet framing independent of Rust representation."""
    return words([len(raw)]) + raw


def report(name: str, value: object) -> None:
    """Require explicit review intent when updating expected facts."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def master_oracle(master: dict) -> bytes:
    """Keep directory-selected rows, every pattern and every unassigned fixed region."""
    raw = words([master['version'], *master['header_words']])
    raw += words([value for pair in master['field_pairs'] for value in pair])
    raw += words(master['directory'])
    for key in ('header_padding', 'field_table_padding', 'directory_padding'):
        raw += span(master[key])
    raw += words([len(master['rows'])])
    for row in master['rows']:
        raw += words([row['record_number'], row['body_offset'], *row['fields']]) + span(row['padding'])
    return raw


def node_oracle(node: dict) -> bytes:
    """Patterns avoid silently normalizing NaNs or assigning coordinate units/frames."""
    return words([node['activity_byte'], node['body_group_tag'], node['body_group_tag_bits'], *node['coordinate_patterns']])


def element_oracle(element: dict) -> bytes:
    """Separate selected references and invalid IDs from all retained raw slots."""
    raw = words([element['activity_byte'], element['accessor_node_count'], *element['elis_slots'], *element['node_slots']])
    for key in ('node_ids', 'invalid_node_ids'):
        raw += words([len(element[key]), *element[key]])
    return raw


def gen_oracle(gen: dict) -> bytes:
    """Retain the entire framing and each documented mesh view from actual readers."""
    import simulationmesh
    raw = words([gen['prefix']['version'], *gen['prefix']['words'], gen['prefix']['late_word'], gen['auxiliary_stride'], int(gen['fallback'])])
    for key in ('opaque_header', 'opaque_middle'):
        raw += span(gen['prefix'][key])
    for key in ('capacities', 'opaque_a', 'state_a', 'state_b', 'opaque_b', 'nodes', 'elements', 'auxiliary', 'extras', 'trailer'):
        raw += words([gen[key]['stream_offset']]) + span(gen[key]['raw'])
    raw += vector_oracle(gen)
    mesh = simulationmesh.project(gen)
    raw += words([len(mesh['nodes'])]) + b''.join(map(node_oracle, mesh['nodes']))
    raw += words([len(mesh['elements'])]) + b''.join(map(element_oracle, mesh['elements']))
    return raw


def vector_oracle(gen: dict) -> bytes:
    """Distinguish absent vectors from present empty vectors at exact stream positions."""
    raw = words([len(gen['auxiliary_rows'])])
    for row, vector in zip(gen['auxiliary_rows'], gen['vectors']):
        raw += words([row['identifier'], row['vector_length']]) + span(row['raw'])
        raw += words([int(vector is not None)])
        if vector is not None:
            raw += words([vector['stream_offset']]) + span(vector['raw'])
    return raw


def inspect(master_raw: bytes, gen_raw: bytes) -> tuple[bytes, dict[str, object]]:
    """Derive GEN context from a real complete MAS parse, never a fake dictionary."""
    import simulationmaster
    import simulationgen
    master = simulationmaster.read(master_raw)
    gen = simulationgen.read(gen_raw, master=master)
    assert gen['end'] == len(gen_raw)
    oracle = span(master_oracle(master)) + span(gen_oracle(gen))
    facts = dict(master_bytes=len(master_raw), gen_bytes=len(gen_raw), master_sha256=sha(master_raw),
                 gen_sha256=sha(gen_raw), oracle_sha256=sha(oracle), version=master['version'],
                 nodes=gen['prefix']['node_count'], elements=gen['prefix']['element_count'],
                 auxiliary_stride=gen['auxiliary_stride'], auxiliary_count=len(gen['auxiliary_rows']),
                 vector_bytes=sum(len(value['raw']) for value in gen['vectors'] if value is not None))
    return oracle, facts


def freeze_owned() -> None:
    """Preserve complete owned pairs and independent byte-edit goldens."""
    packets, facts = [], []
    for label, master_raw, gen_raw in owned_cases():
        oracle, record = inspect(master_raw, gen_raw)
        goldens = edit_goldens(master_raw, gen_raw)
        carrier, carrier_golden = carrier_goldens(master_raw, gen_raw, goldens['gen_edit'])
        facts.append(dict(label=label, **record, **{key + '_sha256': sha(value) for key, value in goldens.items()}, carrier_sha256=sha(carrier), carrier_golden_sha256=sha(carrier_golden)))
        packets.append(span(master_raw) + span(gen_raw) + span(oracle) + span(goldens['master_edit']) + span(goldens['gen_edit']) + span(carrier) + span(carrier_golden))
    packet = zlib.compress(b'SIMO' + words([len(packets)]) + b''.join(packets), 9)
    fixture = ROOT / 'tests/fixtures/simulation-images/owned.bin'
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True, exist_ok=True)
        fixture.write_bytes(packet)
    assert fixture.read_bytes() == packet
    report('owned-reference.json', dict(fixture_sha256=sha(packet), cases=facts))


def edit_goldens(master_raw: bytes, gen_raw: bytes) -> dict[str, bytes]:
    """Patch literal independently reviewed fields, preserving every other byte."""
    import simulationmaster
    master = simulationmaster.read(master_raw)
    edited_master = bytearray(master_raw)
    struct.pack_into('<I', edited_master, 4 + master['rows'][69]['body_offset'] + 80 * 4, 0x12345678)
    edited_gen = bytearray(gen_raw)
    if struct.unpack_from('<i', gen_raw, 416)[0]:
        struct.pack_into('<3I', edited_gen, 1004 + 32, 0x40000000, 0x40400000, 0x40800000)
    inspect(bytes(edited_master), gen_raw)
    inspect(master_raw, bytes(edited_gen))
    return dict(master_edit=bytes(edited_master), gen_edit=bytes(edited_gen))


def carrier_goldens(master: bytes, gen: bytes, edited: bytes) -> tuple[bytes, bytes]:
    """Reopen independently built whole files, including the unrelated stream."""
    from cfb import CFB
    carriers = [compound_study(master, gen), compound_study(master, edited)]
    with tempfile.TemporaryDirectory(prefix='bessemer-owned-cwr-') as temporary:
        for raw, expected in zip(carriers, (gen, edited)):
            path = Path(temporary) / 'owned.cwr'
            path.write_bytes(raw)
            document = CFB(path)
            assert document.read('MAS') == master and document.read('GEN') == expected
            assert document.read('Other') == OTHER
            inspect(document.read('MAS'), document.read('GEN'))
    return carriers[0], carriers[1]


def native_pairs() -> list[tuple[dict, bytes, bytes, bytes]]:
    """Read all three retained CWR files through the actual copied compound reader."""
    from cfb import CFB
    catalog = json.loads((FEATURE.parent / 'compound-files/native-reference.json').read_text())
    pairs = []
    for record in catalog['records']:
        path = Path(record['path'])
        if path.suffix.lower() != '.cwr':
            continue
        raw = path.read_bytes()
        assert sha(raw) == record['sha256']
        document = CFB(path)
        pairs.append((record, raw, document.read('MAS'), document.read('GEN')))
    assert len(pairs) == 3
    return pairs


def freeze_native() -> None:
    """Capture whole carriers, paired image bytes and every expected projection."""
    packets, facts = [], []
    for original, cwr, master_raw, gen_raw in native_pairs():
        oracle, record = inspect(master_raw, gen_raw)
        facts.append(dict(path=original['path'], carrier_sha256=sha(cwr), **record))
        if '--export' in sys.argv:
            packets.append(span(cwr) + span(master_raw) + span(gen_raw) + span(oracle))
    report('native-reference.json', dict(files=len(facts), records=facts))
    if '--export' in sys.argv:
        packet = b'SIMN' + words([len(packets)]) + b''.join(packets)
        (ROOT / 'target/simulation-images-native.bin').write_bytes(packet)
        report('native-probe-reference.json', dict(bytes=len(packet), sha256=sha(packet)))


def schema() -> None:
    """Pin complete relevant clauses with platform-independent line endings."""
    path = ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md'
    text = path.read_text().replace('\r\n', '\n')
    begin = text.index('### 14.100 ')
    end = text.index('### 14.106 ', begin)
    snapshot = (text[begin:end].rstrip() + '\n').encode()
    destination = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        destination.write_bytes(snapshot)
    assert destination.read_bytes() == snapshot
    report('schema-reference.json', dict(issue=112, clauses=['14.100', '14.101', '14.102', '14.103', '14.104', '14.105'], snapshot_sha256=sha(snapshot)))


def run() -> bool:
    """Freeze module contents and origins before any reference reader is imported."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-reference-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_owned, freeze_native, schema))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cfb', 'simulationmaster', 'simulationgen', 'simulationmesh'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
