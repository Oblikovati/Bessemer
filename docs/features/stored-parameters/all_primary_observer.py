"""Independent primary spans for every documented parameter value kind (#202)."""
from types import CodeType, FrameType
from typing import cast
from native_primary_observer import NativePrimaryObserver, observe_native_primary
from source_parameter_observer import owner_object, PARAMETER_CLASSES
from source_root_observer import ObservedArchive


class AllPrimaryObserver(NativePrimaryObserver):
    def __init__(self, codes: tuple[CodeType, ...], integer_reader: CodeType) -> None:
        """Bind original numeric methods and the exact integer reader caller; e.g. AllPrimaryObserver(codes,reader)."""
        super().__init__(*codes[:3])
        self.integer_code, self.integer_reader = codes[3], integer_reader
        self.integer_entries: dict[int, tuple[ObservedArchive, int, dict[str, object]]] = {}
        self.integer_spans: dict[tuple[int, int], list[tuple[int, bytes]]] = {}
        self.text_spans: dict[tuple[int, int], tuple[int, bytes]] = {}

    def __call__(self, frame: FrameType, event: str, argument: object) -> None:
        """Capture CString spans before the base observer consumes its entry; e.g. sys.setprofile(observer)."""
        text = self.string_entries.get(id(frame)) if event == 'return' and frame.f_code is self.string_code else None
        super().__call__(frame, event, argument)
        if text is not None:
            archive, start, owner = text
            if owner.get('_class') == 'moStringParameter_c':
                self.text_spans[id(archive), id(owner)] = start, archive.b[start:archive.p]
        if frame.f_code is self.integer_code:
            self.capture_integer(frame, event)

    def capture_integer(self, frame: FrameType, event: str) -> None:
        """Exclude node-state integers by the exact original caller code; e.g. observer.capture_integer(frame,event)."""
        parent = frame.f_back
        if event == 'call' and parent is not None and parent.f_code is self.integer_reader:
            owner = owner_object(parent, self.object_code)
            assert owner is not None and owner.get('_class') == 'moIntegerParameter_c'
            archive = cast(ObservedArchive, frame.f_locals['self'])
            self.integer_entries[id(frame)] = archive, archive.p, owner
        if event == 'return' and id(frame) in self.integer_entries:
            archive, start, owner = self.integer_entries.pop(id(frame))
            assert archive.p == start + 4, f'{archive.p-start}; expected original4-byte integer'
            self.integer_spans.setdefault((id(archive), id(owner)), []).append((start, archive.b[start:archive.p]))

    def primary_spans(self, archive: ObservedArchive) -> list[tuple[int, bytes]]:
        """Select each exact primary, retaining full CString frames; e.g. observer.primary_spans(archive)."""
        return [self.primary_span(archive, value, record.class_name)
                for (identity, _), (value, record) in sorted(self.parameters.items()) if identity == id(archive)]

    def primary_span(self, archive: ObservedArchive, value: dict[str, object], class_name: str) -> tuple[int, bytes]:
        """Keep integer, string and double role selection distinct; e.g. observer.primary_span(archive,value,name)."""
        key = id(archive), id(value)
        if class_name == 'moIntegerParameter_c':
            return self.integer_spans[key][0]
        if class_name == 'moStringParameter_c':
            return self.text_spans[key]
        return self.double_spans[key][0]


def observe_all_primary(document: object, stream: str) -> tuple[object, ...]:
    """Run unchanged original readers across all six classes; e.g. observe_all_primary(doc,stream)."""
    import mfcobjects, swarchive, dimensions
    methods = (mfcobjects.ObjectArchive.object, swarchive.Archive.cstring,
               swarchive.Archive.f64, swarchive.Archive.i32)
    observer = AllPrimaryObserver(tuple(method.__code__ for method in methods), dimensions.integer_parameter.__code__)
    result = observe_native_primary(document, stream, observer)
    assert methods == (mfcobjects.ObjectArchive.object, swarchive.Archive.cstring,
                       swarchive.Archive.f64, swarchive.Archive.i32), 'expected unchanged original methods'
    return result
