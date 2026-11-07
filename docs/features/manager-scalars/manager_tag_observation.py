"""Observe actual MFC framing and relocate later references after map insertion (#54)."""
from types import CodeType,FrameType
from typing import Protocol,cast
import struct,sys
from freeze_owned_features import ReferenceDocument


class ObservedManagerArchive(Protocol):
    b: bytes
    p: int
    map: list[object]


class CompleteReferenceManager(Protocol):
    def read_supported(self, document: ReferenceDocument) -> dict[str,object]: ...


class ManagerTagObservation:
    """Observe source execution without replacing any reader or primitive method."""
    def __init__(self, raw: bytes, code: CodeType) -> None:
        self.raw=raw;self.code=code;self.events: list[tuple[int,int]]=[]

    def capture(self, frame: FrameType, event: str, argument: object) -> None:
        if event!='call' or frame.f_code is not self.code:return
        archive=cast(ObservedManagerArchive,frame.f_locals['self'])
        if archive.b==self.raw:self.events.append((archive.p,len(archive.map)))


def observe_manager(original: ReferenceDocument, reference_manager: CompleteReferenceManager, object_code: CodeType) -> tuple[dict[str,object],list[tuple[int,int]]]:
    """Capture each actual object-call position and current map length with profiling restored."""
    observer=ManagerTagObservation(original.read('Contents/CMgr'),object_code);previous=sys.getprofile()
    try:
        sys.setprofile(observer.capture);parsed=reference_manager.read_supported(original)
    finally:sys.setprofile(previous)
    return parsed,observer.events


def relocated_tail(raw: bytes, events: list[tuple[int,int]], position: int, first_slot: int, added: int) -> bytes:
    """Move only recorded later MFC indices; scalar integers and earlier aliases stay verbatim."""
    changed=bytearray(raw)
    for offset,_ in events:
        if offset<=position:continue
        tag=struct.unpack_from('<H',raw,offset)[0]
        if tag in (0,65535):continue
        extended=tag==32767;start=offset+2 if extended else offset
        value=struct.unpack_from('<I' if extended else '<H',raw,start)[0]
        mask=0x80000000 if extended else 0x8000;index=value&(mask-1)
        if index<first_slot:continue
        assert index+added<mask-1
        struct.pack_into('<I' if extended else '<H',changed,start,(value&mask)|(index+added))
    return bytes(changed)
