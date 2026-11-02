"""Independent complete feature-stream literals for node projections and selected-bit edits (#178)."""
from pathlib import Path
from typing import TYPE_CHECKING, cast
import importlib
import struct
import hashlib
import sys
from node_state_evidence import FEATURE, node_state_baseline
if TYPE_CHECKING:
    from freeze_owned_features import ReferenceDocument

PARAMETER_CLASSES = ('moDensityParameter_c', 'moLengthParameter_c', 'moAngleParameter_c',
                     'moIntegerParameter_c', 'moParameter_c', 'moScalerParameter_c', 'moStringParameter_c')
GENERIC_CLASSES = ('moFavoriteFeatItemData_c', 'moHistoryFeatItemData_c')
UPDATE_MASKS = (1, 2, 4, 0x100, 0x4000, 0x8000, 0x20000)
UPDATE_NAMES = ('stored_needs_update', 'stored_update_error', 'stored_update_warning',
                'stored_sat_not_solved', 'stored_update_locked', 'stored_update_pending', 'stored_update_security')


def owned_node_prefix(state: int, flags: int, second: int) -> bytes:
    """Use a null node name and complete modern node fields with independent words; e.g. owned_node_prefix(1, 0, 0)."""
    words = struct.pack('<HIIiI', 0, state, flags, -19, second)
    assert len(words) == 18, f'{len(words)}; expected18 bytes through the second node word'
    return words + b'\0'


def owned_parameter_suffix(name: str, alternate: bool) -> bytes:
    """Supply each complete concrete body, retaining raw scalar patterns; e.g. owned_parameter_suffix('moParameter_c', False)."""
    if name in GENERIC_CLASSES or name == 'moParameter_c':
        return b'\0\0'
    if name == 'moStringParameter_c':
        return b'\0\0\xff\xfe\xff\x01\xa9\x03'
    scalar = struct.pack('<i', -2147483648) if name == 'moIntegerParameter_c' else struct.pack('<Q', 0x7ff8000000000042)
    replacement = struct.pack('<i', 2147483647) if name == 'moIntegerParameter_c' else struct.pack('<Q', 0x8000000000000000)
    return b'\0\0' + scalar + bytes([255 if alternate else 0]) + (replacement if alternate else b'')


def owned_node_stream(name: str, state: int, flags: int, second: int, alternate: bool) -> tuple[bytes, list[int]]:
    """Build two class-loaded definitions and one alias against the owned97-slot map; e.g. owned_node_stream('moParameter_c', 0, 0, 0, False)."""
    encoded = name.encode('ascii')
    first_tag = struct.pack('<HHH', 0xffff, 1, len(encoded)) + encoded
    suffix = owned_parameter_suffix(name, alternate)
    first = owned_node_prefix(state, flags, second) + suffix
    following = owned_node_prefix(state ^ 0x80000008, flags ^ 0x40000000, second ^ 0xfffffffe) + suffix
    header = struct.pack('<iH', -71, 3)
    starts = [len(header) + len(first_tag) + 2, len(header) + len(first_tag) + len(first) + 4]
    return header + first_tag + first + struct.pack('<H', 0x8061) + following + struct.pack('<HI', 98, 0), starts


def owned_node_cases() -> list[tuple[str, int, int, int, bool]]:
    """Cover every known update combination and parameter role with unrelated patterns; e.g. owned_node_cases()."""
    cases: list[tuple[str, int, int, int, bool]] = []
    for name in GENERIC_CLASSES:
        for selection in range(128):
            state = sum(mask for index, mask in enumerate(UPDATE_MASKS) if selection & (1 << index))
            for extra in (0, 0x80000008, 0xfffd3ef8):
                cases.append((name, state | extra, 0xf1234567, 0xfffffffe, False))
    for name in PARAMETER_CLASSES:
        for selection in range(16):
            flags = 0x4abcdef0 | ((selection & 1) << 28) | (((selection >> 1) & 1) << 29)
            flags |= ((selection >> 2) & 1) << 31
            second = 0xfffffffe | ((selection >> 3) & 1)
            for alternate in (False, True):
                cases.append((name, 0x8002c107, flags, second, alternate))
    return cases


def verify_owned_node_result(parsed: dict[str, object], name: str, state: int, flags: int, second: int) -> None:
    """Require original reader roles, coexisting bits and alias identity; e.g. verify_owned_node_result(result, name, 0, 0, 0)."""
    items = cast(list[dict[str, object]], parsed['items'])
    assert len(items) == 3 and items[0] is items[2], name
    assert all(item['_class'] == name for item in items), name
    for item, words in zip(items[:2], ((state, flags, second), (state ^ 0x80000008, flags ^ 0x40000000, second ^ 0xfffffffe))):
        node = cast(dict[str, object], item['node'])
        signed = struct.unpack('<i', struct.pack('<I', words[0]))[0]
        assert node['update_state'] == node['unknown_0c'] == signed and node['flags_28'] == words[1], node
        assert node['flags_2c'] == node['unknown_2c'] == words[2] and node['id'] == -19, node
        assert node['update_state_unresolved_bits'] == words[0] & 0xfffd3ef8, node
        for key, mask in zip(UPDATE_NAMES, UPDATE_MASKS):
            assert node[key] == bool(words[0] & mask), node
        assert node['stored_needs_id_to_be_set'] == bool(words[1] & 0x40000000), node
        if name not in PARAMETER_CLASSES:
            assert not any(key in node for key in ('stored_read_only', 'stored_shared_parameter', 'stored_global_variable_parameter', 'stored_shared_thickness')), node
            continue
        for key, mask in (('stored_read_only', 0x10000000), ('stored_shared_parameter', 0x20000000), ('stored_global_variable_parameter', 0x80000000)):
            assert node[key] == bool(words[1] & mask), node
        assert node['stored_shared_thickness'] == bool(words[2] & 1), node


def persist_owned_node_packet(packet: bytearray, rows: list[dict[str, object]], context_hashes: list[str], primitives: bytearray) -> None:
    """Lock the complete literal packet without replacing earlier fixtures; e.g. persist_owned_node_packet(packet, rows)."""
    destination = FEATURE.parents[2] / 'tests/fixtures/model-node-state/owned.bin'
    destination.parent.mkdir(exist_ok=True)
    if '--write' in sys.argv:
        destination.write_bytes(packet)
    assert destination.read_bytes() == packet, f'{destination}; expected complete unchanged owned archive packet'
    oracle = ''.join(f'{row["class_name"]}\t{row["update_state"]}\t{row["flags_28"]}\t{row["flags_2c"]}\t'
                     f'{cast(list[int], row["state_offsets"])[0]}\t{cast(list[int], row["state_offsets"])[1]}\t{row["bytes"]}\n' for row in rows).encode()
    expected = destination.with_name('nodes.tsv')
    if '--write' in sys.argv:
        expected.write_bytes(oracle)
    assert expected.read_bytes() == oracle, f'{expected}; expected independent literal node words and offsets'
    physical = destination.with_name('primitives.tsv')
    if '--write' in sys.argv:
        physical.write_bytes(primitives)
    assert physical.read_bytes() == primitives, f'{physical}; expected full original-reader primitive partitions'
    node_state_baseline('owned-reference.json', dict(cases=len(rows), nodes=2 * len(rows), aliases=len(rows),
        sha256=hashlib.sha256(packet).hexdigest(), packet_bytes=len(packet), context_streams_sha256=context_hashes,
        node_oracle_sha256=hashlib.sha256(oracle).hexdigest(), primitive_oracle_sha256=hashlib.sha256(primitives).hexdigest(), rows=rows))


def owned_node_archive_context() -> tuple['ReferenceDocument', list[str]]:
    """Bind the existing complete owned configuration and its actual inherited map; e.g. owned_node_archive_context()."""
    container = importlib.import_module('swcontainer')
    fixture = FEATURE.parents[2] / 'tests/fixtures/blank-part.SLDPRT'
    assert hashlib.sha256(fixture.read_bytes()).hexdigest() == 'b52e080ba2cc64d1d07ed28306f27281b07521aef550bc0c7fea8cd54be2b276', fixture
    document = container.Document(fixture, table={})
    context_hashes = [hashlib.sha256(document.read(name)).hexdigest() for name in
                      ('Contents/Config-0-ModelHeader', 'Contents/Config-0', 'Contents/CMgr')]
    _, context = importlib.import_module('modelprefix').read(document, 'Contents/Config-0', through_part_resolved=True, configuration_context=True)
    assert len(context.map) == 97, f'{len(context.map)}; expected complete owned97-slot inherited map'
    return document, context_hashes


def capture_owned_node_stream(document: 'ReferenceDocument', raw: bytes, starts: list[int], index: int) -> tuple[dict[str, object], bytes]:
    """Corroborate literal word offsets and every physical byte through original primitives; e.g. capture_owned_node_stream(doc, raw, starts, 0)."""
    from freeze_owned_features import ReplacedFeatureDocument, STREAM
    from primitive_inventory import observe_primitives, verify_primitive_coverage
    archive = importlib.import_module('swarchive')
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = importlib.import_module('resolvedfeatures').read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    physical = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, physical)
    words = [read for read in physical if read.source_file == 'modelnode.py' and read.kind in ('i', 'I')]
    offsets = [(start + delta, start + delta + 4) for start in starts for delta in (0, 4, 8, 12)]
    assert [(read.start, read.end) for read in words] == offsets, f'{index}; expected exact literal node word spans {offsets}'
    oracle = ''.join(f'{index}\t{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\t'
                     f'{read.source_file}\t{read.source_function}\t{read.source_line}\n' for read in physical).encode()
    return parsed, oracle


def verify_owned_node_archives() -> None:
    """Check every complete owned stream through the actual inherited-map reader; e.g. verify_owned_node_archives()."""
    document, context_hashes = owned_node_archive_context()
    packet, primitives = bytearray(), bytearray()
    rows: list[dict[str, object]] = []
    for name, state, flags, second, alternate in owned_node_cases():
        raw, starts = owned_node_stream(name, state, flags, second, alternate)
        parsed, physical = capture_owned_node_stream(document, raw, starts, len(rows))
        assert parsed['end'] == len(raw), name
        verify_owned_node_result(parsed, name, state, flags, second)
        packet.extend(struct.pack('<I', len(raw)) + raw)
        primitives.extend(physical)
        rows.append(dict(class_name=name, update_state=state, flags_28=flags, flags_2c=second, alternate=alternate, bytes=len(raw), state_offsets=starts))
    persist_owned_node_packet(packet, rows, context_hashes, primitives)
