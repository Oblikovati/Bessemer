"""Complete independent owned contexts for fresh and cached current-part units (#54)."""
from pathlib import Path
from typing import cast
import json
import struct
from freeze_detail_references import WORKSPACE,REFERENCE,source_module
from freeze_blank_part import reference_snapshot
from freeze_owned_features import ReferenceDocument,STREAM
from object_fixture_observer import ReferenceObjectCursor
from sheet_owned_fixtures import SavedSheetMetalDocument,saved_manager
from owned_circular_fixtures import write_observation
from primitive_inventory import observe_primitives,verify_primitive_coverage
from detail_reference_bytes import CurrentPartIndices,detail_graph


def current_indices(context: ReferenceObjectCursor) -> CurrentPartIndices:
    """Normalize actual current-root and internal-unit identities once; e.g. current_indices(context)."""
    root=cast(dict[str,object]|None,getattr(context,'model_root_object',None))
    if getattr(context,'model_root_verified',False) is not True or root is None or root.get('_class')!='moPart_c':
        raise ValueError(f'current root class {None if root is None else root.get("_class")}; expected verified moPart_c')
    roots=[i for i,v in enumerate(context.map) if v is root]
    units=[i for i,v in enumerate(context.map) if isinstance(v,dict) and v.get('_class')=='moUnitComponent_c' and v.get('owner') is root and v.get('has_external_model')==0]
    if len(roots)!=1 or not units:raise ValueError(f'current root/unit indices {roots}/{units}; expected one mapped root and at least one internal unit')
    return CurrentPartIndices(roots[0],units[0])


def owned_context(reference: Path) -> tuple[ReferenceDocument,bytes,ReferenceObjectCursor,CurrentPartIndices]:
    """Load the complete real saved context before allocating any feature objects; e.g. owned_context(source)."""
    container=source_module('swcontainer',reference);prefix=source_module('modelprefix',reference)
    original=cast(ReferenceDocument,container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(),table={}))
    manager,_=saved_manager(reference,original,0)
    _,context=prefix.read(SavedSheetMetalDocument(original,manager,b''),'Contents/Config-0',through_part_resolved=True,configuration_context=True)
    cursor=cast(ReferenceObjectCursor,context)
    return original,manager,cursor,current_indices(cursor)


def owned_case(reference: Path, destination: Path, mode: int, style: int, edited: bool=False) -> dict[str,object]:
    """Use actual context indices for full guarded Source decoding; e.g. owned_case(source,out,2,5)."""
    original,manager,context,indices=owned_context(reference)
    feature,slots=detail_graph(1,19000,style,edited,len(context.map)-1,mode,indices)
    raw=struct.pack('<iH',-123,1)+feature
    archive=source_module('swarchive',reference);features=source_module('resolvedfeatures',reference)
    with observe_primitives(archive.Archive,raw) as reads:
        parsed=features.read_supported(SavedSheetMetalDocument(original,manager,raw),STREAM)
    if (parsed['end'],parsed['tail'],len(parsed['archive'].map))!=(len(raw),b'',slots):
        raise ValueError(f'current-unit stream {(parsed["end"],len(parsed["archive"].map))}; expected EOF/map {(len(raw),slots)}')
    verify_primitive_coverage(raw,reads)
    label=f'current-{mode}-{style}'+('-edited' if edited else '')
    return write_observation(destination,label,raw,[r for r in reads if r.end>r.start])|dict(mode=mode,style=style,edited=edited,slots=slots,root=indices.root,unit=indices.unit)


def freeze_current_units(reference: Path, destination: Path) -> dict[str,object]:
    """Retain both proven nonnull branches across all identity styles; e.g. freeze_current_units(source,out)."""
    before=reference_snapshot(reference)
    destination.mkdir(parents=True,exist_ok=True)
    rows=[owned_case(reference,destination,mode,style) for mode in (1,2) for style in range(6)]
    rows+=[owned_case(reference,destination,mode,2,True) for mode in (1,2)]
    if before!=reference_snapshot(reference):raise ValueError('current-unit reference snapshot changed=True; expected stable source')
    return dict(owned=rows,source_hashes=before)


if __name__=='__main__':
    facts=freeze_current_units(REFERENCE,WORKSPACE/'tests/fixtures/component-detail-references')
    Path(__file__).with_name('current-unit-reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
