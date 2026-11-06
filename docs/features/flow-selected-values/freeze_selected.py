"""Freeze full input records, fresh maps and original selected-value projections."""
import gzip
import json
import struct
from pathlib import Path
import selected_requests as requests
LIMIT=8_000_000


def blob(raw: bytes) -> bytes:
    return struct.pack('<I',len(raw))+raw


def entry_packet(entry: dict) -> bytes:
    return struct.pack('<IIIBB',entry['ordinal'],entry['start'],entry['end'],entry['code_byte'],entry['native_code_byte'])


def projection_packet(context: dict, value: dict) -> bytes:
    raw=blob(requests.links_fixture.numbers(value['patterns'],'Q'))
    raw+=struct.pack('<IIII',context['link_count'],context['selected_count'],value['count'],value['end'])
    raw+=requests.links_fixture.counted(context['dense_indices'],'i')+requests.links_fixture.counted(context['selected_ordinals'],'I')
    raw+=struct.pack('<I',len(value['entry_patterns']))
    raw+=b''.join(b'\0' if pattern is None else b'\1'+struct.pack('<Q',pattern) for pattern in value['entry_patterns'])
    return raw+bytes((context['fresh_map_assumed'],context['native_owner_binding_verified'],
                      value['complete_payload_framing'],value['native_owner_binding_verified'],
                      value['native_runtime_scatter_reproduced'],value['engineering_fields_assigned']))


def case_packet(spec: requests.SelectionSpec) -> bytes:
    links,states,context,value=requests.parse(spec)
    link_raw=requests.zlib.decompress(spec.links) if spec.compressed_links else spec.links
    state_raw=requests.zlib.decompress(spec.states)
    assert links['end']==len(link_raw) and states['end']==len(state_raw) and value['end']==len(spec.values)
    raw=blob(spec.name.encode())+(b'\0' if not spec.native else b'\1'+struct.pack('<i',context['selected_count']))
    raw+=bytes((spec.compressed_links,))+blob(spec.links)+blob(link_raw)+blob(spec.states)+blob(state_raw)
    raw+=struct.pack('<iI',links['version'],len(links['entries']))+b''.join(map(entry_packet,links['entries']))
    assert requests.links_fixture.numbers(value['patterns'],'Q')==spec.values
    return raw+projection_packet(context,value)


def packet(cases: list[bytes]) -> bytes:
    accepted=bytes(requests.schema.accepted_code(code) for code in range(256))
    converted=bytes(requests.flowlinkrecords._version0_code(code) for code in range(256))
    return b'FSL1'+accepted+converted+struct.pack('<I',len(cases))+b''.join(cases)


def batches(rows: list[requests.SelectionSpec]) -> list[list[bytes]]:
    result=[];current=[];size=520
    for spec in rows:
        raw=case_packet(spec);assert len(raw)<LIMIT-520
        if size+len(raw)>LIMIT:
            result.append(current);current=[];size=520
        current.append(raw);size+=len(raw)
    if current:result.append(current)
    return result


def freeze(root: Path=requests.ROOT) -> None:
    rows=requests.cases();_,native=requests.native_cases();summaries=[]
    for index,batch in enumerate(batches(rows)):
        raw=packet(batch);relative=f'tests/fixtures/flow-selected-values/part{index}.bin.gz'
        path=root/relative;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(gzip.compress(raw,mtime=0))
        summaries.append(dict(path=relative,cases=len(batch),bytes=len(raw),sha256=requests.counted.digest(raw)))
    references=('flowrecord112.py','flowlinkrecords.py','flowauxrecords.py','flowlegacytables.py','flowresultdefinitions.py')
    helpers=('flow-counted-bytes/counted_requests.py','flow-link-records/freeze_links.py','flow-medium-objects/freeze_objects.py')
    proof=dict(cases=len(rows),packets=summaries,native=native,
               references={name:requests.counted.digest((requests.SOURCE/name).read_bytes().replace(b'\r\n',b'\n')) for name in references},
               owned_helpers={name:requests.counted.digest((requests.FEATURES/name).read_bytes().replace(b'\r\n',b'\n')) for name in helpers},
               compression_generator=dict(build_version=requests.zlib.ZLIB_VERSION,runtime_version=requests.zlib.ZLIB_RUNTIME_VERSION),
               scope='Complete real402/316 inputs, fresh dense maps, raw112 wide patterns and optional per-entry projections',
               exhaustive_code_state_versions=[0,9],other_version_states=[0,1,127,128,129,255],
               native_owner_binding_verified=False,native_runtime_scatter_reproduced=False,engineering_fields_assigned=False,native_application_checked=False)
    path=root/'docs/features/flow-selected-values/baseline-verification.json'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
