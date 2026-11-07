"""Freeze schema projections with raw field bytes and exact physical intervals."""
import gzip
import json
import struct
from pathlib import Path
import object_requests as requests


def blob(raw: bytes) -> bytes:
    return struct.pack('<I',len(raw))+raw


def optional(raw: bytes | None) -> bytes:
    return b'\0' if raw is None else b'\1'+blob(raw)


def item_packet(item: dict) -> bytes:
    raw=struct.pack('<IIIi',item['ordinal'],item['start'],item['end'],item['member_10_i32'])
    legacy=item['discarded_legacy_i32']
    raw+=optional(None if legacy is None else struct.pack('<4i',*legacy))+blob(item['block16_raw'])
    extra=item['member_7c_i32'];state=item['state_byte']
    raw+=optional(None if extra is None else struct.pack('<i',extra))
    raw+=optional(None if state is None else bytes((state,)))+bytes((item['native_state_byte'],))
    return raw+optional(item['extension56_raw'])


def case_packet(name: str, native: bool, raw: bytes) -> bytes:
    value=requests.parse(raw);assert value['end']==len(raw)
    provenance=b'\0' if not native else b'\1'+struct.pack('<i',value['count'])
    result=blob(name.encode())+provenance+blob(raw)+struct.pack('<iiI',value['version'],value['count'],value['end'])
    result+=struct.pack('<I',len(value['items']))+b''.join(item_packet(item) for item in value['items'])
    trailer=value['legacy_trailer']
    result+=b'\0' if trailer is None else b'\1'+struct.pack('<i',trailer['member_10_i32'])+blob(trailer['block16_raw'])+bytes((trailer['native_appended_object'],))
    return result+bytes(value[key] for key in ('complete_payload_framing','native_runtime_population_reproduced','field_meanings_assigned'))


def freeze(root: Path=requests.ROOT) -> None:
    rows=requests.cases();_,native=requests.native_cases()
    packet=b'FOB1'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    assert len(packet)<=16_000_000
    path=root/'tests/fixtures/flow-object-records/record304.bin.gz'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(gzip.compress(packet,mtime=0))
    references=('flowrecord304.py','flowlegacytables.py')
    proof=dict(cases=len(rows),bytes=len(packet),sha256=requests.digest(packet),native=native,
               manifest_sha256=requests.digest((requests.SOURCE/'research/flow-record304-corpus.json').read_bytes()),
               references={name:requests.digest((requests.SOURCE/name).read_bytes().replace(b'\r\n',b'\n')) for name in references},
               scope='Complete raw versions 0..7, conditional row fields and count-dependent legacy trailer',
               native_runtime_population_reproduced=False,field_meanings_assigned=False,native_application_checked=False)
    path=root/'docs/features/flow-object-records/baseline-verification.json'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
