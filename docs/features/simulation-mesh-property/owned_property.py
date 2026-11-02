"""Complete independent Mesh Property literals across every documented threshold (#163)."""
from typing import TypedDict
import re
from property_literal_values import PropertyBlocks, PropertyLiteralSpan, property_count, property_words, assemble_property_blocks
from property_initial_literals import initial_property_blocks
from property_collection_literals import property_settings_blocks, direct_property_blocks, property_array_objects, property_late_blocks


class PropertyDirectSpan(TypedDict):
    index: int
    start: int
    end: int


class OwnedMeshProperty(TypedDict):
    version: int
    profile: int
    null_texts: bool
    raw: bytes
    spans: list[PropertyLiteralSpan]
    records: list[PropertyDirectSpan]


def owned_property_versions() -> list[int]:
    """Cross every parent/reference/nested gate and every omitted-window version; e.g. owned_property_versions()."""
    gates = (800005, 800021, 800035, 800038, 800040, 800074, 800075, 800078, 810000, 810004, 810006,
             830018, 830020, 830034, 840002, 840013, 840017, 840018, 850001, 850002, 850029, 850030,
             850035, 870018, 890021, 890104, 890111, 890502, 890512, 890514, 890601, 890605,
             890703, 890907, 890909, 891103)
    return sorted({0, 2147483647, 810002, *(gate + delta for gate in gates for delta in (-1, 0, 1))})


def property_duration_blocks(version: int, profile: int) -> PropertyBlocks:
    """Preserve the implicit single Windows-long before the explicit count gate; e.g. property_duration_blocks(840016, 2)."""
    count = 1 if version < 840017 else (0, 1, 2, 3, 2)[profile]
    blocks = [property_count('mesh.duration_count', count)] if version >= 840017 else []
    return blocks + [('mesh.duration_words', property_words(count, profile))]


def property_direct_spans(spans: list[PropertyLiteralSpan]) -> list[PropertyDirectSpan]:
    """Compute literal record ownership before asking any parser for boundaries; e.g. property_direct_spans(spans)."""
    records: dict[int, PropertyDirectSpan] = {}
    for span in spans:
        match = re.match(r'mesh\.record(\d+)\.', span['label'])
        if match is None: continue
        index = int(match[1])
        if index not in records: records[index] = PropertyDirectSpan(index=index, start=span['offset'], end=span['end'])
        records[index]['end'] = span['end']
    return list(records.values())


def owned_property_literal(version: int, profile: int, null_texts: bool = False) -> OwnedMeshProperty:
    """Assemble a complete literal without consulting a decoder; e.g. owned_property_literal(891103, 2)."""
    assert type(version) is int and 0 <= version <= 2147483647, f'{version}; expected an explicit nonnegative signed version'
    assert type(profile) is int and 0 <= profile < 5, f'{profile}; expected literal profile 0..4'
    assert type(null_texts) is bool and (not null_texts or profile == 0), f'{profile}/{null_texts}; expected null-only texts on empty profile 0'
    blocks = initial_property_blocks(version, profile, null_texts)
    if version >= 800021: blocks.extend(property_settings_blocks(version, profile))
    if version >= 800074: blocks.extend(direct_property_blocks(version, profile))
    if version >= 810006: blocks.extend(property_duration_blocks(version, profile))
    if version >= 840013: blocks.append(('mesh.word9b0', property_words(1, profile)))
    if version >= 850001: blocks.extend(property_array_objects(profile))
    blocks.extend(property_late_blocks(version, profile))
    raw, spans = assemble_property_blocks(blocks)
    return OwnedMeshProperty(version=version, profile=profile, null_texts=null_texts, raw=raw, spans=spans, records=property_direct_spans(spans))


def owned_properties() -> list[OwnedMeshProperty]:
    """Cover zero, singleton, repeated, mixed-mode and null-archive contexts; e.g. owned_properties()."""
    cases = [owned_property_literal(version, profile) for version in owned_property_versions() for profile in range(5)]
    return cases + [owned_property_literal(version, 0, True) for version in owned_property_versions()]
