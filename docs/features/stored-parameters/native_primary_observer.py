"""Observe original primary-double byte spans without replacing any method (#202)."""
from types import CodeType, FrameType
from typing import cast
from source_parameter_observer import ParameterObserver, PARAMETER_CLASSES, owner_object
from source_root_observer import ObservedArchive


class NativePrimaryObserver(ParameterObserver):
    def __init__(self, object_code: CodeType, string_code: CodeType, double_code: CodeType) -> None:
        """Bind frozen methods and independent primary span capture; e.g. NativePrimaryObserver(obj,text,double)."""
        super().__init__(object_code, string_code)
        self.double_code = double_code
        self.double_entries: dict[int, tuple[ObservedArchive, int, dict[str, object]]] = {}
        self.double_spans: dict[tuple[int, int], list[tuple[int, bytes]]] = {}

    def __call__(self, frame: FrameType, event: str, argument: object) -> None:
        """Record actual original f64 extents under their nearest owning object; e.g. sys.setprofile(observer)."""
        super().__call__(frame, event, argument)
        if frame.f_code is not self.double_code:
            return
        if event == 'call':
            owner = owner_object(frame.f_back, self.object_code)
            if owner is not None and owner.get('_class') in PARAMETER_CLASSES:
                archive = cast(ObservedArchive, frame.f_locals['self'])
                self.double_entries[id(frame)] = archive, archive.p, owner
        if event == 'return' and id(frame) in self.double_entries:
            archive, start, owner = self.double_entries.pop(id(frame))
            assert archive.p == start + 8, f'{archive.p-start}; expected an original8-byte double'
            self.double_spans.setdefault((id(archive), id(owner)), []).append((start, archive.b[start:archive.p]))

    def primary_spans(self, archive: ObservedArchive) -> list[tuple[int, bytes]]:
        """Return primary double spans in physical definition order; e.g. observer.primary_spans(archive)."""
        return [self.double_spans[id(archive), id(value)][0]
                for (identity, _), (value, _) in sorted(self.parameters.items()) if identity == id(archive)]


def observe_native_primary(document: object, stream: str, observer: NativePrimaryObserver | None = None) -> tuple[object, ...]:
    """Run original complete readers and return both projected rows and raw spans; e.g. observe_native_primary(doc,stream)."""
    import mfcobjects, swarchive, modelprefix, resolvedfeatures
    methods = mfcobjects.ObjectArchive.object, swarchive.Archive.cstring, swarchive.Archive.f64
    observer = observer or NativePrimaryObserver(*(method.__code__ for method in methods))
    import sys
    previous = sys.getprofile()
    try:
        sys.setprofile(observer)
        _, context = modelprefix.read(document, stream.removesuffix('-ResolvedFeatures'),
                                      through_part_resolved=True, configuration_context=True)
        parsed = resolvedfeatures.read_supported(document, stream)
    finally:
        sys.setprofile(previous)
    assert methods == (mfcobjects.ObjectArchive.object, swarchive.Archive.cstring, swarchive.Archive.f64)
    assert context.p == len(context.b) and parsed['tail'] == b'' and parsed['list_complete']
    archives = context, parsed['archive']
    return tuple((observer.rows(archive), observer.primary_spans(archive)) for archive in archives)
