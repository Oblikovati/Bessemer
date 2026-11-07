"""Freeze actual bounded names, explicit workspaces and ID-gated requests (#121)."""
from pathlib import Path
import json
import struct
import sys
import tempfile
import unittest
import zlib

FEATURE=Path(__file__).resolve().parent
ROOT=FEATURE.parents[2]
sys.path.insert(0,str(FEATURE.parent/'simulation-images'))
import freeze_images as shared
from pinned_reference_tests import FrozenSourceImports,copy_sources,record_snapshot
from owned_auxiliary import LOGICAL,RESULTS,raw_units,name_cases,caller_names,name_golden,workspace_cases,normalized_golden,plan_golden,binding_golden
span,words,sha=shared.span,shared.words,shared.sha


def report(name: str,value: object) -> None:
    """Require explicit review intent before replacing source-derived baselines."""
    path=FEATURE/name
    raw=(json.dumps(value,indent=2)+'\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes()==raw,f'{path}; expected unchanged reviewed baseline'


def freeze_fixture(name: str,packet: bytes) -> None:
    """Ship only complete synthetic frames and independent expected values in normal tests."""
    path=ROOT/'tests/fixtures/simulation-auxiliary-runtime'/name
    compressed=zlib.compress(packet,9)
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True,exist_ok=True)
        path.write_bytes(compressed)
    assert path.read_bytes()==compressed


def name_oracle(raw: bytes) -> bytes:
    """Keep full raw buffers and surrogate-preserving prefixes without native trimming substitutions."""
    import simulationauxiliary as reader
    parsed=reader.read(raw)
    assert parsed['raw']==raw and parsed['native_returned_name'] is None
    assert not parsed['native_trim_right_applied'] and not parsed['field_semantics_complete']
    assert parsed['name_prefix'].encode('utf-16-le',errors='surrogatepass')==parsed['name_prefix_utf16']
    packet=words([parsed[k] for k in ('stride','body_selection_id','vector_length','first_element','last_element','name_offset','prefix_units')])
    packet+=words([int(parsed['source_nul_index'] is not None),parsed['source_nul_index'] or 0])
    for key in ('opaque_prefix','name_buffer','name_prefix_utf16','name_unconsumed'):
        packet+=span(parsed[key])
    return packet+words(parsed['trailing_words'])


def freeze_names() -> None:
    """Compare literal store goldens with actual code-unit copying for every supplied destination."""
    import simulationauxiliary as reader
    rows,callers=name_cases(),caller_names()
    packet=bytearray(b'AUN1'+words([len(rows)]))
    for raw in rows:
        packet+=span(raw)+span(name_oracle(raw))+words([len(callers)])
        for units in callers:
            golden,count=name_golden(raw,units)
            parsed=reader.project_name_store(raw,raw_units(units).decode('utf-16-le',errors='surrogatepass'))
            assert parsed['raw']==golden and parsed['copied_units']==count
            assert not parsed['tail_cleared'] and not parsed['native_trim_right_applied'] and not parsed['field_semantics_complete']
            packet+=span(raw_units(units))+span(golden)+words([count])+span(name_oracle(golden))
    freeze_fixture('names.bin',bytes(packet))
    report('name-reference.json',dict(rows=len(rows),stores=len(rows)*len(callers),oracle_sha256=sha(bytes(packet))))


def plan_oracle(raw: bytes,logical: int) -> bytes:
    """Independently establish selection/count stops before comparing actual request metadata."""
    import simulationauxruntime as reader
    expected=plan_golden(raw,logical)
    parsed=reader.allocation_plan(raw,logical_index=logical)
    selected,count,length=expected
    assert parsed['selected']==selected and parsed['allocation_called']==selected and not parsed['stored_pointer_usable']
    assert (parsed['vector_count'],parsed['allocation_bytes'],parsed['read_bytes'])==(count,length,length)
    return words([int(selected),int(selected),count,int(length is not None),length or 0,int(length is not None),length or 0,0])


def normalized_oracle(raw: bytes,initial: bytes | None) -> tuple[bytes,bytes | None]:
    """Require the actual legacy source context, and distinguish discarded source bytes from retained prior bytes."""
    import simulationauxruntime as reader
    parsed=reader.normalize(raw,initial_workspace=initial)
    expected=normalized_golden(raw,initial) if initial is not None else raw
    assert parsed['raw']==expected and parsed['memory_only']
    discarded=raw[8:12] if len(raw)==284 else None
    assert parsed['legacy_word_discarded']==discarded
    return parsed['raw'],discarded


def binding_packet(normalized: bytes,logical: int,result: int | None) -> bytes:
    """Freeze successful data projections separately from unsupported count/result contexts."""
    import simulationauxruntime as reader
    try:
        golden=binding_golden(normalized,logical,result)
    except ValueError:
        with unittest.TestCase().assertRaises(ValueError):
            reader.project_pointer_binding(normalized,logical_index=logical,allocation_result=result)
        return words([0])
    parsed=reader.project_pointer_binding(normalized,logical_index=logical,allocation_result=result)
    assert parsed['raw']==golden and parsed['memory_only']
    assert not parsed['allocation_executed'] and not parsed['native_read_executed'] and not parsed['file_write_executed']
    return words([1])+span(golden)


def plan_packet(raw: bytes,logical: int) -> bytes:
    """Do not turn an actual-reader rejection of an independently valid case into an accepted stop."""
    import simulationauxruntime as reader
    try:
        plan_golden(raw,logical)
    except ValueError:
        with unittest.TestCase().assertRaises(ValueError):
            reader.allocation_plan(raw,logical_index=logical)
        return words([0])
    return words([1])+span(plan_oracle(raw,logical))


def freeze_workspaces() -> None:
    """Cross both profiles with signed count limits, ID mismatch and explicit zero-size results."""
    cases=workspace_cases()
    packet=bytearray(b'AUW1'+words([len(cases)]))
    for raw,initial in cases:
        normalized,discarded=normalized_oracle(raw,initial)
        packet+=span(raw)+span(initial)+span(normalized)+words([int(discarded is not None)])
        if discarded is not None:
            packet+=span(discarded)
        packet+=words([len(LOGICAL)])
        for logical in LOGICAL:
            stored,workspace=plan_packet(raw,logical),plan_packet(normalized,logical)
            assert stored==workspace
            packet+=words([logical])+stored+words([len(RESULTS)])
            for result in RESULTS:
                packet+=words([int(result is not None)])+struct.pack('<Q',result or 0)+binding_packet(normalized,logical,result)
    freeze_fixture('workspaces.bin',bytes(packet))
    report('workspace-reference.json',dict(rows=len(cases),plans=len(cases)*len(LOGICAL),bindings=len(cases)*len(LOGICAL)*len(RESULTS),oracle_sha256=sha(bytes(packet))))


def native_row(parsed: dict[str,object],gen: bytes,index: int) -> tuple[bytes,dict[str,object]]:
    """Use the exact original 288-byte trial-read slice, never a zero seed or legacy row substitute."""
    raw=parsed['auxiliary_rows'][index]['raw']
    offset=parsed['auxiliary']['stream_offset']+index*288
    initial=gen[offset:offset+288]
    assert len(initial)==288
    normalized,discarded=normalized_oracle(raw,initial)
    namefacts,replay=stored_name_replay(raw)
    plan=plan_oracle(normalized,index+1)
    assert struct.unpack('<8I',plan)[:3]==(1,1,0)
    bound=binding_packet(normalized,index+1,0)
    packet=span(raw)+span(name_oracle(raw))+span(replay)+words([offset])+span(initial)+span(normalized)
    packet+=words([int(discarded is not None)])+(span(discarded) if discarded is not None else b'')
    return packet+span(plan)+bound,dict(trial_offset=offset,trial_sha256=sha(initial),normalized_sha256=sha(normalized),legacy=len(raw)==284,**namefacts)


def stored_name_replay(raw: bytes) -> tuple[dict[str,object],bytes]:
    """Verify only the supplied-destination store stage, leaving historical temporary tails unassigned."""
    import simulationauxiliary as reader
    parsed=reader.read(raw)
    replay=reader.project_name_store(raw,parsed['name_prefix'])['raw']
    assert replay==raw
    consumed=len(parsed['name_prefix_utf16'])+(2 if parsed['source_nul_index'] is not None else 0)
    residual=parsed['name_buffer'][consumed:]
    return dict(prefix=parsed['name_prefix'],prefix_including_nul_bytes=consumed,residual_bytes=len(residual),residual_nonzero_bytes=sum(v!=0 for v in residual)),replay


def native_pair(master: bytes,gen: bytes) -> tuple[bytes,dict[str,object]]:
    """Obtain all auxiliary rows and offsets from actual complete paired readers."""
    import simulationmaster
    import simulationgen
    parsed=simulationgen.read(gen,master=simulationmaster.read(master))
    packet=bytearray(words([len(parsed['auxiliary_rows'])]))
    facts=[]
    for index in range(len(parsed['auxiliary_rows'])):
        encoded,record=native_row(parsed,gen,index)
        packet+=encoded
        facts.append(record)
    return bytes(packet),dict(stride=parsed['auxiliary_stride'],rows=facts)


def freeze_native() -> None:
    """Retain installed raw names and real trial contexts; supplied result zero is explicitly synthetic."""
    packets,facts=[],[]
    projected=bytearray(b'AUXV')
    for source,carrier,master,gen in shared.native_pairs():
        expected,counts=native_pair(master,gen)
        projected+=expected
        packets.append(span(master)+span(gen)+span(expected))
        facts.append(dict(path=source['path'],carrier_sha256=sha(carrier),oracle_sha256=sha(expected),**counts))
    totals=native_totals(facts)
    report('native-reference.json',dict(files=len(facts),supplied_allocation_result=0,allocation_result_is_synthetic=True,totals=totals,records=facts))
    verify_rust(bytes(projected))
    if '--export' in sys.argv:
        packet=b'AUXN'+words([len(packets)])+b''.join(packets)
        (ROOT/'target/simulation-auxiliary-runtime-native.bin').write_bytes(packet)
        report('native-probe-reference.json',dict(bytes=len(packet),sha256=sha(packet)))


def verify_rust(expected: bytes) -> None:
    """Bind every independently emitted Rust view/replay/trial/normalization/plan/binding byte to actual readers."""
    if '--verify-rust' not in sys.argv:
        return
    actual=(ROOT/'target/simulation-auxiliary-runtime-native-projections.bin').read_bytes()
    assert actual==expected
    report('rust-projection-reference.json',dict(bytes=len(actual),sha256=sha(actual),rows=5,legacy_original_trial_contexts=4,supplied_result_is_synthetic=True))


def native_totals(facts: list[dict[str,object]]) -> dict[str,int]:
    """Keep observed residual bytes separate from consumed prefixes and global coverage claims."""
    rows=[row for item in facts for row in item['rows']]
    totals=dict(rows=len(rows),name_buffer_bytes=len(rows)*256,prefix_including_nul_bytes=sum(r['prefix_including_nul_bytes'] for r in rows),
        residual_bytes=sum(r['residual_bytes'] for r in rows),residual_nonzero_bytes=sum(r['residual_nonzero_bytes'] for r in rows),legacy_trials=sum(r['legacy'] for r in rows))
    assert tuple(totals.values())==(5,1280,146,1134,565,4)
    return totals


def schema() -> None:
    """Retain complete name/slot clauses without attributing historical allocator outcomes."""
    text=(ROOT.parent/'inventor-interface/docs/solidworks-file-format/specification.md').read_text().replace('\r\n','\n')
    raw=(text[text.index('### 14.118 '):text.index('### 14.120 ')].rstrip()+'\n').encode()
    path=FEATURE/'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes()==raw
    report('schema-reference.json',dict(issue=121,clauses=['14.118','14.119'],sha256=sha(raw)))


def run() -> bool:
    """Import only complete immutable actual source modules and retain their origin/hash evidence."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-auxiliary-') as temporary:
        pin=Path(temporary)
        hashes=copy_sources(pin)
        shared.compound.pin_reader(pin,hashes)
        sys.meta_path.insert(0,FrozenSourceImports(pin))
        suite=unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_names,freeze_workspaces,freeze_native,schema))
        result=unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin,hashes,result,FEATURE,('cfb','simulationmaster','simulationgen','simulationauxiliary','simulationauxruntime'),())
        (FEATURE/'fixture-reference-snapshot.json').replace(FEATURE/'source-snapshot.json')
        return result.wasSuccessful()


if __name__=='__main__':
    raise SystemExit(0 if run() else 1)
