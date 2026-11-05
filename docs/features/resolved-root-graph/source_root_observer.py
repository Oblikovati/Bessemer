"""Observe original MFC object calls through profiling events, without method replacement (#201)."""
from dataclasses import dataclass
from types import CodeType, FrameType
from typing import Protocol, cast


class ObservedArchive(Protocol):
    b: bytes
    p: int
    map: list[object]


@dataclass(frozen=True)
class LoadedIdentity:
    class_name: str
    schema: int
    archive: ObservedArchive
    value: object


@dataclass(frozen=True)
class RootObservation:
    tag: int
    extension: int | None
    kind: int
    index: int
    class_name: str
    schema: int
    target_kind: int
    target_index: int


class OriginalObjectObserver:
    def __init__(self, object_code: CodeType) -> None:
        """Bind only the frozen original method code; e.g. OriginalObjectObserver(ObjectArchive.object.__code__)."""
        self.object_code = object_code
        self.entries: dict[int, tuple[ObservedArchive, int, int]] = {}
        self.identities: dict[int, LoadedIdentity] = {}
        self.records: dict[tuple[int, int], RootObservation] = {}

    def __call__(self, frame: FrameType, event: str, argument: object) -> None:
        """Observe entry, reader dispatch and return; e.g. sys.setprofile(observer)."""
        if event == 'call':
            parent = frame.f_back
            if parent is not None and parent.f_code is self.object_code:
                self.capture_definition(parent)
            if frame.f_code is self.object_code:
                archive = cast(ObservedArchive, frame.f_locals['self'])
                self.entries[id(frame)] = archive, archive.p, len(archive.map)
        if event == 'return' and frame.f_code is self.object_code:
            self.capture_return(frame, argument)

    def capture_definition(self, frame: FrameType) -> None:
        """Record a new object's schema before its body can reference itself; e.g. observer.capture_definition(frame)."""
        if 'obj' not in frame.f_locals or id(frame) not in self.entries:
            return
        archive, _, prior_slots = self.entries[id(frame)]
        value = frame.f_locals['obj']
        tag = cast(int, frame.f_locals['tag'])
        index = prior_slots + int(tag == 0xffff)
        assert archive.map[index] is value, f'{index}; expected original newly installed object identity'
        self.identities[id(value)] = LoadedIdentity(cast(str, frame.f_locals['name']),
            cast(int, frame.f_locals['schema']), archive, value)

    def capture_return(self, frame: FrameType, argument: object) -> None:
        """Retain raw framing and original loaded identity; e.g. observer.capture_return(frame,result)."""
        archive, start, prior_slots = self.entries.pop(id(frame))
        tag = cast(int, frame.f_locals.get('tag', -1))
        if tag < 0 or (argument is None and tag != 0):
            return
        extension = cast(int | None, frame.f_locals.get('big'))
        if tag == 0:
            self.records[id(archive), start] = RootObservation(tag, extension, 0, 0, '', 0, 0, 0)
            return
        identity = self.identities[id(argument)]
        kind = 1 if 'obj' in frame.f_locals else 2
        index = prior_slots + int(tag == 0xffff) if kind == 1 else cast(int, frame.f_locals['idx'])
        target_kind = 2 if identity.archive is archive else 1
        self.records[id(archive), start] = RootObservation(tag, extension, kind, index,
            identity.class_name, identity.schema, target_kind, index)
