"""Freeze owned attribute graphs and parameter values with independent readers (#54)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
from freeze_manager_context import WORKSPACE, reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage

sys.path.insert(0, str(WORKSPACE/'docs/features/display-lists'))
from generate_shaded_vectors import ObjectBuilder
from generate_named_vectors import cstring


def parameter_body(state: ObjectBuilder, kind: str, override: int, value: float) -> bytes:
    """Share node/name identity without clamping negative values; e.g. parameter_body(state, 'scalar', 255, -1.75)."""
    name_id = state.next_index + int('moNodeName_c' not in state.classes)
    raw = state.object('moNodeName_c', lambda _: cstring('parameter Ω', True))
    raw += struct.pack('<iIiI', -2, 0xffffffff, -3, 0x80000000)+b'\0'+struct.pack('<H', name_id)
    if kind == 'base': return raw
    if kind == 'string': return raw+cstring('AΩ', True)
    return raw+struct.pack('<dB', value, override)+(struct.pack('<d', -9.5) if override else b'')


def attribute_body(state: ObjectBuilder, identity: int, definition: bool, rich: bool, value: float) -> bytes:
    """Omit the constructor-disabled feature child list; e.g. attribute_body(state, 2, False, True, -1.75)."""
    parameters = []
    if rich:
        parameters = [state.object('moScalerParameter_c', lambda state: parameter_body(state, 'scalar', 255, value)),
                      state.object('moStringParameter_c', lambda state: parameter_body(state, 'string', 0, value))]
    raw = bytes(137)+struct.pack('<H', len(parameters))+b''.join(parameters)+struct.pack('<H', identity)
    if definition:
        raw += struct.pack('<H', identity)
    else:
        nested = state.next_index + int('moAttributeDef_c' not in state.classes)
        raw += state.object('moAttributeDef_c', lambda state: attribute_body(state, nested, True, False, value))
    return raw+struct.pack('<Hi', identity, -123)


def attribute_object(definition: bool, value: float = -1.75) -> bytes:
    """Build a self-referencing definition and shared parameters; e.g. attribute_object(False)."""
    state = ObjectBuilder(1)
    name = 'moAttributeDef_c' if definition else 'moAttribute_c'
    return state.object(name, lambda state: attribute_body(state, 2, definition, not definition, value))


def freeze_attribute_case(reference: Path, destination: Path, stem: str, raw: bytes) -> dict[str, object]:
    """Observe every primitive through EOF; e.g. freeze_attribute_case(source, root, stem, raw)."""
    objects, archive = source_module('mfcobjects', reference), source_module('swarchive', reference)
    attributes = source_module('attributes', reference)
    cursor = objects.ObjectArchive(raw, 19000)
    cursor.feature_context = 'ordinary_external_xml'
    cursor.readers = dict(cursor.readers, moAttribute_c=attributes.read, moAttributeDef_c=attributes.read,
        moParameter_c=attributes.parameter, moScalerParameter_c=attributes.scaler_parameter,
        moStringParameter_c=attributes.string_parameter)
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = cursor.object()
    if cursor.p != len(raw): raise ValueError(f'{stem} at {cursor.p}; expected EOF {len(raw)}')
    if parsed['_class'] in ['moAttribute_c', 'moAttributeDef_c'] and parsed['model_feature']['feature']['feature_list'] is not None:
        raise ValueError(f'{stem} has feature children; expected constructor-disabled child list')
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in nonempty).encode()
    (destination/f'{stem}.bin').write_bytes(raw)
    (destination/f'{stem}-primitives.tsv').write_bytes(rows)
    return {'stem': stem, 'bytes': len(raw), 'reads': len(nonempty),
            'sha256': hashlib.sha256(raw).hexdigest(), 'oracle_sha256': hashlib.sha256(rows).hexdigest()}


def freeze_attributes(reference: Path, destination: Path) -> dict[str, object]:
    """Bind complete raw graphs; e.g. freeze_attributes(source, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    destination.mkdir(parents=True, exist_ok=True)
    cases = [('attribute-object', attribute_object(False)), ('attribute-definition', attribute_object(True)),
             ('attribute-object-edited', attribute_object(False, -17.5))]
    for kind, name, override in [('base', 'moParameter_c', 0), ('scalar', 'moScalerParameter_c', 0), ('scalar', 'moScalerParameter_c', 255), ('string', 'moStringParameter_c', 0)]:
        state = ObjectBuilder(1)
        wire = state.object(name, lambda state: parameter_body(state, kind, override, -1.75))
        cases.append((f'attribute-{kind}-{override}', wire))
    report = [freeze_attribute_case(reference, destination, *case) for case in cases]
    if before != reference_snapshot(reference): raise ValueError('reference changed; expected stable sources')
    builders = ['generate_shaded_vectors.py', 'generate_named_vectors.py']
    return {'issue': 54, 'records': report, 'source_hashes': before,
            'wire_builder_hashes': {name: hashlib.sha256((WORKSPACE/'docs/features/display-lists'/name).read_bytes()).hexdigest() for name in builders},
            'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


if __name__ == '__main__':
    report = freeze_attributes(WORKSPACE.parent/'inventor-interface/solidworks', WORKSPACE/'tests/fixtures/resolved-features')
    Path(__file__).with_name('attributes-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print(report['records'])
