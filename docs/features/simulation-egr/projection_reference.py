"""Compose complete actual-reader projection bytes without shared mutable oracle state."""
from collections.abc import Callable
from owned_egr import TableCase, table_cases, table_image, name_cases, egroup_cases
import name_reference

Words = Callable[[list[int]], bytes]
Span = Callable[[bytes], bytes]
TableOracle = Callable[[bytes], bytes]
FieldOracle = Callable[[bytes, TableCase, int, int, int], bytes]


def complete_packet(table_oracle: TableOracle, field_oracle: FieldOracle, words: Words, span: Span) -> bytes:
    """Keep complete stored metadata, field edits, names and EGROUP outcomes in total order."""
    parts = [b'EGRV', table_packet(table_oracle, words, span), field_packet(field_oracle, words, span)]
    names = name_cases()
    parts.append(words([len(names)]))
    parts.extend(span(name_reference.name_oracle(code, destination, words, span)) for code, destination in names)
    groups = egroup_cases()
    parts.append(words([len(groups)]))
    parts.extend(span(name_reference.egroup_oracle(raw, logical, offset, words, span)) for raw, logical, offset in groups)
    return b''.join(parts)


def table_packet(oracle: TableOracle, words: Words, span: Span) -> bytes:
    """Retain every actual field/row/partition output and separate descriptor-assignment bytes."""
    import simulationegr as reader
    cases = table_cases()
    parts = [words([len(cases)])]
    for case in cases:
        raw = table_image(case)
        descriptor = reader.descriptor_writer_view(raw)
        parts.extend([span(oracle(raw)), span(descriptor['projected_descriptor_bytes'])])
    return b''.join(parts)


def field_packet(oracle: FieldOracle, words: Words, span: Span) -> bytes:
    """Compose independently patched complete-file field goldens without replacing actual range selection."""
    cases = table_cases()
    parts = [words([len(cases)])]
    for case in cases:
        raw = table_image(case)
        contexts = [(logical, first, last) for logical, number in enumerate(case.directory, 1) if number
                    for first in range(1, len(case.widths) + 1) for last in range(first, len(case.widths) + 1)]
        parts.append(words([len(contexts)]))
        parts.extend(span(oracle(raw, case, logical, first, last)) for logical, first, last in contexts)
    return b''.join(parts)
