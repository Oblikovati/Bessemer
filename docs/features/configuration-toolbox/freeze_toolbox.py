"""Bind complete Toolbox records to both shared serializers and the guarded adapter (#54)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/model-archives'))
sys.path.insert(0, str(WORKSPACE/'docs/features/resolved-features'))
sys.path.insert(0, str(WORKSPACE/'docs/features/circular-patterns'))
from freeze_blank_part import reference_snapshot, source_module
from object_fixture_observer import ReferenceObjectCursor, object_cursor, observe_object_body
from primitive_inventory import observe_primitives
from owned_circular_fixtures import write_observation
from toolbox_fixture_bytes import toolbox_object, FLOAT_BITS

VERSIONS = (0x27cc, 0x27d9, 0x27da, 19000)


def shared_readers(cursor: ReferenceObjectCursor) -> None:
    """Observe consuming reads without the adapter's separate lookahead cursor; e.g. shared_readers(cursor)."""
    import assemblyreferences
    cursor.readers.update(moToolboxPartSpecArray_c=assemblyreferences.toolbox_specs,
                          moToolboxPartSpec_c=assemblyreferences.toolbox_spec)


def verify_value(value: dict[str, object], kind: int, mode: int, bits: int, edited: bool = False) -> None:
    """Check exact nonfinite bits, self/shared identity and selected names; e.g. verify_value(root, 0, 2, FLOAT_BITS[1])."""
    item = value
    if kind == 0:
        items = cast(list[dict[str, object] | None], value['items'])
        if len(items) != (4 if mode else 0):
            raise ValueError(f'Toolbox item count {len(items)}; expected {4 if mode else 0}')
        if not mode:
            return
        if items[0] is not items[1] or items[2] is not None or items[3] is not value:
            raise ValueError('Toolbox identities; expected repeated item, null and root self reference')
        item = cast(dict[str, object], items[0])
        expected = 'Bessemer 工具 Ω' if edited else '工具 Ω'
        if value['raw_70'] != expected:
            raise ValueError(f'Toolbox name {value["raw_70"]!r}; expected {expected!r}')
    if struct.pack('<d', cast(float, item['raw_20'])) != struct.pack('<Q', bits):
        raise ValueError(f'Toolbox floating word; expected {bits:#018x}')


def verify_adapter(reference: Path, raw: bytes, slots: int, kind: int, version: int, mode: int, bits: int) -> None:
    """Check the guarded adapter independently of primitive observation; e.g. verify_adapter(source, raw, 5, 0, 19000, 2, FLOAT_BITS[1])."""
    adapter = source_module('configurationtoolbox', reference)
    cursor = object_cursor(reference, raw, version, adapter.install)
    value = cast(dict[str, object], cursor.object())
    if (cursor.p, len(cursor.map)) != (len(raw), slots):
        raise ValueError(f'Toolbox adapter end/map {(cursor.p, len(cursor.map))}; expected {(len(raw), slots)}')
    verify_value(value, kind, mode, bits)


def freeze_records(reference: Path) -> list[tuple[bytes, dict[str, object]]]:
    """Cover every version, class, collection form and floating word; e.g. freeze_records(source)."""
    records: list[tuple[bytes, dict[str, object]]] = []
    for version in VERSIONS:
        for kind, mode, bits_index in ((kind, mode, bits) for kind in (0, 1) for mode in range(3) for bits in range(3)):
            raw, slots = toolbox_object(kind, mode, FLOAT_BITS[bits_index])
            verify_adapter(reference, raw, slots, kind, version, mode, FLOAT_BITS[bits_index])
            records.append(observe_object_body(reference, raw, slots, kind, version,
                f'toolbox-{version:04x}-{kind}-{mode}-{bits_index}', shared_readers))
    return records


def freeze_owned_case(reference: Path, destination: Path, mode: int, bits: int, edited: bool) -> dict[str, object]:
    """Use actual imported slots and independently built edit bytes; e.g. freeze_owned_case(source, output, 2, FLOAT_BITS[1], True)."""
    container, prefix, archive = (source_module(name, reference) for name in
        ('swcontainer', 'modelprefix', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    document = container.Document(baseline, table={})
    _, imported = prefix.read(document, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    context = cast(ReferenceObjectCursor, imported)
    manager = source_module('configurationmanager', reference).read_supported(document)
    configuration_class = manager['root']['configuration_358']['_class']
    feature, slots = toolbox_object(0, mode, bits, len(context.map)-1, edited)
    raw = struct.pack('<iH', -123, 1)+feature+bytes(4)
    shared_readers(context)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = read_owned(reference, raw, context, configuration_class)
    verify_owned(parsed, raw, slots, mode, bits, edited)
    source_module('configurationtoolbox', reference).install(context)
    verify_owned(read_owned(reference, raw, context, configuration_class), raw, slots, mode, bits, edited)
    label = f'owned-{mode}-{bits:016x}'+('-edited' if edited else '')
    return write_observation(destination, label, raw, reads) | dict(slots=slots,
        imported_slots=len(context.map), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def read_owned(reference: Path, raw: bytes, context: ReferenceObjectCursor, configuration_class: str) -> dict[str, object]:
    """Complete the actual saved helper's MBSM continuation; e.g. read_owned(source, raw, context, 'moPartConfiguration_c')."""
    parsed = cast(dict[str, object], source_module('resolvedfeatures', reference).read_data(raw, context))
    if getattr(context, 'sheet_metal_helper_850', 0):
        source_module('resolvedmbsm', reference).read_map(parsed['archive'], configuration_class=configuration_class)
    return parsed


def verify_owned(parsed: dict[str, object], raw: bytes, slots: int, mode: int, bits: int, edited: bool) -> None:
    """Require complete list framing and pointer semantics; e.g. verify_owned(result, raw, 101, 2, FLOAT_BITS[1], False)."""
    cursor = cast(ReferenceObjectCursor, parsed['archive'])
    if not parsed['list_complete'] or (cursor.p, len(cursor.map)) != (len(raw), slots):
        raise ValueError(f'Toolbox owned end/map {(cursor.p, len(cursor.map))}; expected complete {(len(raw), slots)}')
    items = cast(list[dict[str, object]], parsed['items'])
    verify_value(items[0], 0, mode, bits, edited)


def freeze(reference: Path, destination: Path) -> dict[str, object]:
    """Keep both source adapters hash-stable while freezing every byte; e.g. freeze(source, output)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference));destination.mkdir(parents=True, exist_ok=True)
    records = freeze_records(reference)
    packet = b'TBF1'+struct.pack('<I', len(records))
    packet += b''.join(struct.pack('<I', row['slots']) for _, row in records)+b''.join(raw for raw, _ in records)
    (destination/'cases.bin').write_bytes(packet)
    owned = [freeze_owned_case(reference, destination, mode, bits, edited) for mode, bits, edited in
        [(0, FLOAT_BITS[0], False), (1, FLOAT_BITS[1], False), (2, FLOAT_BITS[2], False),
         (2, FLOAT_BITS[1], False), (2, FLOAT_BITS[1], True)]]
    if before != reference_snapshot(reference):
        raise ValueError('reference changed; expected stable source hashes')
    scripts = sorted(Path(__file__).parent.glob('*.py'))
    builders = {Path(module.__file__).resolve() for module in sys.modules.values()
                if getattr(module, '__file__', None) and str(WORKSPACE/'docs/features') in str(module.__file__)}
    return dict(cases=[row for _, row in records], owned=owned, source_hashes=before,
        packet_sha256=hashlib.sha256(packet).hexdigest(),
        generator_hashes={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in scripts},
        builder_hashes={path.relative_to(WORKSPACE).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(builders)})


if __name__ == '__main__':
    result = freeze(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/configuration-toolbox')
    (Path(__file__).parent/'reference.json').write_bytes((json.dumps(result, indent=2)+'\n').encode())
