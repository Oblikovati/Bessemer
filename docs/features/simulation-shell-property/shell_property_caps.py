"""Independent sufficient-payload cap controls and all-null resource cases (#159)."""
from collections.abc import Callable
from typing import cast
import struct
from owned_shell_property import empty_shell_base, ShellRawContext
from owned_thermal import thermal_reference
from owned_material import material_literal, material_texts, property_literal
from shell_property_projection import shell_presence_packet, shell_property_fields
from shell_property_metrics import shell_budget_packet
from shell_property_malformed import actual_shell_case
from shell_property_evidence import shell_baseline, FEATURE
from native_carriers import digest

CapBuilder = Callable[[int], tuple[int, bytes]]


def shell_null_prefix(version: int, subtype: int = 0) -> bytes:
    """Build the declared null-text base and derived widths independently; e.g. shell_null_prefix(840005)."""
    derived = bytearray(36)
    struct.pack_into('<i', derived, 12, subtype)
    if version >= 820017: derived.extend(bytes(8))
    if version > 830008:
        derived.extend(bytes(4))
        if version >= 840005: derived.extend(bytes(12))
    return empty_shell_base(version) + derived


def shell_null_tail(version: int) -> bytes:
    """Retain both independently gated final fields as null words/text; e.g. shell_null_tail(890006)."""
    return bytes((4 if version >= 870022 else 0) + (4 if version >= 890006 else 0))


def reference_cap_literal(count: int) -> tuple[int, bytes]:
    """Supply all complete old reference records on either side of the cap; e.g. reference_cap_literal(10000)."""
    return 0, struct.pack('<i', count) + thermal_reference(0) * count + shell_null_prefix(0)[4:]


def blob_cap_literal(count: int) -> tuple[int, bytes]:
    """Supply every declared blob byte inside a complete reference container; e.g. blob_cap_literal(1000000)."""
    reference = bytes(32) + struct.pack('<i', count) + b'\xaa' * count
    return 0, struct.pack('<i', 1) + reference + shell_null_prefix(0)[4:]


def layer_cap_literal(count: int) -> tuple[int, bytes]:
    """Use complete external-material layers without runtime-dependent flags; e.g. layer_cap_literal(10000)."""
    version = 830010
    layers = struct.pack('<4i', count, 0, 0, 1) + bytes(16 * count)
    return version, shell_null_prefix(version, 2) + layers + shell_null_tail(version)


def curve_cap_literal(count: int) -> tuple[int, bytes]:
    """Supply all fifteen double/word pairs per curve under a permitted mode; e.g. curve_cap_literal(50)."""
    version = 890011
    base = empty_shell_base(version)
    curves = struct.pack('<2i', 0, count) + bytes(184 * count)
    return version, base[:-8] + curves + shell_null_prefix(version)[len(base):] + shell_null_tail(version)


def optional_cap_literal(count: int) -> tuple[int, bytes]:
    """Populate the first base virtual body and its complete pair payload; e.g. optional_cap_literal(100000)."""
    version = 840005
    prefix = bytearray(shell_null_prefix(version))
    presence = 148 + 4 + 4
    struct.pack_into('<i', prefix, presence, 1)
    body = bytes(4) + struct.pack('<5i', 0, 4, 0, 0, count) + bytes(76) + bytes(16 * count)
    return version, prefix[:presence + 4] + body + prefix[presence + 4:]


def material_cap_literal(count: int) -> tuple[int, bytes]:
    """Insert complete independent Material properties and retain its continuation; e.g. material_cap_literal(10000)."""
    version = 830010
    material = bytearray(material_literal(version, 0, empty=True))
    count_offset = len(material_texts(2))
    header_end = count_offset + 16 + len(material_texts(3)) + 4 + len(material_texts(2))
    struct.pack_into('<i', material, count_offset, count)
    body = material[:header_end] + property_literal(version, 0, 0) * count + material[header_end:]
    layers = struct.pack('<4i', 1, 0, 0, 0) + bytes(16) + struct.pack('<i', 1) + body
    return version, shell_null_prefix(version, 2) + layers + shell_null_tail(version)


def shell_case_for_raw(kind: int, version: int, raw: bytes) -> ShellRawContext:
    """Bind a context with no required runtime lookup, without invented expected statistics; e.g. shell_case_for_raw(0, 0, raw)."""
    return ShellRawContext(kind=kind, version=version, raw=raw, layer_presence=None)


def shell_cap_builders() -> tuple[tuple[str, int, CapBuilder], ...]:
    """Keep each fixed cap separate from permissive aggregate caller budgets; e.g. shell_cap_builders()."""
    return (('references', 10000, reference_cap_literal), ('blob', 1000000, blob_cap_literal),
            ('layers', 10000, layer_cap_literal), ('curves', 50, curve_cap_literal),
            ('optional_pairs', 100000, optional_cap_literal), ('material_properties', 10000, material_cap_literal))


def verify_shell_caps() -> None:
    """Run complete positive and one-above controls for both concrete kinds; e.g. verify_shell_caps()."""
    import sys
    packet, rows = bytearray(), []
    for name, maximum, builder in shell_cap_builders():
        for count in (maximum, maximum + 1):
            version, raw = builder(count)
            for kind in range(2):
                parsed = actual_shell_case(shell_case_for_raw(kind, version, raw), raw)
                success = bool(parsed['success'])
                assert success == (count == maximum), (name, count, parsed)
                fields = len(shell_property_fields(raw, version, parsed)) if success else 0
                packet.extend(struct.pack('<IIIII', kind, version, int(success), fields, len(raw)) + raw)
                rows.append(dict(name=name,kind=kind,count=count,success=success,fields=fields,bytes=len(raw),sha256=digest(raw)))
    path = FEATURE.parents[2] / 'tests/fixtures/simulation-shell-property/hardcaps.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    shell_baseline('hardcap-reference.json', dict(contexts=len(rows),cases=rows,bytes=len(packet),sha256=digest(packet)))


def verify_shell_null_budgets() -> None:
    """Pin genuinely zero outer text/entry budgets under all-null frames; e.g. verify_shell_null_budgets()."""
    import sys
    packet, count = bytearray(), 0
    for kind in range(2):
        for version in (0, 800000, 840005, 891106, 2147483647):
            raw = shell_null_prefix(version) + shell_null_tail(version)
            parsed = actual_shell_case(shell_case_for_raw(kind, version, raw), raw)
            assert parsed['success'] and parsed['end'] == len(raw), parsed
            packet.extend(shell_budget_packet(kind, version, raw, parsed, None))
            count += 1
    path = FEATURE.parents[2] / 'tests/fixtures/simulation-shell-property/null-budgets.bin'
    if '--write' in sys.argv: path.write_bytes(packet)
    assert path.read_bytes() == packet
    shell_baseline('null-budget-reference.json', dict(contexts=count,bytes=len(packet),sha256=digest(packet)))
