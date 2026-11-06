"""Freeze full slot spans, raw units, native prefixes and existing/applied UTF-16 state."""
import gzip
import json
import struct
from pathlib import Path
import slot_requests as requests


def blob(raw: bytes) -> bytes:
    return struct.pack('<I',len(raw))+raw


def slot_packet(ordinal: int, slot: dict, previous: str, applied: str) -> bytes:
    raw=struct.pack('<IIIII',ordinal,slot['start'],slot['end'],slot['text_start'],slot['stored_units'])
    raw+=blob(slot['raw_bytes'])+blob(requests.encoded(slot['text']))
    return raw+blob(requests.encoded(previous))+blob(requests.encoded(applied))


def case_packet(name: str, native: bool, raw: bytes, previous: list[str]) -> bytes:
    value=requests.parse(raw);applied=requests.flowauxrecords.apply_311(value,existing=previous)
    assert len(value['slots'])==len(applied)==len(previous)==50 and value['end']==len(raw)
    provenance=b'\0' if not native else b'\1'+struct.pack('<i',value['header_i32'])
    result=blob(name.encode())+provenance+blob(raw)+struct.pack('<iI',value['header_i32'],value['end'])
    result+=bytes((value['native_header_dispatch'],))+struct.pack('<I',len(value['slots']))
    return result+b''.join(slot_packet(index,slot,previous[index],applied[index]) for index,slot in enumerate(value['slots']))


def freeze(root: Path=requests.ROOT) -> None:
    rows=requests.cases();_,native=requests.native_cases()
    packet=b'FTS1'+struct.pack('<I',len(rows))+b''.join(case_packet(*row) for row in rows)
    assert len(packet)<=16_000_000
    path=root/'tests/fixtures/flow-text-slots/record311.bin.gz'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(gzip.compress(packet,mtime=0))
    references=('flowauxrecords.py','flowmetadataprefix.py','flowlegacytables.py')
    helpers=(requests.messages,requests.messages.load_requests,requests.objects)
    proof=dict(cases=len(rows),bytes=len(packet),sha256=requests.objects.digest(packet),native=native,
               references={name:requests.objects.digest((requests.SOURCE/name).read_bytes().replace(b'\r\n',b'\n')) for name in references},
               helper_lf_sha256={Path(module.__file__).name:requests.objects.digest(Path(module.__file__).read_bytes().replace(b'\r\n',b'\n')) for module in helpers},
               scope='Complete arbitrary signed header and fifty strict UTF-16 slots under explicit mode zero; empty native prefixes retain prior text state',
               native_header_dispatch=False,native_runtime_updates_executed=False,engineering_fields_assigned=False,native_application_checked=False)
    path=root/'docs/features/flow-text-slots/baseline-verification.json'
    path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8',newline='\n')


if __name__=='__main__':
    freeze()
