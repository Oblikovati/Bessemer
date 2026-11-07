"""Freeze exact viewer-prefix outputs while retaining unknown decoded bytes separately."""
import gzip
import json
import struct
from pathlib import Path
import viewer_requests as requests
from wire_packet import blob,packed


def array_packet(array: dict) -> bytes:
    return struct.pack('<IIi',array['start'],array['end'],array['count'])+blob(packed(array['patterns'],'Q'))


def case_packet(name: str, native: bool, payload: bytes) -> bytes:
    value=requests.parse(payload);body=requests.zlib.decompress(payload[4:])
    provenance=b'\0' if not native else b'\1'+struct.pack('<i',value['outer_version'])
    result=blob(name.encode())+provenance+blob(payload)+blob(body)
    result+=struct.pack('<4i',value['outer_version'],value['ignored_initial_i32'],value['stored_second_i32'],value['native_second_output_i32'])
    result+=struct.pack('<I',len(value['arrays']))+b''.join(array_packet(array) for array in value['arrays'])
    result+=blob(packed(value['discarded_wide_triplet_patterns'],'Q'))+struct.pack('<i',value['final_output_i32'])
    fields=('prefix_end','known_decoded_prefix_bytes','unknown_decoded_bytes','stored_payload_bytes','decoded_body_bytes','raw_outer_prefix_bytes')
    result+=struct.pack('<6I',*(value[key] for key in fields))+struct.pack('<2i',value['native_metadata_outputs']['first_i32'],value['native_metadata_outputs']['second_i32'])
    result+=blob(value['opaque_tail'])+bytes(value[key] for key in ('complete_payload_framing','engineering_fields_assigned','native_runtime_state_reproduced'))
    return result+blob(value['native_profile'].encode())


def freeze(root: Path=requests.ROOT) -> None:
    rows=requests.cases();_,native=requests.native_cases()
    packet=b'FVP1'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    assert len(packet)<=16_000_000
    path=root/'tests/fixtures/flow-viewer-prefix/record121.bin.gz'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(gzip.compress(packet,mtime=0))
    references=('flowrecord121.py','flowmetadataprefix.py','flowresultdefinitions.py','flowlegacytables.py')
    helpers=(requests.wides,requests.wides.objects,requests.byte_states)
    proof=dict(cases=len(rows),bytes=len(packet),sha256=requests.wides.counted.digest(packet),native=native,
               references={name:requests.wides.counted.digest((requests.SOURCE/name).read_bytes().replace(b'\r\n',b'\n')) for name in references},
               helper_lf_sha256={Path(module.__file__).name:requests.wides.counted.digest(Path(module.__file__).read_bytes().replace(b'\r\n',b'\n')) for module in helpers},
               compression_generator=dict(build_version=requests.zlib.ZLIB_VERSION,runtime_version=requests.zlib.ZLIB_RUNTIME_VERSION),
               scope='Explicit viewer-inflate-metadata-query profile only; exact version prefix and zlib stream, known decoded prefix and retained opaque suffix',
               complete_payload_framing=False,engineering_fields_assigned=False,native_runtime_state_reproduced=False,native_application_checked=False)
    path=root/'docs/features/flow-viewer-prefix/baseline-verification.json'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
