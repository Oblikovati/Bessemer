"""Observe direct visual maps and complete managers through immutable Source (#54)."""
from pathlib import Path
import hashlib,struct,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','reference-axes','reference-axis-owners','reference-points','coordinate-systems','component-coordinate-systems','split-features','base-flanges','edge-flanges','stock-features','manager-scalars','configuration-visual-maps'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_blank_part import source_module,reference_snapshot
from freeze_trim_edges import REFERENCE
from object_fixture_observer import object_cursor
from primitive_inventory import observe_primitives,verify_primitive_coverage
from visual_map_bytes import VERSIONS,visual_map_bytes
from owned_visual_managers import owned_visual


def observed_case(reference: Path, version: int, style: int) -> tuple[bytes,dict[str,object]]:
    """Require exact EOF, zero object slots, and independent primitive coverage."""
    raw=visual_map_bytes(version,style);cursor=object_cursor(reference,raw,version,None)
    archive,part,extras=(source_module(name,reference) for name in ('swarchive','partconfigurations','configurationextras'))
    with observe_primitives(archive.Archive,raw) as reads:
        count=part.positive_count(cursor,4,'direct-visual-map');parsed=extras.visual_map(cursor,count)
    assert cursor.p==len(raw) and len(cursor.map)==1 and parsed['map_slots_added']==0
    assert len(parsed['entries'])==max(0,count);verify_primitive_coverage(raw,reads)
    spans=[r for r in reads if r.end>r.start];oracle=''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\n' for r in spans).encode()
    frame=struct.pack('<IBII',version,style,len(raw),len(oracle))+raw+oracle
    return frame,dict(version=version,style=style,bytes=len(raw),reads=len(spans),sha256=hashlib.sha256(raw).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str,object]:
    """Bind direct observations and complete manager goldens to the same Source pin."""
    before=reference_snapshot(reference);destination.mkdir(parents=True,exist_ok=True)
    cases=[observed_case(reference,v,s) for v in VERSIONS for s in range(6)]
    packet=b'DVM1'+struct.pack('<I',len(cases))+b''.join(raw for raw,_ in cases);(destination/'cases.bin').write_bytes(packet)
    owned=[owned_visual(reference,destination,s,False) for s in range(6)]+[owned_visual(reference,destination,2,True)]
    assert before==reference_snapshot(reference)
    return dict(cases=[row for _,row in cases],owned=owned,source_hashes=before,packet_sha256=hashlib.sha256(packet).hexdigest(),reads=sum(row['reads'] for _,row in cases))
