"""Complete enclosing streams with original context and a fixture-only twist reader (#211)."""
from pathlib import Path
from typing import Callable, Protocol, cast
import inspect
import struct
from freeze_blank_part import source_module
from sweep_fixture_bytes import SweepOptions, sweep_object
from twist_pointer_sequence import TwistPointerSequence, TwistStyle
from twist_wire_fixtures import CLASS, declared_twist_reader

ROOT = Path(__file__).resolve().parents[3]


class OwnedTwistContext(Protocol):
    map: list[object]
    readers: dict[str, Callable[..., object]]
    sheet_metal_helper_850: bool


class OwnedTwistArchive(Protocol):
    p: int
    map: list[object]


def owned_twist_context(reference: Path) -> tuple[object, OwnedTwistContext]:
    """Use the actual blank document's configuration map; e.g. owned_twist_context(pin)."""
    container = source_module('swcontainer', reference)
    prefix = source_module('modelprefix', reference)
    document = container.Document((ROOT / 'tests/fixtures/blank-part.SLDPRT').read_bytes(), table={})
    _, context = prefix.read(document, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    return document, cast(OwnedTwistContext, context)


def owned_twist_stream(first_slot: int, rich: bool, name: str, style: TwistStyle,
                       payloads: tuple[bytes, ...]) -> tuple[bytes, int, TwistPointerSequence]:
    """Build roots against one enclosing map; e.g. owned_twist_stream(96,False,'moSweep_c','fresh',(body,))."""
    sequence = TwistPointerSequence(style, payloads)
    records: list[bytes] = []
    slots = first_slot + 1
    for _ in payloads:
        raw, slots = sweep_object(19000, rich, SweepOptions(), slots - 1, name=name, twist=sequence)
        records.append(raw)
    return struct.pack('<iH', -123, len(records)) + b''.join(records) + bytes(4), slots, sequence


def read_owned_twist(reference: Path, document: object, context: OwnedTwistContext,
                     raw: bytes, extended: bool) -> dict[str, object]:
    """Call unchanged original functions with a local reader table; e.g. read_owned_twist(pin,doc,ctx,raw,True)."""
    features = source_module('resolvedfeatures', reference)
    context.readers = dict(context.readers)
    if extended:
        context.readers[CLASS] = declared_twist_reader
    else:
        context.readers.pop(CLASS, None)
    flags = {name: True for name, parameter in inspect.signature(features.read_data).parameters.items()
             if parameter.default is False}
    result = cast(dict[str, object], features.read_data(raw, context, **flags))
    finish_owned_twist(reference, document, context, result, len(raw))
    return result


def finish_owned_twist(reference: Path, document: object, context: OwnedTwistContext,
                       result: dict[str, object], expected_end: int) -> None:
    """Require original tail parsing and exact EOF; e.g. finish_owned_twist(pin,doc,ctx,result,len(raw))."""
    archive = cast(OwnedTwistArchive, result['archive'])
    if context.sheet_metal_helper_850:
        manager = source_module('configurationmanager', reference).read_supported(document)
        tail = source_module('resolvedmbsm', reference)
        tail.read_map(archive, configuration_class=manager['root']['configuration_358']['_class'])
    if not result['list_complete'] or archive.p != expected_end:
        raise ValueError(f'twist list/end {(result["list_complete"], archive.p)}; expected {(True, expected_end)}')
