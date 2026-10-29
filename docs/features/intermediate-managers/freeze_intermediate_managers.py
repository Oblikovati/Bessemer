"""Observe exact-version direct objects and complete native CMgr contexts (#54)."""
from pathlib import Path
import hashlib,struct,sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','manager-scalars','reference-axes','reference-axis-owners','reference-points','coordinate-systems','component-coordinate-systems','split-features','intermediate-managers'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import REFERENCE
from freeze_blank_part import source_module,reference_snapshot
from object_fixture_observer import object_cursor
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from owned_scalar_managers import SavedScalarManagerDocument
from intermediate_bytes import config_object_bytes,VERSIONS
from intermediate_scene import fresh_visual_document
from intermediate_tail import tail_bytes,rejected_tail

SAMPLE='CAM Examples/tutorial_parts/Mill/mill3axii_5.sldprt'


def direct_case(reference: Path, version: int, style: int) -> tuple[bytes,dict[str,object]]:
    """Use the actual direct reader, with explicit scope and complete primitive coverage."""
    raw,slots=config_object_bytes(version,style);cursor=object_cursor(reference,raw,version,None)
    setattr(cursor,'configuration_version11000',True)
    archive,part=(source_module(name,reference) for name in ('swarchive','partconfigurations'))
    with observe_primitives(archive.Archive,raw) as reads:part.part_config_object(cursor)
    assert cursor.p==len(raw) and len(cursor.map)==slots;verify_primitive_coverage(raw,reads)
    spans=[r for r in reads if r.end>r.start];oracle=''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\n' for r in spans).encode()
    frame=struct.pack('<IBIII',version,style,len(raw),len(oracle),slots)+raw+oracle
    return frame,dict(version=version,style=style,bytes=len(raw),slots=slots,reads=len(spans),sha256=hashlib.sha256(raw).hexdigest())


def native_manager(reference: Path, destination: Path, edited: bool, fresh_visual: bool=False) -> dict[str,object]:
    """Read the actual native manager to EOF without altering its version."""
    container,manager,archive,cover=(source_module(name,reference) for name in ('swcontainer','configurationmanager','swarchive','swcover'))
    original=container.Document((Path(cover.SAMPLES)/SAMPLE).read_bytes(),table={});raw=original.read('Contents/CMgr')
    if fresh_visual:original=fresh_visual_document(reference,original);raw=original.read('Contents/CMgr')
    baseline=manager.read_supported(original);owned=baseline['root']['configuration_358']['config_object_4c8'];assert owned['end']-owned['start']==50
    if edited:raw=raw[:owned['start']]+struct.pack('<i',12345)+raw[owned['start']+4:]
    document=SavedScalarManagerDocument(original,raw)
    with observe_primitives(archive.Archive,raw) as reads:parsed=manager.read_supported(document)
    assert parsed['archive'].version==11000 and parsed['end']==len(raw)
    probes=[r for r in reads if r.source_file=='configurationoldscenes.py' and r.source_function=='scene_data']
    spans=[r for r in reads if r.end>r.start and r not in probes];verify_primitive_coverage(raw,spans)
    label='native-manager'+('-edited' if edited else '')+('-fresh-visual' if fresh_visual else '')
    return write_observation(destination,label,raw,spans)|dict(slots=len(parsed['archive'].map),objects=sum(parsed['archive'].classes.values()),reference_preflight_reads_excluded=len(probes),owned_position=owned['start'])


def direct_tail(reference: Path, destination: Path, style: int, caller: dict[str,object]) -> dict[str,object]:
    """Retain the verified native caller graph while observing the standalone suffix bytes."""
    raw=tail_bytes(style);cursor=object_cursor(reference,raw,11000,None)
    setattr(cursor,'configuration_version11000',True);setattr(cursor,'complete_supported',True)
    archive,reader=(source_module(name,reference) for name in ('swarchive','configurationintermediate'))
    with observe_primitives(archive.Archive,raw) as reads:reader.owned(cursor,caller.copy())
    assert cursor.p==len(raw) and len(cursor.map)==1;verify_primitive_coverage(raw,reads)
    return write_observation(destination,f'tail-{style}',raw,[r for r in reads if r.end>r.start])|dict(slots=1)


def dependency_stops(reference: Path, destination: Path) -> list[dict[str,object]]:
    """Require the actual scoped reader to reject fresh decal and conversion bodies."""
    reader,archive=(source_module(name,reference) for name in ('configurationintermediate','swarchive'));facts=[]
    for name,conversion in [('moDecalProperties_c',False),('moConvertFeatIds_c',True)]:
        raw=rejected_tail(name,conversion);cursor=object_cursor(reference,raw,11000,None)
        setattr(cursor,'configuration_version11000',True);prior=cursor.readers
        try:reader.owned(cursor,{})
        except archive.ArchiveError as error:assert name in str(error)
        else:raise AssertionError(f'{name}; expected an unsupported fresh dependency')
        assert cursor.readers is prior;label='fresh-conversion' if conversion else 'fresh-decal'
        (destination/(label+'.bin')).write_bytes(raw);facts.append(dict(label=label,cls=name,bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest()))
    return facts


def freeze(reference: Path, destination: Path) -> dict[str,object]:
    """Bind direct record gates and the native complete manager to one Source snapshot."""
    before=reference_snapshot(reference);destination.mkdir(parents=True,exist_ok=True)
    cases=[direct_case(reference,v,s) for v in VERSIONS for s in range(6)]
    packet=b'ICM1'+struct.pack('<I',len(cases))+b''.join(frame for frame,_ in cases);(destination/'cases.bin').write_bytes(packet)
    native=[native_manager(reference,destination,edited) for edited in (False,True)]+[native_manager(reference,destination,False,True)]
    container,manager,cover=(source_module(name,reference) for name in ('swcontainer','configurationmanager','swcover'))
    original=container.Document((Path(cover.SAMPLES)/SAMPLE).read_bytes(),table={});caller=manager.read_supported(original)['root']
    tails=[direct_tail(reference,destination,s,caller) for s in range(6)];stops=dependency_stops(reference,destination)
    assert before==reference_snapshot(reference)
    return dict(tails=tails,dependency_stops=stops,cases=[r for _,r in cases],native=native,reads=sum(r['reads'] for _,r in cases),packet_sha256=hashlib.sha256(packet).hexdigest(),source_hashes=before)
