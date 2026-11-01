"""Independent complete Source outcomes for every owned parent field mutation (#166)."""
from collections import Counter
from dataclasses import dataclass
import json
import struct
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_projection import actual_post_record, dialog_fields, dialog_fields_from_record
from dialog_operation_packets import persist_dialog_packets


@dataclass(frozen=True)
class DialogEditField:
    path: str
    tag: int
    start: int
    end: int
    wire: bytes


def decode_edit_field(encoded: bytes) -> DialogEditField:
    """Read the complete frozen primitive descriptor, including its independent span; e.g. decode_edit_field(encoded)."""
    width = struct.unpack_from('<I', encoded)[0]
    path = encoded[4:4 + width].decode('utf-8')
    tag, start, end = struct.unpack_from('<III', encoded, 4 + width)
    wire = encoded[16 + width:]
    assert len(wire) == end - start, (path, start, end, len(wire))
    return DialogEditField(path, tag, start, end, wire)


def edit_wire(field: DialogEditField, donors: tuple[bytes, ...]) -> bytes:
    """Use literal bit flips and complete variable frames rather than Rust output; e.g. edit_wire(field, donors)."""
    if field.tag in (0, 7, 8): return struct.pack('<I', struct.unpack('<I', field.wire)[0] ^ 32)
    if field.tag == 9: return bytes([field.wire[0] ^ 1]) + field.wire[1:]
    if field.tag == 3:
        first = struct.pack('<I', 3) + b'q\0\0'
        return first if first != field.wire else struct.pack('<I', 3) + b'r\0\0'
    if field.tag in (6, 10):
        count = struct.unpack_from('<i', field.wire)[0]
        suffix = b'\x00\xd8' if field.tag == 6 else b'\xff'
        return struct.pack('<i', count + 1) + field.wire[4:] + suffix
    assert field.tag == 5, (field.path, field.tag)
    return next(wire for wire in donors if wire != field.wire)


def source_edit_outcome(version: int, original: bytes, fields: list[DialogEditField], index: int, replacement: bytes) -> tuple[bool, bool]:
    """Keep complete grammar acceptance separate from isolated-field preservation; e.g. source_edit_outcome(version, original, fields, 0, replacement)."""
    target = fields[index]
    edited = original[:target.start] + replacement + original[target.end:]
    parsed = actual_post_record(0, version, edited)
    if not parsed['success']: return False, False
    assert parsed['end'] == len(edited), (version, parsed['end'], len(edited))
    observed = [decode_edit_field(field) for field in dialog_fields_from_record(edited, parsed)]
    return True, isolated_edit_preserved(fields, observed, index, replacement)


def isolated_edit_preserved(original: list[DialogEditField], observed: list[DialogEditField], changed: int, replacement: bytes) -> bool:
    """Require every path, type, raw value and rebased extent, not just successful EOF; e.g. isolated_edit_preserved(original, observed, 0, replacement)."""
    if len(original) != len(observed): return False
    delta = len(replacement) - len(original[changed].wire)
    for index, (before, after) in enumerate(zip(original, observed, strict=True)):
        start = before.start + (delta if index > changed else 0)
        end = start + len(replacement) if index == changed else before.end + (delta if index > changed else 0)
        wire = replacement if index == changed else before.wire
        if (after.path, after.tag, after.start, after.end, after.wire) != (before.path, before.tag, start, end, wire): return False
    return True


def owned_edit_contexts() -> list[tuple[int, bytes, list[DialogEditField]]]:
    """Reuse every independently assembled parent input without narrowing the version corpus; e.g. owned_edit_contexts()."""
    contexts: list[tuple[int, bytes, list[DialogEditField]]] = []
    for family, version, _, raw in dialog_owned_cases():
        if family != 0: continue
        fields = [decode_edit_field(field) for field in dialog_fields(0, version, raw)]
        contexts.append((version, raw, fields))
    assert len(contexts) == 225, len(contexts)
    return contexts


def context_edit_packet(version: int, raw: bytes, fields: list[DialogEditField], donors: tuple[bytes, ...], counts: Counter[str]) -> bytes:
    """Record both Source outcomes and the complete literal replacement for each physical occurrence; e.g. context_edit_packet(version, raw, fields, donors, counts)."""
    packet = bytearray(struct.pack('<II', version, len(fields)))
    for index, field in enumerate(fields):
        replacement = edit_wire(field, donors)
        assert replacement != field.wire, field.path
        grammar, isolated = source_edit_outcome(version, raw, fields, index, replacement)
        counts[f'{field.tag}:grammar_{int(grammar)}:isolated_{int(isolated)}'] += 1
        packet.extend(struct.pack('<IIIIIII', index, field.tag, int(grammar), int(isolated), field.start, field.end, len(replacement)))
        packet.extend(replacement)
    return bytes(packet)


def verify_dialog_edit_matrix() -> None:
    """Freeze complete changed-value outcomes for all eight kinds over the full owned parent corpus; e.g. verify_dialog_edit_matrix()."""
    contexts = owned_edit_contexts()
    donors = tuple(dict.fromkeys(field.wire for _, _, fields in contexts for field in fields if field.tag == 5))
    assert len(donors) >= 2, len(donors)
    packet = bytearray(struct.pack('<II', 1, len(contexts)))
    counts: Counter[str] = Counter()
    for index, (version, raw, fields) in enumerate(contexts):
        packet.extend(context_edit_packet(version, raw, fields, donors, counts))
        print(json.dumps(dict(dialog_edit_context=index, version=version, fields=len(fields))), flush=True)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'edit-matrix.bin': bytes(packet)})
    input_hash = digest((FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog/owned.bin').read_bytes())
    dialog_baseline('edit-matrix-reference.json', dict(contexts=len(contexts), changed_fields=sum(counts.values()), input_sha256=input_hash, outcomes=dict(sorted(counts.items())), packets=summaries))
