"""Place derived views in complete configuration-manager object maps (#54)."""
from pathlib import Path
from types import FrameType,CodeType
import struct,sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from owned_scalar_managers import SavedScalarManagerDocument
from manager_tag_observation import observe_manager,relocated_tail,ObservedManagerArchive,CompleteReferenceManager
from typing import cast
from freeze_owned_features import ReferenceDocument
from view_bytes import view_object


class ObjectMapPosition:
    """Observe the actual object-map count call without replacing a reader."""
    def __init__(self, raw: bytes, code: CodeType) -> None:
        self.raw=raw;self.code=code;self.positions: list[tuple[int,int]]=[]

    def capture(self, frame: FrameType, event: str, argument: object) -> None:
        if event!='call' or frame.f_code is not self.code or frame.f_locals.get('label')!='object-map':return
        archive=cast(ObservedManagerArchive,frame.f_locals['a'])
        if archive.b==self.raw:self.positions.append((archive.p,len(archive.map)))


def map_position(original: ReferenceDocument, manager: CompleteReferenceManager, count_code: CodeType) -> tuple[int,int]:
    """Locate the zero-count map through actual public Source execution."""
    observer=ObjectMapPosition(original.read('Contents/CMgr'),count_code);previous=sys.getprofile()
    try:
        sys.setprofile(observer.capture);manager.read_supported(original)
    finally:sys.setprofile(previous)
    assert len(observer.positions)==1
    return observer.positions[0]


def manager_fixture(reference: Path, style: int, edited: bool) -> tuple[SavedScalarManagerDocument,dict[str,int]]:
    """Insert real views and relocate later aliases using observed native framing."""
    root=Path(__file__).resolve().parents[3]
    container,manager,archive,objects,part=(source_module(name,reference) for name in ('swcontainer','configurationmanager','swarchive','mfcobjects','partconfigurations'))
    original=container.Document((root/'tests/fixtures/blank-part.SLDPRT').read_bytes(),table={})
    baseline,events=observe_manager(original,manager,objects.ObjectArchive.object.__code__);raw=original.read('Contents/CMgr')
    position,first_slot=map_position(original,manager,part.positive_count.__code__)
    assert raw[position:position+4]==bytes(4)
    view,slots=view_object(0,19000,style,edited,first_slot-1);added=slots-first_slot
    entries=struct.pack('<i2I',3,7,9)+view+struct.pack('<2IH2IH',7,9,0,7,9,first_slot+1)
    relocated=relocated_tail(raw,events,position,first_slot,added)
    changed=relocated[:position]+entries+relocated[position+4:];document=SavedScalarManagerDocument(original,changed)
    return document,dict(slots=len(baseline['archive'].map)+added,map_position=position,inserted_slots=added)


def owned(reference: Path, destination: Path, style: int, edited: bool) -> dict[str,object]:
    """Require actual CMgr EOF and complete independent primitive coverage."""
    document,details=manager_fixture(reference,style,edited);changed=document.read('Contents/CMgr')
    archive,manager=(source_module(name,reference) for name in ('swarchive','configurationmanager'))
    with observe_primitives(archive.Archive,changed) as reads:parsed=manager.read_supported(document)
    assert parsed['end']==len(changed) and len(parsed['archive'].map)==details['slots']
    verify_primitive_coverage(changed,reads)
    label=f'owned-0-{style}'+('-edited' if edited else '')
    return write_observation(destination,label,changed,[r for r in reads if r.end>r.start])|details
