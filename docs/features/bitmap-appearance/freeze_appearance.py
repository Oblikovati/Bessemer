"""Freeze independent literal complete-root fixtures for C-3341 (#189)."""
from pathlib import Path
import hashlib,itertools,json,struct,sys

root=Path(__file__).resolve().parents[3]
destination=root/'tests/fixtures/bitmap-appearance'
signed=(-2147483648,-1,0,1,2147483647)
colors=(0,1,0x7fffffff,0x80000000,0xffffffff)
name=b'moSketchBitmapData_c'
tag=struct.pack('<HHH',65535,1,len(name))+name
bits=(0x8000000000000000,0x7ff8000000000123,0xfff0000000000000,0x7ff0000000000042,0,0x7ff0000000000000)
contexts={int(row.split('\t')[0]):int(row.split('\t')[3]) for row in destination.joinpath('native-contexts.tsv').read_text().splitlines()}
contexts[19000]=97
cases=[];rows=[]
for version in (11000,18000,19000):
    for matching,color,ink in itertools.product(signed,colors,signed if version>=0x2bd3 else (None,)):
        for presence in (0,1,255):
            index=len(cases);bodies=[];local=[]
            for owner in range(2):
                m,c,i=(matching,color,ink) if owner==0 else (~matching,color^0xffffffff,None if ink is None else ~ink)
                dimensions=(2,2) if presence else (-1,-2147483648)
                prefix=struct.pack('<6Q3i',*bits,*dimensions,-7)+b'\x05Entry'+struct.pack('<I',0)
                metadata=struct.pack('<3iIiB',-7,2147483647,m,c,-8,presence)
                plane=b'\x00\x01\xfe\xff' if presence else b''
                body=prefix+metadata+plane+(b'' if i is None else struct.pack('<i',i))
                local.append((len(prefix)+8,len(prefix)+12,len(body)-4 if i is not None else -1,m,c,i))
                bodies.append(body)
            raw=struct.pack('<iH',-71,3)+tag+bodies[0]+struct.pack('<H',0x8000|contexts[version])+bodies[1]+struct.pack('<H',contexts[version]+1)+(struct.pack('<I',0) if version==19000 else b'')
            starts=(6+len(tag),6+len(tag)+len(bodies[0])+2)
            for owner,(start,locations) in enumerate(zip(starts,local)):
                a,b,c,m,color_value,i=locations
                rows.append((index,version,owner,start+a,start+b,-1 if c<0 else start+c,m,color_value,'none' if i is None else i))
            cases.append((version,raw))
packet=bytearray(b'BAP1'+struct.pack('<I',len(cases)))
for version,raw in cases:packet+=struct.pack('<II',version,len(raw))+raw
scalars=''.join('\t'.join(map(str,row))+'\n' for row in rows).encode()
assert len(cases)==825 and len(rows)==1650
artifacts={'roots.bin':bytes(packet),'scalars.tsv':scalars}
if '--write-owned-appearance-fixtures' in sys.argv:
    destination.mkdir(parents=True,exist_ok=True)
    for filename,payload in artifacts.items():destination.joinpath(filename).write_bytes(payload)
for filename,payload in artifacts.items():assert destination.joinpath(filename).read_bytes()==payload,filename
proof={'issue':189,'cases':len(cases),'physical_owners':len(rows),'aliases':len(cases),'archive_versions':[11000,18000,19000],'signed_domain':signed,'unsigned_color_domain':colors,'plane_presence_words':[0,1,255],'source_parser_executed':False,'qualification':'Independent literal roots and scalar byte offsets; original Source validation is separate.','artifacts':{name:{'bytes':len(payload),'sha256':hashlib.sha256(payload).hexdigest()} for name,payload in artifacts.items()}}
Path(__file__).with_name('owned-fixture-reference.json').write_text(json.dumps(proof,indent=2)+'\n')
print(json.dumps({'cases':len(cases),'owners':len(rows)}))
