"""Freeze independent compact-count resource literals for C-3345 (#190)."""
from pathlib import Path
import hashlib,json,struct,sys

root=Path(__file__).resolve().parents[3]
destination=root/'tests/fixtures/bitmap-resource-arrays'
counts=(0,1,2,254,255,256,65533,65534,65535,65536,70000,1048576)
packet=bytearray(b'BRA1'+struct.pack('<I',20));rows=[]
for count in counts:
    payload=bytes((index*73+(count&255))&255 for index in range(count))
    for width in ((2,6) if count<65535 else (6,)):
        prefix=struct.pack('<H',count) if width==2 else struct.pack('<HI',65535,count)
        raw=prefix+payload
        packet+=struct.pack('<BII',width,count,len(raw))+raw
        rows.append(dict(case=len(rows),count=count,prefix_bytes=width,bytes=len(raw),
            sha256=hashlib.sha256(raw).hexdigest(),payload_sha256=hashlib.sha256(payload).hexdigest()))
assert len(rows)==20
if '--write-owned-resource-fixtures' in sys.argv:
    destination.mkdir(parents=True,exist_ok=True)
    destination.joinpath('arrays.bin').write_bytes(packet)
assert destination.joinpath('arrays.bin').read_bytes()==packet
proof=dict(issue=190,cases=rows,packet_bytes=len(packet),packet_sha256=hashlib.sha256(packet).hexdigest(),
    scope='Independent complete headerless count/payload literals; both valid widths below65535, exact65534/65535 boundary and opaque full-byte payloads.',
    source_parser_executed=False,native_target_executed=False,global_accounting_credit=0)
Path(__file__).with_name('owned-fixture-reference.json').write_text(json.dumps(proof,indent=2)+'\n')
print(json.dumps(dict(cases=len(rows),bytes=len(packet),sha256=proof['packet_sha256'])))
