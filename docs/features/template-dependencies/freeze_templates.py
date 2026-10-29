"""Observe template aliases with actual saved-state source readers and complete owned streams (#54)."""
from pathlib import Path
from functools import partial
import json,sys,struct,hashlib
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('model-archives','reference-axes','sweeps','sheet-metal','base-flanges','resolved-features','circular-patterns','flat-patterns','template-dependencies'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_blank_part import source_module,reference_snapshot
from freeze_base_flanges import install_flange_dimensions
from sheet_reference_observer import observe_sheet_object,inventory_bytes
from sheet_owned_fixtures import SavedSheetMetalDocument,saved_manager
from primitive_inventory import observe_primitives
from sheet_primitive_projection import dynamic_payload_ranges,project_payload_words
from owned_circular_fixtures import write_observation
from freeze_owned_features import STREAM
from object_fixture_observer import ReferenceObjectCursor
from template_bytes import template_object,VERSIONS,CLASSES
REFERENCE=WORKSPACE.parent/'inventor-interface/solidworks'


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Bind the actual flat-pattern and two representation readers locally."""
    install_flange_dimensions(cursor,reference)
    flat=source_module('flatpatterns',reference);boundary=source_module('modelboundary',reference)
    cursor.readers.update(zip(CLASSES,(flat.read,boundary.three_integer_surface,boundary.three_integer_surface)))


def verify(root: dict[str,object], kind: int) -> None:
    """Require the concrete class and helper-selected flat-pattern suffix."""
    assert root['_class']==(CLASSES[0] if kind<2 else CLASSES[kind-1])
    if kind<2:assert ('raw_a80' in root['payload'])==(kind==1)


def owned(reference: Path, destination: Path, kind: int, style: int, edited: bool) -> dict[str,object]:
    """Read complete owned feature streams using CMgr-derived helper state and typed kernels."""
    container,prefix,features,archive=(source_module(name,reference) for name in ('swcontainer','modelprefix','resolvedfeatures','swarchive'))
    original=container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(),table={})
    state=int(kind==1);manager,_=saved_manager(reference,original,state)
    assert manager==(WORKSPACE/f'tests/fixtures/sheet-metal/manager-{state}.bin').read_bytes()
    probe=SavedSheetMetalDocument(original,manager,b'')
    _,context=prefix.read(probe,'Contents/Config-0',through_part_resolved=True,configuration_context=True)
    feature,slots=template_object(kind,19000,style,edited,len(context.map)-1)
    raw=struct.pack('<iH',-123,1)+feature+(bytes(4) if state else b'')
    with observe_primitives(archive.Archive,raw) as reads,dynamic_payload_ranges(reference,raw) as ranges:
        parsed=features.read_supported(SavedSheetMetalDocument(original,manager,raw),STREAM);verify(parsed['items'][0],kind)
    assert (parsed['end'],parsed['tail'],len(parsed['archive'].map))==(len(raw),b'',slots)
    label=f'owned-{kind}-{style}'+('-edited' if edited else '')
    return write_observation(destination,label,raw,project_payload_words(raw,reads,ranges))|dict(slots=slots,helper=state,manager_sha256=hashlib.sha256(manager).hexdigest())


def freeze(reference: Path, destination: Path) -> dict[str,object]:
    """Keep independent primitive partitions and saved helper controls in one owned packet."""
    before=reference_snapshot(reference);destination.mkdir(parents=True,exist_ok=True);cases=[]
    for kind in range(4):
        for version in VERSIONS:
            for style in range(6):
                raw,slots=template_object(kind,version,style)
                cases.append(observe_sheet_object(reference,raw,slots,kind*6+style,version,int(kind==1),partial(verify,kind=kind),partial(configure,reference=reference)))
    packet=b'TDP1'+struct.pack('<I',len(cases))+b''.join(struct.pack('<I',r['slots']) for _,r in cases)+b''.join(raw for raw,_ in cases)
    (destination/'cases.bin').write_bytes(packet)
    saved=[owned(reference,destination,k,s,False) for k in range(4) for s in range(6)]+[owned(reference,destination,k,2,True) for k in range(4)]
    assert reference_snapshot(reference)==before
    return dict(cases=[r for _,r in cases],owned=saved,source_hashes=before,packet_sha256=hashlib.sha256(packet).hexdigest(),reads=sum(r['reads'] for _,r in cases))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/template-dependencies')
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
