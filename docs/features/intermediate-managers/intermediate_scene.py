"""Complete older spherical scene and fresh visual input in the real native caller (#54)."""
from pathlib import Path
from types import FrameType,CodeType
from typing import cast
import struct,sys
from freeze_blank_part import source_module
from owned_scalar_managers import SavedScalarManagerDocument
from manager_tag_observation import ObservedManagerArchive,observe_manager,relocated_tail
from split_item_bytes import visual_body
from freeze_owned_features import ReferenceDocument


class IntermediateVisualPosition:
    """Observe the actual scene visual-pointer call without changing reader functions."""
    def __init__(self, object_code: CodeType, scene_code: CodeType) -> None:
        self.object_code=object_code;self.scene_code=scene_code;self.positions: list[tuple[int,int]]=[]

    def capture(self, frame: FrameType, event: str, argument: object) -> None:
        if event!='call' or frame.f_code is not self.object_code:return
        if frame.f_back is None or frame.f_back.f_code is not self.scene_code or frame.f_locals.get('expect')!='moVisualProperties_c':return
        archive=cast(ObservedManagerArchive,frame.f_locals['self']);self.positions.append((archive.p,len(archive.map)))


def spherical_document(reference: Path, original: ReferenceDocument) -> SavedScalarManagerDocument:
    """Change only the actual class header bound to the shared scene serializer."""
    manager=source_module('configurationmanager',reference);baseline=manager.read_supported(original)
    scene=baseline['root']['data_6e0']['objects'][0]['object'];raw=original.read('Contents/CMgr')
    old=b'swCubicalScene_c';new=b'swSphericalScene_c';position=scene['start']-6-len(old)
    assert raw[position:position+2]==b'\xff\xff' and raw[position+6:scene['start']]==old
    changed=raw[:position+4]+struct.pack('<H',len(new))+new+raw[scene['start']:]
    return SavedScalarManagerDocument(original,changed)


def fresh_visual_document(reference: Path, original: ReferenceDocument) -> SavedScalarManagerDocument:
    """Insert one complete visual and relocate only observed later archive references."""
    manager,objects,scenes=(source_module(name,reference) for name in ('configurationmanager','mfcobjects','configurationoldscenes'))
    document=spherical_document(reference,original);raw=document.read('Contents/CMgr')
    observer=IntermediateVisualPosition(objects.ObjectArchive.object.__code__,scenes.scene.__code__);previous=sys.getprofile()
    try:
        sys.setprofile(observer.capture);manager.read_supported(document)
    finally:sys.setprofile(previous)
    assert len(observer.positions)==1;position,first_slot=observer.positions[0];assert raw[position:position+2]==bytes(2)
    _,events=observe_manager(document,manager,objects.ObjectArchive.object.__code__)
    name=b'moVisualProperties_c';pointer=struct.pack('<3H',65535,1,len(name))+name+visual_body(11000)
    relocated=relocated_tail(raw,events,position,first_slot,2)
    return SavedScalarManagerDocument(original,relocated[:position]+pointer+relocated[position+2:])
