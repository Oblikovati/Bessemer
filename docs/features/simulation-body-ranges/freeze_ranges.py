"""Freeze actual runtime body-type queries and the bounded auxiliary field extractor (#120)."""
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
sys.path.insert(0,str(FEATURE.parent/'simulation-selection'))
import freeze_images as shared
from pinned_reference_tests import FrozenSourceImports,copy_sources,record_snapshot
from bounded_auxiliary import load_fields,fields_source,NAME
from owned_ranges import TypeCase,auxiliary_cases,edit_golden,type_cases,expected_type
span,words,sha=shared.span,shared.words,shared.sha


def report(name: str,value: object) -> None:
    """Require explicit review intent before replacing any reference baseline."""
    path=FEATURE/name
    raw=(json.dumps(value,indent=2)+'\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes()==raw,f'{path}; expected unchanged reviewed baseline'


def auxiliary_oracle(raw: bytes) -> bytes:
    """Execute the unchanged actual bounded extractor, retaining wider mathematical decrements."""
    reader=sys.modules[NAME]
    result=reader.fields(raw,len(raw))
    return words([result['body_selection_id'],result['stored_low'],result['stored_high']])+struct.pack('<2q',result['runtime_start'],result['runtime_upper'])+words([int(result['descriptor_appended'])])


def freeze_auxiliary() -> None:
    """Own all interpreted patterns and literal complete-row field-edit goldens."""
    rows=auxiliary_cases()
    packet=bytearray(b'BRA1'+words([len(rows)]))
    for raw in rows:
        golden=edit_golden(raw)
        packet+=span(raw)+span(auxiliary_oracle(raw))+span(golden)+span(auxiliary_oracle(golden))
    freeze_fixture('auxiliary.bin',bytes(packet))
    report('auxiliary-reference.json',dict(rows=len(rows),profiles=[284,288],oracle_sha256=sha(bytes(packet))))


def freeze_fixture(name: str,packet: bytes) -> None:
    """Keep only synthetic independently constructed wire bytes in the normal test suite."""
    path=ROOT/'tests/fixtures/simulation-body-ranges'/name
    compressed=zlib.compress(packet,9)
    if '--write' in sys.argv:
        path.parent.mkdir(parents=True,exist_ok=True)
        path.write_bytes(compressed)
    assert path.read_bytes()==compressed


def type_oracle(case: TypeCase) -> bytes:
    """Compare actual return/visit/callback fields before attaching independently derived relation coverage."""
    import simulationbodytype
    n,e,body,start,end,mode,callback=case
    expected=expected_type(case)
    actual=simulationbodytype.project(n,e,body_id=body,start=start,end=end,mode=mode,callback_value=callback)
    assert actual['type_flag']==expected['flag'] and actual['visited_elements']==expected['visited']
    assert not actual['file_descriptor_binding'] and not actual['callback_executed'] and not actual['native_io_executed']
    applied=actual.get('supplied_callback_result')
    assert applied==expected['callback']
    packet=words([actual['type_flag'],len(actual['visited_elements']),*actual['visited_elements']])
    packet+=words([int(applied is not None),actual.get('callback_initial_i16',0),int(applied is not None),applied or 0])
    first=expected['first']
    return packet+words([expected['completion'],expected['check'],expected['selected'],expected['references'],int(first is not None),*(first or (0,0,0))])


def case_context(case: TypeCase) -> bytes:
    """Retain every supplied input, even when native precedence bypasses its later checks."""
    _,_,body,start,end,mode,callback=case
    return words([body,start,end,mode,int(callback is not None),callback or 0])


def freeze_types() -> None:
    """Cross every dispatch/endpoint boundary and preserve rejected safe-profile contexts separately."""
    import simulationbodytype
    accepted,rejected=[],[]
    for case in type_cases():
        try:
            expected_type(case)
        except ValueError:
            n,e,body,start,end,mode,callback=case
            with unittest.TestCase().assertRaises(ValueError):
                simulationbodytype.project(n,e,body_id=body,start=start,end=end,mode=mode,callback_value=callback)
            rejected.append(case)
            continue
        oracle=type_oracle(case)
        accepted.append((case,oracle))
    packet=type_packets(accepted,rejected)
    freeze_fixture('types.bin',packet)
    report('type-reference.json',dict(accepted=len(accepted),rejected=len(rejected),oracle_sha256=sha(packet)))


def type_packets(accepted: list[tuple[TypeCase,bytes]],rejected: list[TypeCase]) -> bytes:
    """Keep accepted outputs and typed rejected contexts distinct in the portable fixture packet."""
    packet=bytearray(b'BRT1'+words([len(accepted)]))
    for case,oracle in accepted:
        packet+=span(case[0])+span(case[1])+case_context(case)+span(oracle)
    packet+=words([len(rejected)])
    for case in rejected:
        packet+=span(case[0])+span(case[1])+case_context(case)
    return bytes(packet)


def native_pair(master: bytes,gen: bytes) -> tuple[bytes,dict[str,object]]:
    """Read stored profiles/rows only from actual complete paired readers, never runtime offsets."""
    import simulationmaster
    import simulationgen
    parsed=simulationgen.read(gen,master=simulationmaster.read(master))
    rows=parsed['auxiliary_rows']
    packet=bytearray(words([len(rows)]))
    covered=[]
    facts=[]
    for row in rows:
        encoded,record=native_row(parsed,row['raw'])
        packet+=encoded
        covered.extend(range(record['low'],record['high']+1))
        facts.append(record)
    count=len(parsed['elements']['raw'])//140
    assert sorted(covered)==list(range(1,count+1))
    return bytes(packet),dict(stride=parsed['auxiliary_stride'],elements=count,rows=facts)


def native_row(parsed: dict,raw: bytes) -> tuple[bytes,dict[str,object]]:
    """Bind inclusive stored intervals separately from the unchanged exclusive runtime inputs."""
    fields=sys.modules[NAME].fields(raw,len(raw))
    body,low,high=fields['body_selection_id'],fields['stored_low'],fields['stored_high']
    n,e=parsed['nodes']['raw'],parsed['elements']['raw']
    assert 1<=low<=high<=len(e)//140
    interval=e[(low-1)*140:high*140]
    assert all(struct.unpack_from('<H',interval,i+28)[0]==body for i in range(0,len(interval),140))
    whole=(n,e,body,0,len(e)//140,3,-1)
    proposed=(n,e,body,low-1,high-1,3,-1)
    golden=edit_golden(raw)
    packet=b''.join(map(span,(raw,auxiliary_oracle(raw),golden,auxiliary_oracle(golden),type_oracle(whole),type_oracle(proposed))))
    return packet,dict(body=body,low=low,high=high,stored_rows=high-low+1,interval_sha256=sha(interval),whole_flag=expected_type(whole)['flag'],proposed_flag=expected_type(proposed)['flag'])


def freeze_native() -> None:
    """Check all five installed intervals and full/proposed exclusive query contexts."""
    packets,facts=[],[]
    projected=bytearray(b'BRV1')
    for source,carrier,master,gen in shared.native_pairs():
        expected,counts=native_pair(master,gen)
        projected+=flatten_native(expected)
        packets.append(span(master)+span(gen)+span(expected))
        facts.append(dict(path=source['path'],carrier_sha256=sha(carrier),oracle_sha256=sha(expected),**counts))
    report('native-reference.json',dict(files=len(facts),records=facts))
    verify_rust(bytes(projected))
    if '--export' in sys.argv:
        packet=b'BRN1'+words([len(packets)])+b''.join(packets)
        (ROOT/'target/simulation-body-ranges-native.bin').write_bytes(packet)
        report('native-probe-reference.json',dict(bytes=len(packet),sha256=sha(packet)))


def take_span(packet: bytes,offset: int) -> tuple[bytes,int]:
    """Read only the bounded portable oracle transport, never infer native object framing."""
    size=struct.unpack_from('<I',packet,offset)[0]
    return packet[offset+4:offset+4+size],offset+4+size


def flatten_native(packet: bytes) -> bytes:
    """Compare actual stored/edited fields and exclusive results in the independently emitted Rust order."""
    count=struct.unpack_from('<I',packet)[0]
    parts,offset=[],4
    for _ in range(count):
        entries=[]
        for _ in range(6):
            raw,offset=take_span(packet,offset)
            entries.append(raw)
        original,fields,golden,edited,whole,proposed=entries
        body=struct.unpack_from('<I',fields)[0]
        start,upper=struct.unpack_from('<2q',fields,12)
        parts.extend((fields,span(golden),edited,words([body,start,upper]),span(whole),span(proposed)))
    assert offset==len(packet)
    return b''.join(parts)


def verify_rust(expected: bytes) -> None:
    """Bind complete Rust native projections to actual unmodified source-helper and query outputs."""
    if '--verify-rust' not in sys.argv:
        return
    actual=(ROOT/'target/simulation-body-ranges-native-projections.bin').read_bytes()
    assert actual==expected
    report('rust-projection-reference.json',dict(bytes=len(actual),sha256=sha(actual),auxiliary_rows=5,runtime_queries=10))


def schema() -> None:
    """Freeze both complete clauses while keeping the serialized-descriptor boundary explicit."""
    text=(ROOT.parent/'inventor-interface/docs/solidworks-file-format/specification.md').read_text().replace('\r\n','\n')
    raw=(text[text.index('### 14.116 '):text.index('### 14.118 ')].rstrip()+'\n').encode()
    path=FEATURE/'schema-snapshot.md'
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes()==raw
    report('schema-reference.json',dict(issue=120,clauses=['14.116','14.117'],sha256=sha(raw)))


def run() -> bool:
    """Pin actual modules and the original bounded helper before any evidence invocation."""
    with tempfile.TemporaryDirectory(prefix='bessemer-simulation-body-ranges-') as temporary:
        pin=Path(temporary)
        hashes=copy_sources(pin)
        shared.compound.pin_reader(pin,hashes)
        sys.meta_path.insert(0,FrozenSourceImports(pin))
        load_fields(pin)
        report('bounded-helper-reference.json',dict(path='research/simulation_aux_body_ranges_evidence.py',file_sha256=hashes['research/simulation_aux_body_ranges_evidence.py'],function_sha256=sha(fields_source(pin)),function='fields',unchanged_function_ast=True,top_level_native_inspection_executed=False))
        suite=unittest.TestSuite(unittest.FunctionTestCase(test) for test in (freeze_auxiliary,freeze_types,freeze_native,schema))
        result=unittest.TextTestRunner(verbosity=2).run(suite)
        record_snapshot(pin,hashes,result,FEATURE,('cfb','simulationmaster','simulationgen','simulationbodytype',NAME),())
        (FEATURE/'fixture-reference-snapshot.json').replace(FEATURE/'source-snapshot.json')
        return result.wasSuccessful()


if __name__=='__main__':
    raise SystemExit(0 if run() else 1)
