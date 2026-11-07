"""Observe complete bodies with the actual independently maintained readers (#54)."""
from pathlib import Path
from functools import partial
import json,sys,struct
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('trim-edge-identifiers','base-flanges','edge-flanges','stock-features','alignment-groups'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from freeze_trim_edges import ObjectFixtureFamily,REFERENCE,freeze as freeze_graphs
from freeze_blank_part import source_module
from sheet_owned_fixtures import SavedSheetMetalDocument,saved_manager
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from freeze_owned_features import STREAM
from freeze_display_dimensions import install as install_display
from object_fixture_observer import ReferenceObjectCursor,object_cursor
from alignment_bytes import alignment_object,VERSIONS,CLASSES


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Install the actual six class readers in this archive only."""
    install_display(cursor)
    source_module('sketchprefix',reference).install_scoped_readers(cursor)
    source_module('alignmentgroups',reference).install(cursor)
    cursor.readers['sgSketch']=source_module('assemblyprofiles',reference).sketch


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require complete bodies, exact EOF and shared roots and reference targets."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));root=cursor.object()
    assert cursor.p==len(raw)
    assert root['_class']==CLASSES[kind]



def fixture_family() -> ObjectFixtureFamily:
    """Cross all six complete class bindings and inherited version boundaries."""
    return ObjectFixtureFamily(build=alignment_object,verify=verify,configure=partial(configure,reference=REFERENCE),magic=b'ALG1',kinds=6,versions=VERSIONS,edited_kinds=(0,1,2,3,4,5))


def active_owned(reference: Path, destination: Path, kind: int) -> dict[str,object]:
    """Read actual CMgr-selected +850 state without treating it as profile +834."""
    container,prefix,features,archive=(source_module(n,reference) for n in ('swcontainer','modelprefix','resolvedfeatures','swarchive'))
    original=container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(),table={})
    manager,_=saved_manager(reference,original,1)
    probe=SavedSheetMetalDocument(original,manager,b'')
    _,context=prefix.read(probe,'Contents/Config-0',through_part_resolved=True,configuration_context=True)
    feature,slots=alignment_object(kind,19000,2,False,len(context.map)-1)
    raw=struct.pack('<iH',-123,1)+feature+bytes(4)
    with observe_primitives(archive.Archive,raw) as reads:
        parsed=features.read_supported(SavedSheetMetalDocument(original,manager,raw),STREAM)
    assert (parsed['end'],parsed['tail'],len(parsed['archive'].map))==(len(raw),b'',slots)
    verify_primitive_coverage(raw,reads)
    return write_observation(destination,f'active-{kind}',raw,[r for r in reads if r.end>r.start])|dict(slots=slots)


def freeze(reference: Path, destination: Path, family: ObjectFixtureFamily) -> dict[str,object]:
    """Keep active sheet-metal-state proofs alongside the unchanged ordinary fixtures."""
    facts=freeze_graphs(reference,destination,family)
    facts['active_owned']=[active_owned(reference,destination,kind) for kind in range(6)]
    return facts


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/body-extensions',fixture_family())
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
    print(len(facts['cases']),facts['reads'],len(facts['owned']))
