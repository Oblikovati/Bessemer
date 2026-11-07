"""Freeze complete signed node-tag/run and shared-reference readers (#122)."""
from pathlib import Path
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
from owned_groups import group_cases, independent_runs, independent_selection
span, words, sha = shared.span, shared.words, shared.sha


def report(name: str, value: object) -> None:
    """Require explicit review before changing any reference baseline."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged reviewed baseline'


def node_oracle(nodes: bytes) -> bytes:
    """Bind every signed/raw word and activity byte to actual complete record views."""
    import simulationmesh as reader
    packet = bytearray(words([len(nodes) // 52]))
    for offset in range(0, len(nodes), 52):
        raw = nodes[offset:offset + 52]
        parsed = reader.node(raw)
        bits = int.from_bytes(raw[22:24], 'little')
        signed = bits if bits < 32768 else bits - 65536
        assert parsed['raw'] == raw and parsed['active'] == bool(raw[0])
        assert (parsed['body_group_tag'], parsed['body_group_tag_bits']) == (signed, bits)
        assert not parsed['field_semantics_complete']
        packet += words([parsed['activity_byte'], parsed['body_group_tag'], parsed['body_group_tag_bits']])
    return bytes(packet)


def run_oracle(nodes: bytes) -> bytes:
    """Compare independent ordinal grouping with actual activity-independent diagnostic runs."""
    import simulationmesh as reader
    actual = reader.node_group_runs(nodes)
    assert actual == independent_runs(nodes)
    return words([len(actual)]) + b''.join(words([run['tag'], run['first_node'], run['last_node']]) for run in actual)


def freeze_owned() -> None:
    """Own the entire signed domain, repeated separated runs and allocation-chunk crossings."""
    cases = group_cases()
    packet = b'SNG1' + words([len(cases)])
    packet += b''.join(span(nodes) + span(node_oracle(nodes)) + span(run_oracle(nodes)) for nodes in cases)
    path = ROOT / 'tests/fixtures/simulation-node-groups/groups.bin'
    raw = zlib.compress(packet, 9)
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('owned-reference.json', dict(cases=len(cases), nodes=sum(len(n) // 52 for n in cases), signed_patterns=65536, oracle_sha256=sha(packet)))


def freeze_guards() -> None:
    """Require complete records without allowing signed patterns to become shape errors."""
    import simulationmesh as reader
    for length in (*range(52), 53, 104):
        with unittest.TestCase().assertRaises(ValueError):
            reader.node(bytes(length))
    for length in (*range(1, 52), 53, 103, 105):
        with unittest.TestCase().assertRaises(ValueError):
            reader.node_group_runs(bytes(length))


def query_oracle(nodes: bytes, elements: bytes, body: int) -> tuple[bytes, dict[str, object]]:
    """Keep native element selection independent of node tag membership."""
    import simulationbodynodes as reader
    parsed = reader.project(nodes, elements, body_id=body, mode=3, callback_value=-1, start=0, end=len(elements) // 140 - 1)
    selected = [index + 1 for index, value in enumerate(parsed['output']) if value is not None]
    assert selected == independent_selection(nodes, elements, body)
    assert not parsed['callback_executed'] and not parsed['native_io_executed']
    crossed = [ordinal for ordinal in selected if struct.unpack_from('<h', nodes, (ordinal - 1) * 52 + 22)[0] != body]
    packet = words([body, len(selected), *selected, len(crossed), *crossed])
    facts = dict(body=body, selected=len(selected), cross_tag_selected=len(crossed), cross_tag_one_based_ordinals=crossed)
    return packet, facts


def native_pair(master: bytes, gen: bytes) -> tuple[bytes, dict[str, object]]:
    """Obtain stored blocks only through complete paired reference readers."""
    import simulationmaster
    import simulationgen
    parsed = simulationgen.read(gen, master=simulationmaster.read(master))
    nodes, elements = parsed['nodes']['raw'], parsed['elements']['raw']
    keys = sorted({struct.unpack_from('<H', elements, offset + 28)[0] for offset in range(0, len(elements), 140)})
    queries = [query_oracle(nodes, elements, body) for body in keys]
    packet = span(node_oracle(nodes)) + span(run_oracle(nodes)) + words([len(queries)])
    packet += b''.join(span(query[0]) for query in queries)
    facts = dict(nodes=len(nodes) // 52, tag_bytes=len(nodes) // 26, runs=independent_runs(nodes), queries=[query[1] for query in queries])
    return packet, facts


def freeze_native() -> None:
    """Bind all installed runs and shared nodes without reducing evidence to a histogram."""
    packets, facts, projected = [], [], bytearray(b'SNGV')
    for source, carrier, master, gen in shared.native_pairs():
        expected, record = native_pair(master, gen)
        packets.append(span(master) + span(gen) + span(expected))
        projected += expected
        facts.append(dict(path=source['path'], carrier_sha256=sha(carrier), **record))
    assert sum(record['nodes'] for record in facts) == 45246
    assert sum(query['cross_tag_selected'] for record in facts for query in record['queries']) == 152
    report('native-reference.json', dict(files=len(facts), records=facts))
    export_native(packets, bytes(projected))


def export_native(packets: list[bytes], projected: bytes) -> None:
    """Separate input transport from independently emitted Rust projection bytes."""
    if '--export' in sys.argv:
        packet = b'SNGN' + words([len(packets)]) + b''.join(packets)
        (ROOT / 'target/simulation-node-groups-native.bin').write_bytes(packet)
        report('native-probe-reference.json', dict(bytes=len(packet), sha256=sha(packet)))
    if '--verify-rust' in sys.argv:
        actual = (ROOT / 'target/simulation-node-groups-native-projections.bin').read_bytes()
        assert actual == projected
        report('rust-projection-reference.json', dict(bytes=len(actual), sha256=sha(actual), nodes=45246, cross_tag_selected=152))


def schema() -> None:
    """Keep the complete signed-tag clause and its non-ownership limitation."""
    text = (ROOT.parent / 'inventor-interface/docs/solidworks-file-format/specification.md').read_text().replace('\r\n', '\n')
    raw = (text[text.index('### 14.120 '):text.index('### 14.121 ')].rstrip() + '\n').encode()
    path = FEATURE / 'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('schema-reference.json', dict(issue=122, clauses=['14.120'], sha256=sha(raw)))


def run() -> bool:
    """Copy complete actual readers before import and retain their exact identities."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-node-groups-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        shared.compound.pin_reader(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_owned, freeze_guards, freeze_native, schema))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin, hashes, result, FEATURE, ('cfb', 'simulationmaster', 'simulationgen', 'simulationmesh', 'simulationbodynodes'), ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
