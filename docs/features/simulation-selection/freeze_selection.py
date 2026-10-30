"""Freeze real selection/body-query readers, independent histories and complete native contexts (#119)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest
import zlib

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0,str(FEATURE.parent/'simulation-images'))
import freeze_images as shared
from pinned_reference_tests import FrozenSourceImports,copy_sources,record_snapshot
from owned_selection import QueryCase,WriteHistory,record,MASKS,mask_expected,scalar_expected,predicate_cases,query_cases,independent_query
span,words,sha = shared.span,shared.words,shared.sha


def report(name: str, value: object) -> None:
    """Require explicit review intent before replacing reference baselines."""
    path = FEATURE/name
    raw = (json.dumps(value,indent=2)+'\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw,f'{path}; expected unchanged reviewed baseline'


def predicate_oracle(raw: bytes, mask: int, key: int) -> bytes:
    """Cross-check every numeric predicate without promoting its key to CAD ownership."""
    import simulationselection as reader
    parsed = reader.read(raw)
    assert parsed['raw'] == raw and parsed['key_domain'] == 'native_body_selection_id' and not parsed['field_semantics_complete']
    stored = struct.unpack_from('<H',raw,28)[0]
    assert parsed['selection_key'] == parsed['body_selection_id'] == stored
    valid,scalar = reader.valid_for_mask(raw,mask),reader.solid_averaging_scalar_gate(raw,mask=mask,key=key)
    assert valid == mask_expected(raw[20],mask) and scalar == scalar_expected(raw,mask,key)
    sets = ([],[stored],[stored+1],[-1,65535],[0,1,1])
    flags = [reader.solid_averaging_exclusion_gate(raw,mask=mask,excluded_keys=s) for s in sets]
    assert flags == [scalar_expected(raw,mask,0) and stored not in s for s in sets]
    return words([stored,parsed['family'],int(valid),int(scalar),*map(int,flags)])


def freeze_predicates() -> None:
    """Freeze complete low-bit mask truth tables and the independent scalar/set cross-product."""
    cases = predicate_cases()
    packet = bytearray(b'SEP1'+words([len(cases)]))
    for raw,mask,key in cases:
        packet += span(raw)+words([mask,key])+span(predicate_oracle(raw,mask,key))
    truth = [(record(f,10,1),mask) for f in range(256) for mask in MASKS]
    packet += words([len(truth)])
    for raw,mask in truth:
        packet += span(raw)+words([mask])+span(predicate_oracle(raw,mask,0))
    freeze_fixture('predicates.bin',bytes(packet))
    report('predicate-reference.json',dict(cross_product=len(cases),family_mask_pairs=len(truth),oracle_sha256=sha(bytes(packet))))


def freeze_fixture(name: str, packet: bytes) -> None:
    """Keep complete synthetic inputs and expected results independent of native corpus availability."""
    fixture = ROOT/'tests/fixtures/simulation-selection'/name
    compressed = zlib.compress(packet,9)
    if '--write' in sys.argv:
        fixture.parent.mkdir(parents=True,exist_ok=True)
        fixture.write_bytes(compressed)
    assert fixture.read_bytes() == compressed


def context_packet(body: int, mode: int, callback: int | None, bounds: tuple[int,int] | None,
                   initial: list[int | None] | None) -> bytes:
    """Retain callback/range/prior-value absence separately from zero patterns."""
    packet = words([body,mode,int(callback is not None),callback or 0,int(bounds is not None),*(bounds or (0,0)),int(initial is not None)])
    if initial is not None:
        packet += optional_patterns(initial)
    return packet


def optional_patterns(values: list[int | None]) -> bytes:
    """Preserve every prior/final unknown marker rather than assigning zero."""
    return words([len(values)])+b''.join(words([int(v is not None),v or 0]) for v in values)


def query_oracle(case: QueryCase) -> bytes:
    """Bind output, zero return and callback fields to actual readers before adding independent history."""
    import simulationbodynodes as reader
    n,e,body,mode,callback,bounds,initial = case
    expected = independent_query(*case)
    parsed = reader.project(n,e,body_id=body,mode=mode,callback_value=callback,
        start=None if bounds is None else bounds[0],end=None if bounds is None else bounds[1],initial_output=initial)
    assert parsed['output'] == expected['output'] and parsed['written_node_slots'] == expected['writes']
    assert parsed['type_flag'] == expected['flag'] and parsed['return_value'] == 0
    assert not parsed['callback_executed'] and not parsed['native_io_executed']
    assert parsed.get('supplied_callback_result') == (None if mode == 1 else callback)
    packet = optional_patterns(parsed['output'])+words([len(expected['writes']),*expected['writes'],parsed['type_flag'],0])
    callback_initial = parsed.get('callback_initial_i16')
    packet += words([int(mode != 1),callback_initial or 0,int(mode != 1),0 if mode == 1 else callback,
        int(expected['resolved']),expected['visited'],expected['selected'],expected['references']])
    return packet+history_packet(expected['history'])


def history_packet(history: list[WriteHistory]) -> bytes:
    """Bind every duplicate write to node, element/reference and overwritten-value parents."""
    packet = bytearray(words([len(history)]))
    for kind,node,element,slot,previous,pattern in history:
        packet += words([kind,node,element,slot,int(previous is not None),previous or 0,pattern])
    return bytes(packet)


def freeze_queries() -> None:
    """Own every supported branch with complete raw blocks and reviewed prior marks."""
    cases = query_cases()
    packet = bytearray(b'SEQ1'+words([len(cases)]))
    for case in cases:
        n,e,*context = case
        packet += span(n)+span(e)+context_packet(*context)+span(query_oracle(case))
    freeze_fixture('queries.bin',bytes(packet))
    report('query-reference.json',dict(cases=len(cases),oracle_sha256=sha(bytes(packet))))


def native_query_cases(n: bytes, e: bytes) -> list[QueryCase]:
    """Use explicit supplied contexts, never infer historical UI or callback state."""
    keys = sorted(set(struct.unpack_from('<H',e,i+28)[0] for i in range(0,len(e),140)))
    result = [(n,e,key,3,-1,(0,len(e)//140-1),None) for key in keys]
    return result+[(n,e,1,0,-1,None,None),(n,e,0xffffffff,1,None,None,None)]


def native_pair(master: bytes, gen: bytes) -> tuple[bytes,dict[str,object]]:
    """Resolve node/element blocks only through actual complete paired readers."""
    import simulationmaster
    import simulationgen
    parsed = simulationgen.read(gen,master=simulationmaster.read(master))
    n,e = parsed['nodes']['raw'],parsed['elements']['raw']
    rows = [e[i:i+140] for i in range(0,len(e),140)]
    packet = bytearray(words([len(rows)]))
    for raw in rows:
        packet += span(predicate_oracle(raw,0,0))+span(predicate_oracle(raw,0xffffffff,-1))
    cases = native_query_cases(n,e)
    packet += words([len(cases)])
    for case in cases:
        packet += context_packet(*case[2:])+span(query_oracle(case))
    return bytes(packet),native_facts(cases)


def native_facts(cases: list[QueryCase]) -> dict[str,object]:
    """Record per-query counts and exact mark/write histories without union double-counting."""
    facts = []
    primary = []
    for case in cases:
        result = independent_query(*case)
        if case[3] == 3:
            primary.append({i for i,value in enumerate(result['output']) if value is not None})
        facts.append(dict(body=case[2],mode=case[3],flag=result['flag'],writes=len(result['writes']),
            marked=sum(v is not None for v in result['output']),oracle_sha256=sha(query_oracle(case))))
    return dict(queries=facts,**body_selection_coverage(primary))


def body_selection_coverage(primary: list[set[int]]) -> dict[str,int | None]:
    """Check the documented union/overlap independently of accumulated per-query marked totals."""
    union = len(set.union(*primary))
    overlap = len(set.intersection(*primary)) if len(primary)==2 else None
    assert (union,overlap) == ((10553,76) if len(primary)==2 else (24140,None))
    return dict(primary_union_nodes=union,primary_overlap_nodes=overlap)


def freeze_native() -> None:
    """Retain all installed source pairs and every documented supplied-context body query."""
    packets,facts = [],[]
    projected = bytearray(b'SEV1')
    for source,carrier,master,gen in shared.native_pairs():
        expected,counts = native_pair(master,gen)
        projected += flattened_native(expected)
        packets.append(span(master)+span(gen)+span(expected))
        facts.append(dict(path=source['path'],carrier_sha256=sha(carrier),oracle_sha256=sha(expected),**counts))
    report('native-reference.json',dict(files=len(facts),records=facts))
    verify_rust_projection(bytes(projected))
    if '--export' in sys.argv:
        packet = b'SEN1'+words([len(packets)])+b''.join(packets)
        (ROOT/'target/simulation-selection-native.bin').write_bytes(packet)
        report('native-probe-reference.json',dict(bytes=len(packet),sha256=sha(packet)))


def verify_rust_projection(expected: bytes) -> None:
    """Bind the independently emitted Rust marks/history/predicates to actual reader bytes."""
    if '--verify-rust' not in sys.argv:
        return
    actual = (ROOT/'target/simulation-selection-native-projections.bin').read_bytes()
    assert actual == expected
    report('rust-projection-reference.json',dict(bytes=len(actual),sha256=sha(actual),queries=11))


def flattened_native(oracle: bytes) -> bytes:
    """Retain only actual computed projections, omitting supplied-context transport fields."""
    count = struct.unpack_from('<I',oracle)[0]
    parts,offset = [words([count])],4
    for _ in range(count*2):
        size = struct.unpack_from('<I',oracle,offset)[0]
        parts.append(oracle[offset+4:offset+4+size])
        offset += 4+size
    queries = struct.unpack_from('<I',oracle,offset)[0]
    parts.append(words([queries]))
    offset += 4
    for _ in range(queries):
        offset = skip_context(oracle,offset)
        size = struct.unpack_from('<I',oracle,offset)[0]
        parts.append(oracle[offset:offset+4+size])
        offset += 4+size
    assert offset == len(oracle)
    return b''.join(parts)


def skip_context(packet: bytes, offset: int) -> int:
    """Respect explicit prior-value presence when skipping a transport context."""
    initial_present = struct.unpack_from('<I',packet,offset+28)[0]
    offset += 32
    if initial_present:
        count = struct.unpack_from('<I',packet,offset)[0]
        offset += 4+count*8
    return offset


def schema() -> None:
    """Retain both complete selection/query clauses as the acceptance scope."""
    text = (ROOT.parent/'inventor-interface/docs/solidworks-file-format/specification.md').read_text().replace('\r\n','\n')
    raw = (text[text.index('### 14.114 '):text.index('### 14.116 ')].rstrip()+'\n').encode()
    path = FEATURE/'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw
    report('schema-reference.json',dict(issue=119,clauses=['14.114','14.115'],sha256=sha(raw)))


def run() -> bool:
    """Pin actual source modules before import and retain verified origins/hashes after checks."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-selection-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        shared.compound.pin_reader(pin,hashes)
        sys.meta_path.insert(0,FrozenSourceImports(pin))
        suite = unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_predicates,freeze_queries,freeze_native,schema))
        result = unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin,hashes,result,FEATURE,('cfb','simulationmaster','simulationgen','simulationselection','simulationbodynodes'),())
        (FEATURE/'fixture-reference-snapshot.json').replace(FEATURE/'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
