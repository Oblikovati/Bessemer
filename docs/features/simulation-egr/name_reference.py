"""Independent solver-name and conditional EGROUP oracle packets (#124)."""
import struct
import unittest
from collections.abc import Callable
from owned_egr import SOLVER_NAMES, name_cases, egroup_cases


def name_oracle(code: int, destination: bytes, words: Callable[[list[int]], bytes], span: Callable[[bytes], bytes]) -> bytes:
    """Distinguish null-source incompleteness from a complete unchanged unknown-code result."""
    import simulationelementnames as reader
    names = dict(SOLVER_NAMES)
    assert reader.NAMES == names
    prefix = None if code not in names else names[code].encode('ascii') + b'\0'
    if prefix is not None and len(prefix) > len(destination):
        with unittest.TestCase().assertRaises(ValueError):
            reader.project(code, destination)
        return words([0])
    actual = reader.project(code, destination)
    status = 'matched_null_source' if code == 0 else 'known_name' if prefix is not None else 'unmatched_no_write'
    expected = None if code == 0 else destination if prefix is None else prefix + destination[len(prefix):]
    assert actual['status'] == status and actual['output'] == expected
    assert actual['native_return'] == (None if code == 0 else 0) and not actual['wire_field_binding']
    kind = {'known_name': 1, 'matched_null_source': 2, 'unmatched_no_write': 3}[status]
    return words([1, kind, int(expected is not None), int(code != 0), 0]) + span(b'' if expected is None else expected)


def freeze_names(fixture: Callable[[str, bytes], None], report: Callable[[str, object], None], words: Callable[[list[int]], bytes], span: Callable[[bytes], bytes], sha: Callable[[bytes], str]) -> None:
    """Keep all 33 known names and explicit raw-destination preservation cases."""
    cases = name_cases()
    packet = words([len(cases)])
    for code, destination in cases:
        packet += words([code]) + span(destination) + span(name_oracle(code, destination, words, span))
    fixture('names.bin', b'EGRN' + packet)
    report('name-reference.json', dict(known_codes=33, contexts=len(cases), accepted_contexts=150, insufficient_destination_rejections=33, oracle_sha256=sha(b'EGRN' + packet)))


def egroup_oracle(raw: bytes, logical: int, offset: int, words: Callable[[list[int]], bytes], span: Callable[[bytes], bytes]) -> bytes:
    """Bind signed integer permutation, wrapping and literal ASCII line bytes to the actual reader."""
    import simulationegr as reader
    actual = reader.egroup_export_view(raw, column_widths=[4] * 32, logical_ordinal=logical, group_offset=offset)
    identifier, code, *parameters = struct.unpack_from('<10i', raw)
    name = dict(SOLVER_NAMES).get(code)
    kind = 0 if identifier != logical else 1 if name is not None else 2
    expected_status = ('skipped_identifier_mismatch', 'known_name_projection', 'requires_name_lookup_state')[kind]
    assert actual['status'] == expected_status and not actual['complete_file_schema']
    if kind == 0:
        assert actual['output'] is None
        return words([kind])
    ordered = [parameters[index] for index in (0, 1, 2, 3, 4, 5, 7, 6)]
    group = (logical + offset) % 0x100000000
    group = group if group < 0x80000000 else group - 0x100000000
    return checked_group_line(actual, kind, group, name, ordered, words, span)


def checked_group_line(actual: dict[str, object], kind: int, group: int, name: str | None, ordered: list[int], words: Callable[[list[int]], bytes], span: Callable[[bytes], bytes]) -> bytes:
    """Construct the expected line from independent literal pieces, never native formatter execution."""
    expected = None if name is None else (f' EGROUP, {group} {name} ,' + ''.join(f' {value},' for value in ordered) + '\n').encode('ascii')
    assert actual['output_group_number'] == group and actual['output_parameter_int32'] == ordered
    assert actual['output'] == expected
    return words([kind, group, *ordered, int(expected is not None)]) + span(b'' if expected is None else expected)


def freeze_groups(fixture: Callable[[str, bytes], None], report: Callable[[str, object], None], words: Callable[[list[int]], bytes], span: Callable[[bytes], bytes], sha: Callable[[bytes], str]) -> None:
    """Own all known/null/unknown codes with mismatch and signed-boundary ordinal/offset contexts."""
    cases = egroup_cases()
    packet = b'EGRO' + words([len(cases)])
    for raw, logical, offset in cases:
        packet += span(raw) + words([logical, offset]) + span(egroup_oracle(raw, logical, offset, words, span))
    fixture('groups.bin', packet)
    report('group-reference.json', dict(contexts=len(cases), known_lines=396, skipped_identifier_mismatches=468, incomplete_name_states=72, oracle_sha256=sha(packet)))
