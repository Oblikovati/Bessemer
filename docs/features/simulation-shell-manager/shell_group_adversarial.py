"""Actual MFC/count mutants, complete truncation checks and sufficient-payload record caps (#156)."""
from typing import cast
from collections.abc import Callable
import struct
from owned_shell import OwnedShellGroup, owned_shell_groups
from shell_group_packets import group_primitives
from thermal_malformed import encode_mutation_cases
from native_carriers import digest


def group_mutant_images(case: OwnedShellGroup, parsed: dict[str, object]) -> set[bytes]:
    """Retain valid changed controls alongside framing/count failures; e.g. group_mutant_images(case, parsed)."""
    raw = case['raw']
    images = {struct.pack('<i', count) + raw[4:] for count in (-1, 0, 1, 2, 99999, 100000, 100001, 2147483647)}
    if not case['records']: return images
    fields = {field['path']: field for field in group_primitives(raw, parsed)}
    first, second, signed = (fields['record0.' + label] for label in ('string0', 'string1', 'word28'))
    images.add(raw[:first['start']] + b'\xff\xfe\xff\x00' + raw[first['end']:])
    images.add(raw[:first['start']] + b'\x01' + raw[first['end']:])
    images.add(raw[:second['start']] + b'\xff\xfe\xff\xff\xff\xff\xff\xff\xff\xff' + raw[second['end']:])
    payload_size = len(cast(str, second['text']).encode('utf-16-le'))
    offset = second['end'] - payload_size
    images.add(raw[:offset] + b'\x00\xd8' + raw[offset + 2:])
    replacement = struct.pack('<i', struct.unpack('<i', signed['wire'])[0] ^ 64)
    images.add(raw[:signed['start']] + replacement + raw[signed['end']:])
    return images


def group_mutation_packet() -> tuple[bytes, int, int]:
    """Freeze actual outcomes for every complete owned context and exact splice; e.g. group_mutation_packet()."""
    from cosworks_shell_group_layout import parse
    cases: list[tuple[int, int, bytes, list[tuple[bool, bytes]]]] = []
    for case in owned_shell_groups():
        version, profile, raw = case['version'], case['wire_profile'], case['raw']
        parsed = parse(raw, version, wire_profile=profile)
        assert parsed['success']
        outcomes = [(cast(bool, parse(image, version, wire_profile=profile)['success']), image)
                    for image in sorted(group_mutant_images(case, parsed))]
        cases.append((int(profile == 'writer'), version, raw, outcomes))
    return encode_mutation_cases(cases)


def verify_group_prefixes(store: Callable[[str, object], None]) -> None:
    """Check every proper prefix without retaining an enormous duplicate corpus; e.g. verify_group_prefixes(store)."""
    from cosworks_shell_group_layout import parse
    rows: list[dict[str, object]] = []
    for case in owned_shell_groups():
        raw, version, profile = case['raw'], case['version'], case['wire_profile']
        for end in range(len(raw)):
            assert not parse(raw[:end], version, wire_profile=profile)['success'], (version, profile, end)
        assert not parse(raw + b'\x00', version, wire_profile=profile)['success']
        rows.append(dict(version=version, wire_profile=profile, pattern=case['pattern'], prefixes_checked=len(raw), trailing_rejected=True))
    store('group-prefix-reference.json', dict(cases=len(rows), prefixes=sum(cast(int, row['prefixes_checked']) for row in rows), rows=rows))


def verify_group_hardcap(store: Callable[[str, object], None]) -> None:
    """Supply all complete empty-string records on both sides of the fixed cap; e.g. verify_group_hardcap(store)."""
    from cosworks_shell_group_layout import parse
    rows: list[dict[str, object]] = []
    for count in (100000, 100001):
        raw = struct.pack('<i', count) + bytes(count * 32)
        parsed = parse(raw, 890015)
        assert parsed['success'] == (count == 100000)
        if parsed['success']:
            assert parsed['end'] == len(raw) and len(parsed['records']) == count
            assert len(parsed['spans']) == 1 + count * 10 and parsed['reconstruction_sha256'] == digest(raw)
        rows.append(dict(version=890015, wire_profile='reader', count=count, bytes=len(raw), sha256=digest(raw), success=parsed['success']))
    store('group-hardcap-reference.json', rows)
