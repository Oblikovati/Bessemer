"""Reference-validated complete sketch tails, roots and standalone tail records."""
from pathlib import Path
import hashlib
import json
import struct
from freeze_views import REFERENCE, WORKSPACE, view_oracle
from freeze_blank_part import reference_snapshot
import sketch_tail_fixture_bytes as wire
from sketch_reference_fixture_bytes import external_reference, temporary_axis
import assemblyprofiles
import modelheader
import sketchprefix
import sketchtail
import sketchintersections
import sketchequidistant
from mfcobjects import ObjectArchive

READERS = [None, sketchequidistant.points, sketchequidistant.constraint, sketchequidistant.logical_group,
           None, sketchtail.offset_collections, sketchtail.styled_intersection_collections,
           sketchtail.state_prefix, sketchtail.visual_state, sketchtail.local_relation_entities,
           sketchtail.coincidence_regions, sketchtail.silhouette_collection, sketchtail.block_manager,
           sketchtail.loop_manager, sketchtail.slot_manager, sketchtail.final_fields,
           None, assemblyprofiles.sketch, None, sketchtail.intersection, sketchtail.sketch_offset,
           sketchtail.offset, sketchtail.styled_curve, sketchtail.entity_handle_data_manager]


def constraint_collections(source):
    value = {'equal_distance_points': [sketchequidistant.points(source) for _ in range(source.u16())],
             'equal_distance_constraints': [sketchequidistant.constraint(source) for _ in range(source.u16())]}
    if source.version > 0x46cb:
        value['logical_groups'] = [sketchequidistant.logical_group(source) for _ in range(source.u16())]
    return value


def install(source):
    sketchprefix.install_scoped_readers(source)
    for name, reader in [('sgPGMProfileData_c', sketchtail.profile_data),
        ('sgPGMLoopOffsetData_c', sketchtail.loop_offset_data), ('sgPGMLoopOffsetEdgeData_c', sketchtail.loop_edge_data),
        ('sgPGMSectionedOffsetEdgeData_c', sketchtail.sectioned_edge_data), ('sgPGMSectionHandles_c', sketchtail.section_handles),
        ('moExtensionLineIndicator_c', sketchtail.extension_line_indicator), ('sgSketch', assemblyprofiles.sketch),
        ('moLoopRef_c', sketchintersections.loop_reference), ('moIntersectionEdge_c', sketchintersections.edge),
        ('bentRefArr', sketchintersections.bent_references), ('suObArray', modelheader.object_list)]:
        source.readers[name] = reader


def remove_derived(value, seen=None):
    seen = set() if seen is None else seen
    if not isinstance(value, (dict, list)) or id(value) in seen: return
    seen.add(id(value))
    if isinstance(value, dict):
        for key in ['present', 'has_profile_feature', 'logicals', 'pattern_constraints', 'surfaces', 'sketch_start', 'sketch_end', 'sketch_unknown_bytes']:
            value.pop(key, None)
        for child in value.values(): remove_derived(child, seen)
    else:
        for child in value: remove_derived(child, seen)


def freeze(mode, version, raw):
    source = ObjectArchive(raw, version)
    source.feature_context = 'ordinary_external_xml'
    source.sketch_profile_mode = False
    install(source)
    if mode == 0: value = source.object()
    elif mode == 4: value = constraint_collections(source)
    elif mode == 16: value = sketchtail.after_patterns(source, {})
    elif mode == 18:
        import stockfeatures
        value = stockfeatures.dynamic_words(source)
    else: value = READERS[mode](source)
    if source.p != len(raw): raise ValueError(f'tail mode {mode}, version {version}, end {source.p}; expected {len(raw)}')
    remove_derived(value)
    oracle = view_oracle('root.object' if mode == 0 else 'root', value)
    return struct.pack('<BIII', mode, version, len(raw), len(oracle))+raw+oracle, len(oracle.splitlines())


def direct(mode, version, rich):
    w = wire.WireMap()
    bodies = {1: lambda: wire.points(w, rich), 2: lambda: wire.constraint(w, version, rich),
        3: lambda: wire.logical_group(version, rich), 4: lambda: wire.constraints(w, version, rich),
        5: lambda: wire.offsets(w, version, rich), 6: lambda: wire.styled_intersections(w, version, rich),
        7: wire.state, 8: lambda: wire.visual(w, rich), 9: lambda: wire.local_relations(w, rich),
        10: lambda: wire.coincidences(w, rich), 11: lambda: wire.silhouettes(version, rich),
        12: lambda: wire.blocks(w, rich), 13: lambda: wire.loops(w, rich), 14: lambda: wire.slots(w, rich),
        15: lambda: wire.P('<i', -2**31), 16: lambda: wire.after_patterns(w, version, rich),
        17: lambda: wire.ordinary(w, version, rich), 18: lambda: wire.dynamic_words(rich),
        19: lambda: wire.intersection(w, version, rich), 20: lambda: wire.sketch_offset(w, rich),
        21: lambda: wire.offset(w, version, rich), 22: lambda: wire.styled(w, rich), 23: lambda: wire.handle_data(w, rich)}
    return bodies[mode]()


def tagged(name, version, rich, final_word=-2**31):
    w = wire.WireMap()
    bodies = {'sgPGMProfileData_c': lambda: wire.profile(w, rich),
        'sgPGMLoopOffsetData_c': lambda: wire.loop_offset(w, rich),
        'sgPGMLoopOffsetEdgeData_c': lambda: wire.loop_edge(w, rich),
        'sgPGMSectionedOffsetEdgeData_c': lambda: wire.sectioned_edge(w, rich),
        'sgPGMSectionHandles_c': lambda: wire.section_handles(w, rich),
        'moIntersectionEdge_c': lambda: wire.edge(w, version, rich),
        'moLoopRef_c': lambda: wire.boundary_base(version, False)+wire.P('<I', 0xffffffff),
        'bentRefArr': lambda: wire.P('<H', 2)+bytes(4),
        'moExtensionLineIndicator_c': lambda: bytes(2)+wire.P('<H', 65535),
        'sgSketch': lambda: wire.ordinary(w, version, rich, final_word)}
    return w.new(name, bodies[name])


def generate():
    before = reference_snapshot(REFERENCE)
    versions = [0x27cc, 0x2b71, 0x2b72, 0x2b98, 0x2b99, 0x2bbe, 0x2bbf, 0x2bdf, 0x2be0, 0x2f50, 0x2f51, 0x46cb, 0x46cc, 19000]
    names = ['sgPGMProfileData_c', 'sgPGMLoopOffsetData_c', 'sgPGMLoopOffsetEdgeData_c',
        'sgPGMSectionedOffsetEdgeData_c', 'sgPGMSectionHandles_c', 'moIntersectionEdge_c',
        'moLoopRef_c', 'bentRefArr', 'moExtensionLineIndicator_c', 'sgSketch']
    records = []
    for version in versions:
        for rich in [False, True]:
            for mode in range(1, 24):
                if mode == 3 and version <= 0x46cb: continue
                records.append(freeze(mode, version, direct(mode, version, rich)))
            records += [freeze(0, version, tagged(name, version, rich)) for name in names]
        for count in [-2**31, -1, 0]:
            records.append(freeze(19, version, wire.intersection(wire.WireMap(), version, False, count)))
        for kind in [0, 6, 8, 9, 10, 65535]:
            if kind in [8, 9] and version > 0x2bbe: continue
            records.append(freeze(21, version, wire.offset(wire.WireMap(), version, True, kind)))
        for reference in [external_reference(version, -1, False), temporary_axis(version, False)]:
            raw = wire.object_tag('sgExtEnt_c')+reference+bytes(6)+b'\0'
            records.append(freeze(0, version, raw))
    wires = {f'sketch-tail-{name}.bin': tagged('sgSketch', 19000, True, final)
        for name, final in [('original', -2**31), ('edited', 12345)]}
    for raw in wires.values(): freeze(0, 19000, raw)
    if before != reference_snapshot(REFERENCE): raise ValueError('reference changed; expected stable source hashes')
    packet = b''.join(record for record, _ in records)
    wires['sketch-tail-cases.bin'] = packet
    for name, raw in wires.items(): (WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    report = {'cases': len(records), 'fields': sum(fields for _, fields in records), 'sha256': hashlib.sha256(packet).hexdigest(),
        'source_hashes': before, 'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        'context': 'ordinary external XML/default sketch helper; no guessed profile flag',
        'excluded_metadata': ['derived presence/completion flags (including surfaces) and sketch cursor/unknown-byte summaries']}
    Path(__file__).with_name('sketch-tail-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({key: value for key, value in report.items() if key not in ['source_hashes', 'context', 'excluded_metadata']})


if __name__ == '__main__': generate()
