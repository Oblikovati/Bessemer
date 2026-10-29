"""Place complete direct PhotoWorks managers inside actual CMgr streams (#54)."""
from pathlib import Path
from types import FrameType,CodeType
from typing import cast
import sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from owned_scalar_managers import SavedScalarManagerDocument
from manager_tag_observation import ObservedManagerArchive,observe_manager,relocated_tail
from material_bytes import manager_bytes


class MaterialPresencePosition:
    """Observe only the direct +608 byte read from the real configuration reader."""
    def __init__(self, raw: bytes, read_code: CodeType, base_code: CodeType) -> None:
        self.raw=raw;self.read_code=read_code;self.base_code=base_code;self.positions: list[tuple[int,int]]=[]

    def capture(self, frame: FrameType, event: str, argument: object) -> None:
        if event!='call' or frame.f_code is not self.read_code:return
        if frame.f_back is None or frame.f_back.f_code is not self.base_code:return
        archive=cast(ObservedManagerArchive,frame.f_locals['self'])
        if archive.b==self.raw:self.positions.append((archive.p,len(archive.map)))


def material_fixture(reference: Path, style: int, edited: bool) -> tuple[SavedScalarManagerDocument,dict[str,int]]:
    """Locate the actual absent branch, insert bytes and relocate only observed aliases."""
    root=Path(__file__).resolve().parents[3]
    container,manager,part,archive,objects=(source_module(name,reference) for name in ('swcontainer','configurationmanager','partconfigurations','swarchive','mfcobjects'))
    original=container.Document((root/'tests/fixtures/blank-part.SLDPRT').read_bytes(),table={});raw=original.read('Contents/CMgr')
    observer=MaterialPresencePosition(raw,archive.Archive.u8.__code__,part.base_prefix.__code__);previous=sys.getprofile()
    try:
        sys.setprofile(observer.capture);manager.read_supported(original)
    finally:sys.setprofile(previous)
    assert len(observer.positions)==1;position,first_slot=observer.positions[0];assert raw[position]==0
    baseline,events=observe_manager(original,manager,objects.ObjectArchive.object.__code__)
    payload,slots=manager_bytes(style,edited,first_slot-1);added=slots-first_slot
    relocated=relocated_tail(raw,events,position,first_slot,added)
    changed=relocated[:position]+bytes([(1,1,255,128,254,1)[style]])+payload+relocated[position+1:]
    return SavedScalarManagerDocument(original,changed),dict(slots=len(baseline['archive'].map)+added,presence_position=position,inserted_slots=added)


def owned_material(reference: Path, destination: Path, style: int, edited: bool) -> dict[str,object]:
    """Require full public manager EOF, correct slots and independent primitive coverage."""
    document,details=material_fixture(reference,style,edited);raw=document.read('Contents/CMgr')
    archive,manager=(source_module(name,reference) for name in ('swarchive','configurationmanager'))
    with observe_primitives(archive.Archive,raw) as reads:parsed=manager.read_supported(document)
    assert parsed['end']==len(raw) and len(parsed['archive'].map)==details['slots'];verify_primitive_coverage(raw,reads)
    label=f'owned-0-{style}'+('-edited' if edited else '')
    return write_observation(destination,label,raw,[r for r in reads if r.end>r.start])|details
