"""Verify helper-one edge families against the complete saved CMgr context (#54)."""
from pathlib import Path
from collections.abc import Callable
import hashlib
import struct
from freeze_edge_flanges import WORKSPACE, source_module
from edge_fixture_bytes import edge_graph
from sheet_owned_fixtures import SavedSheetMetalDocument, saved_manager
from primitive_inventory import observe_primitives, verify_primitive_coverage
from owned_circular_fixtures import write_observation
from freeze_owned_features import STREAM

SavedFeatureBuilder = Callable[[int,int,int,bool,int,bool],tuple[bytes,int]]

def active_document(reference: Path, kind: int, edited: bool, build: SavedFeatureBuilder=edge_graph) -> tuple[SavedSheetMetalDocument, bytes, int]:
    """Derive the helper from a fully decoded manager; e.g. active_document(source,2,False)."""
    container, prefix = (source_module(name, reference) for name in ('swcontainer','modelprefix'))
    original=container.Document((WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes(),table={})
    manager,_=saved_manager(reference,original,1)
    probe=SavedSheetMetalDocument(original,manager,b'')
    _,context=prefix.read(probe,'Contents/Config-0',through_part_resolved=True,configuration_context=True)
    if (context.sheet_metal_helper_850,context.sheet_metal_helper_verified)!=(1,True):
        raise ValueError(f'helper {context.sheet_metal_helper_850}; expected verified saved helper one')
    feature,slots=build(kind,19000,3,edited,len(context.map)-1,True)
    raw=struct.pack('<iH',-123,1)+feature+bytes(4)
    return SavedSheetMetalDocument(original,manager,raw),raw,slots


def freeze_active(reference: Path, destination: Path, kinds: tuple[int,...]=(0,1,2),
                  build: SavedFeatureBuilder=edge_graph) -> list[dict[str,object]]:
    """Observe complete ordinary feature roots and edits; e.g. freeze_active(source,output)."""
    destination.mkdir(parents=True,exist_ok=True)
    return [observe_active(reference,destination,kind,edited,build) for kind in kinds for edited in (False,True)]


def observe_active(reference: Path, destination: Path, kind: int, edited: bool,
                   build: SavedFeatureBuilder=edge_graph) -> dict[str,object]:
    """Require exact EOF, primitive coverage and inherited slots; e.g. observe_active(source,out,0,False)."""
    features,archive=(source_module(name,reference) for name in ('resolvedfeatures','swarchive'))
    document,raw,slots=active_document(reference,kind,edited,build)
    with observe_primitives(archive.Archive,raw) as reads:
        parsed=features.read_supported(document,STREAM)
    if (parsed['end'],parsed['tail'],len(parsed['archive'].map))!=(len(raw),b'',slots):
        raise ValueError(f'active edge end/map {(parsed["end"],len(parsed["archive"].map))}; expected {(len(raw),slots)}')
    verify_primitive_coverage(raw,reads)
    (destination/'manager-1.bin').write_bytes(document.manager)
    label=f'active-{kind}'+('-edited' if edited else '')
    return write_observation(destination,label,raw,[read for read in reads if read.end>read.start]) | dict(
        slots=slots,manager_sha256=hashlib.sha256(document.manager).hexdigest())
