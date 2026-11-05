"""Observe original object/CString calls without replacing reader methods (#202)."""
from types import FrameType, CodeType
from typing import cast
from source_root_observer import OriginalObjectObserver, ObservedArchive, RootObservation

PARAMETER_CLASSES = ('moDensityParameter_c', 'moLengthParameter_c', 'moAngleParameter_c',
                     'moIntegerParameter_c', 'moScalerParameter_c', 'moStringParameter_c')


def owner_object(frame: FrameType | None, object_code: CodeType) -> dict[str, object] | None:
    """Find the nearest original object frame, retaining nesting; e.g. owner_object(frame,code)."""
    while frame is not None:
        if frame.f_code is object_code:
            return cast(dict[str, object] | None, frame.f_locals.get('obj'))
        frame = frame.f_back
    return None


class ParameterObserver(OriginalObjectObserver):
    def __init__(self, object_code: CodeType, string_code: CodeType) -> None:
        """Bind exact frozen original methods before execution; e.g. ParameterObserver(object_code,string_code)."""
        super().__init__(object_code)
        self.string_code = string_code
        self.string_entries: dict[int, tuple[ObservedArchive, int, dict[str, object]]] = {}
        self.strings: dict[int, tuple[str, bytes]] = {}
        self.pointers: dict[int, list[tuple[int, RootObservation, object]]] = {}
        self.parameters: dict[tuple[int, int], tuple[dict[str, object], RootObservation]] = {}

    def __call__(self, frame: FrameType, event: str, argument: object) -> None:
        """Observe unchanged methods and their actual returned strings; e.g. sys.setprofile(observer)."""
        super().__call__(frame, event, argument)
        if frame.f_code is not self.string_code:
            return
        if event == 'call':
            owner = owner_object(frame.f_back, self.object_code)
            if owner is not None:
                archive = cast(ObservedArchive, frame.f_locals['self'])
                self.string_entries[id(frame)] = archive, archive.p, owner
        if event == 'return' and id(frame) in self.string_entries:
            archive, start, owner = self.string_entries.pop(id(frame))
            assert isinstance(argument, str), f'{argument!r}; expected complete original CString'
            self.strings[id(owner)] = argument, archive.b[start:archive.p]

    def capture_return(self, frame: FrameType, argument: object) -> None:
        """Retain physical definitions and both parameter pointer occurrences; e.g. observer.capture_return(frame,result)."""
        archive, start, _ = self.entries[id(frame)]
        super().capture_return(frame, argument)
        record = self.records.get((id(archive), start))
        if record is None:
            return
        owner = owner_object(frame.f_back, self.object_code)
        if owner is not None and owner.get('_class') in PARAMETER_CLASSES:
            self.pointers.setdefault(id(owner), []).append((start, record, argument))
        if record.kind == 1 and record.class_name in PARAMETER_CLASSES:
            assert isinstance(argument, dict), f'{argument!r}; expected original parameter dictionary'
            self.parameters[id(archive), start] = argument, record

    def rows(self, archive: ObservedArchive) -> list[dict[str, object]]:
        """Project original returned objects in physical wire order; e.g. observer.rows(archive)."""
        return [self.parameter(value, record) for (identity, _), (value, record)
                in sorted(self.parameters.items()) if identity == id(archive)]

    def parameter(self, value: dict[str, object], record: RootObservation) -> dict[str, object]:
        """Keep full raw domains rather than effective/native values; e.g. observer.parameter(value,record)."""
        kind = PARAMETER_CLASSES.index(record.class_name)
        pointers = sorted(self.pointers[id(value)])
        assert len(pointers) == 2, f'{len(pointers)}; expected two physical name pointers'
        assert pointers[0][2] is cast(dict[str, object], value['node'])['name']
        assert pointers[1][2] is value['parameter_name' if kind < 4 else 'name_40']
        primary = value['value' if kind < 4 else 'raw_value_50']
        presence = None if kind == 5 else value['has_alternate_value' if kind < 4 else 'override_presence_60']
        alternate = value.get('alternate_value' if kind < 4 else 'raw_override_58')
        return dict(definition=tag_tuple(record), kind=kind,
                    names=[self.name(pointer, target) for _, pointer, target in pointers],
                    presence=presence, primary=self.value(kind, primary, value),
                    alternate=None if not presence else self.value(kind, alternate, value))

    def name(self, record: RootObservation, value: object) -> dict[str, object]:
        """Retain null/local/inherited identity and original CString bytes; e.g. observer.name(record,value)."""
        return dict(tag=tag_tuple(record), origin=record.target_kind,
                    text=None if value is None else self.strings[id(value)])

    def value(self, kind: int, value: object, owner: dict[str, object]) -> tuple[object, ...]:
        """Compare doubles by bytes, including signaling NaNs; e.g. observer.value(0,value,owner)."""
        import struct
        if kind == 3:
            return 1, struct.pack('<i', cast(int, value))
        if kind == 5:
            text, raw = self.strings[id(owner)]
            assert text == value, f'{text!r}/{value!r}; expected actual primary CString result'
            return 2, text, raw
        return 0, struct.pack('<d', cast(float, value))


def tag_tuple(record: RootObservation) -> tuple[object, ...]:
    """Exclude only implementation-specific IR handles; e.g. tag_tuple(record)."""
    return record.tag, record.extension, record.kind, record.index, record.class_name, record.schema
