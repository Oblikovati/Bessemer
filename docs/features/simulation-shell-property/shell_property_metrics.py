"""Independent aggregate budgets and operation statistics from actual shell calls (#159)."""
from typing import cast
import re
import struct
from material_adversarial import material_budget_metrics
from shell_property_projection import shell_property_fields, shell_presence_packet
from sheet_projection import sheet_embedded_fields


def inline_material_entries(raw: bytes, version: int, span: dict[str, object]) -> int:
    """Include declared Material counts even when their conditional payload is absent; e.g. inline_material_entries(raw, 840005, span)."""
    from cosworks_material_layout import parse
    body = raw[cast(int, span['offset']):cast(int, span['end'])]
    parsed = parse(body, version)
    assert parsed['success'] and parsed['end'] == len(body), parsed
    return material_budget_metrics(body, cast(list[dict[str, object]], parsed['spans']))[0]


def shell_entry_count(raw: bytes, version: int, spans: list[dict[str, object]]) -> int:
    """Count only documented record/pair declarations, excluding text/blob extents; e.g. shell_entry_count(raw, 891106, spans)."""
    entries = 0
    for span in spans:
        label, start = cast(str, span['label']), cast(int, span['offset'])
        if label in ('references.count', 'base.curve_count', 'shell.layer_count'):
            entries += struct.unpack_from('<i', raw, start)[0]
        if re.fullmatch(r'base\.virtual[01]\.body\.header', label):
            entries += struct.unpack_from('<i', raw, start + 16)[0]
        if re.fullmatch(r'layer\d+\.material', label): entries += inline_material_entries(raw, version, span)
    return entries


def shell_text_maximum(fields: list[bytes]) -> int:
    """Observe complete independently typed text frames, including inline Material; e.g. shell_text_maximum(fields)."""
    maximum = 0
    for field in fields:
        name_size = struct.unpack_from('<I', field)[0]
        if struct.unpack_from('<I', field, 4 + name_size)[0] == 3:
            maximum = max(maximum, struct.unpack_from('<I', field, 16 + name_size)[0])
    return maximum


def shell_statistics(raw: bytes, parsed: dict[str, object], nested: list[int]) -> bytes:
    """Retain independent shared-base endpoint, subtype and complete layer/body counts; e.g. shell_statistics(raw, parsed, nested)."""
    spans = cast(list[dict[str, object]], parsed['spans'])
    layers = sum(struct.unpack_from('<i', raw, cast(int, span['offset']))[0] for span in spans if span['label'] == 'shell.layer_count')
    materials = sum(bool(re.fullmatch(r'layer\d+\.material', cast(str, span['label']))) for span in spans)
    descriptors = sum(bool(re.fullmatch(r'layer\d+\.descriptor[12]\.double', cast(str, span['label']))) for span in spans)
    return struct.pack('<IiIIIII', parsed['base_end'], parsed['subtype'], layers, materials, descriptors, len(nested), sum(nested))


def shell_archive_counts(raw: bytes, archives: list[dict[str, object]]) -> list[int]:
    """Observe every scoped embedded primitive call independently; e.g. shell_archive_counts(raw, archives)."""
    counts: list[int] = []
    for archive in archives:
        start, end = cast(int, archive['offset']), cast(int, archive['end'])
        observed = sheet_embedded_fields(raw[start:end])
        counts.append(struct.unpack_from('<I', observed, 4 + end - start)[0])
    return counts


def shell_budget_packet(kind: int, version: int, raw: bytes, parsed: dict[str, object], presence: list[bool] | None) -> bytes:
    """Freeze seven exact resource axes and complete coverage statistics independently; e.g. shell_budget_packet(0, 891106, raw, parsed, presence)."""
    assert parsed['success'] and parsed['end'] == len(raw), parsed
    spans = cast(list[dict[str, object]], parsed['spans'])
    archives = cast(list[dict[str, object]], parsed['archive_records'])
    fields = shell_property_fields(raw, version, parsed)
    entries, text = shell_entry_count(raw, version, spans), shell_text_maximum(fields)
    root = max((cast(int, item['end']) - cast(int, item['offset']) for item in archives), default=0)
    objects = max((sum(cast(dict[str, int], item['classes']).values()) for item in archives), default=0)
    nested = shell_archive_counts(raw, archives)
    return (shell_presence_packet(presence) + struct.pack('<III', kind, version, len(raw)) + raw
            + struct.pack('<6I', entries, text, root, objects, len(fields), max(nested, default=0)) + shell_statistics(raw, parsed, nested))
