"""Complete ordinary display-dimension fixture oracle, with no reader substitutions."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, view_oracle
from freeze_blank_part import reference_snapshot
import display_dimension_fixture_bytes as wire
import display_item_fixture_bytes as item
import displaydimensions
import camerafeatures
import sketchprefix
from mfcobjects import ObjectArchive


def install(source):
    sketchprefix.install_scoped_readers(source)
    displaydimensions.install_scoped(source)
    for name, reader in [('moDispObjRef_c', displaydimensions.reference),
        ('moCompDispObjRef_c', displaydimensions.component_reference), ('moArrowSize_c', displaydimensions.arrow_size),
        ('moFavoriteHandle_c', displaydimensions.favorite_handle),
        ('moAttachPoint_c', camerafeatures.attach_point)]: source.readers[name] = reader


def freeze(mode, version, raw):
    source = ObjectArchive(raw, version)
    source.feature_context = 'ordinary_external_xml'
    source.model_root_class = 'moPart_c'
    install(source)
    if mode == 0: value = source.object()
    elif mode == 1: value = displaydimensions.parameter_array(source)
    elif mode == 2:
        value = displaydimensions.item_prefix(source)
        for reader in [displaydimensions.item_geometry, displaydimensions.item_styles, displaydimensions.item_tail]: reader(source, value)
    else: raise ValueError(f'display oracle mode {mode}; expected 0..=2')
    if source.p != len(raw): raise ValueError(f'display mode {mode}, version {version}, end {source.p}; expected {len(raw)}')
    oracle = view_oracle('root.object' if mode == 0 else 'root', value)
    return struct.pack('<BIII', mode, version, len(raw), len(oracle))+raw+oracle, len(oracle.splitlines())


def support(name, rich):
    w = wire.WireMap()
    if name in ['moDispObjRef_c', 'moCompDispObjRef_c']: return item.reference(w, name == 'moCompDispObjRef_c')
    if name == 'moArrowSize_c': return w.new(name, lambda: wire.P('<3d', -0.0, 1.25, -2.5))
    if name == 'moFavoriteHandle_c': return wire.favorite_handle(w)
    return w.new(name, lambda: wire.callout_data(w, rich))


def generate():
    before = reference_snapshot(REFERENCE)
    versions = [0x27cc, 0x27df, 0x27e0, 0x333d, 0x333e, 0x3371, 0x3372, 0x3373, 0x3374, 0x3747, 0x3748, 0x3b21, 0x3b22, 19000]
    records = []
    for version in versions:
        for rich in [False, True]:
            records += [freeze(0, version, wire.tagged(version, name, rich)) for name in wire.CLASSES]
            records += [freeze(0, version, support(name, rich)) for name in ['moDispObjRef_c', 'moCompDispObjRef_c', 'moArrowSize_c', 'moFavoriteHandle_c', 'moHwCalloutData_c']]
            records += [freeze(1, version, wire.parameters(rich)), freeze(2, version, wire.item(wire.WireMap(), version, rich))]
    wires = {f'display-dimension-{name}.bin': wire.tagged(19000, 'moDisplayRevolveDim_c', True, final)
        for name, final in [('original', -1), ('edited', 12345)]}
    for raw in wires.values(): freeze(0, 19000, raw)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(raw for raw, _ in records)
    wires['display-dimension-cases.bin'] = packet
    for name, raw in wires.items(): (WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    report = {'cases': len(records), 'fields': sum(fields for _, fields in records), 'sha256': hashlib.sha256(packet).hexdigest(),
        'source_hashes': before, 'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        'context': 'ordinary modern external XML; complete bodies with actual tagged dependencies and self references', 'excluded_metadata': []}
    Path(__file__).with_name('display-dimension-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key not in ['source_hashes', 'context', 'excluded_metadata']})


if __name__ == '__main__': generate()
