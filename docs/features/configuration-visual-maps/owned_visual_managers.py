"""Compose complete CMgr streams with untagged configuration visual maps (#54)."""
from pathlib import Path
from types import FrameType,CodeType
from typing import cast
import sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from owned_scalar_managers import SavedScalarManagerDocument
from manager_tag_observation import ObservedManagerArchive
from visual_map_bytes import visual_map_bytes


class VisualMapPosition:
    """Observe the actual count call without substituting a Source reader."""
    def __init__(self, raw: bytes, code: CodeType) -> None:
        self.raw=raw;self.code=code;self.positions: list[int]=[]

    def capture(self, frame: FrameType, event: str, argument: object) -> None:
        if event!='call' or frame.f_code is not self.code or frame.f_locals.get('label')!='direct-visual-map':return
        archive=cast(ObservedManagerArchive,frame.f_locals['a'])
        if archive.b==self.raw:self.positions.append(archive.p)


def owned_visual(reference: Path, destination: Path, style: int, edited: bool) -> dict[str,object]:
    """Check complete CMgr EOF and unchanged map-slot count through actual Source."""
    root=Path(__file__).resolve().parents[3]
    container,manager,part,archive=(source_module(name,reference) for name in ('swcontainer','configurationmanager','partconfigurations','swarchive'))
    original=container.Document((root/'tests/fixtures/blank-part.SLDPRT').read_bytes(),table={});raw=original.read('Contents/CMgr')
    observer=VisualMapPosition(raw,part.positive_count.__code__);previous=sys.getprofile()
    try:
        sys.setprofile(observer.capture);baseline=manager.read_supported(original)
    finally:sys.setprofile(previous)
    assert len(observer.positions)==1;position=observer.positions[0];assert raw[position:position+4]==bytes(4)
    changed=raw[:position]+visual_map_bytes(19000,style,edited)+raw[position+4:]
    with observe_primitives(archive.Archive,changed) as reads:parsed=manager.read_supported(SavedScalarManagerDocument(original,changed))
    assert parsed['end']==len(changed) and len(parsed['archive'].map)==len(baseline['archive'].map)
    verify_primitive_coverage(changed,reads);label=f'owned-0-{style}'+('-edited' if edited else '')
    return write_observation(destination,label,changed,[r for r in reads if r.end>r.start])|dict(slots=len(parsed['archive'].map),map_position=position,inserted_slots=0)
