"""Complete explicit ParameterSets contexts and independently classified edits (#133)."""
from dataclasses import dataclass, replace
from cosworkspersist import read_parameter_sets, ParameterSets
from freeze_parameters import carriers
from literal_parameters import parameter_bytes
from owned_persistence import literal_texts, word
from parameter_fields import fields, structural, ParameterField


@dataclass(frozen=True)
class ParameterCase:
    version: int
    raw: bytes
    maximum_records: int = 10000
    maximum_table_items: int = 10000
    maximum_text_bytes: int = 0x100000
    ordinal: int = -1
    replacement_kind: int = -1
    replacement: bytes = b''
    accepted: bool = True


def read(case: ParameterCase) -> ParameterSets:
    """Validate actual complete framing before deriving fields or iterating counts."""
    return read_parameter_sets(case.raw, case.version, max_records=case.maximum_records, max_table_items=case.maximum_table_items, max_string_bytes=case.maximum_text_bytes)


def selected(fields: list[ParameterField], table_count: int) -> list[tuple[int, ParameterField]]:
    """Large fixtures retain all fields but probe edits at explicit retention/array boundaries."""
    if table_count <= 100:
        return list(enumerate(fields))
    indices = {0, 1, 9998, 9999, 10000, table_count - 1}
    return [(index, field) for index, field in enumerate(fields) if field.role[0] not in (4, 10) or (field.role[1] if field.role[0] == 4 else field.role[2]) in indices]


def changed(field: ParameterField) -> bytes:
    """Literal sign-bit flips or a distinct complete raw text frame, never float arithmetic."""
    if field.kind == 3:
        return literal_texts()[3] if field.raw != literal_texts()[3] else literal_texts()[2]
    return (int.from_bytes(field.raw, 'little') ^ (1 << (len(field.raw) * 8 - 1))).to_bytes(len(field.raw), 'little')


def edits(case: ParameterCase) -> list[ParameterCase]:
    """Keep authentic no-ops, changed complete fields and explicit structural rejections."""
    record = read(case)
    cases = []
    for ordinal, field in selected(fields(record), record.table_count):
        cases.append(replace(case, ordinal=ordinal, replacement_kind=field.kind, replacement=field.raw))
        cases.append(replace(case, ordinal=ordinal, replacement_kind=field.kind, replacement=changed(field), accepted=not structural(field, case.version)))
    return cases


def owned_cases() -> list[ParameterCase]:
    """Use every boundary context and independent literal sections, including complete large arrays."""
    cases = []
    versions = (0, 700011, 700012, 700013, 700014, 700018, 700019, 810025, 810026, 820015, 820016, 830009, 830010, 0x7fffffff)
    for version in versions:
        for count in ((100,) if version < 810026 else (0, 1, 2)):
            cases.extend(context_cases(version, count))
    for count in (10000, 10001):
        case = ParameterCase(830010, parameter_bytes(830010, count, (literal_texts()[2],)), 1, count, 8)
        cases += [case, *edits(case)]
    return cases + invalid_cases()


def context_cases(version: int, count: int) -> list[ParameterCase]:
    """Keep complete empty, null-name and multi-record odd/surrogate cases per context."""
    frames = literal_texts()
    cases = []
    for texts in ((), (frames[0],), (frames[1], frames[3], frames[2])):
        case = ParameterCase(version, parameter_bytes(version, count, texts), 3, max(2, count), 8)
        cases += [case, *edits(case)]
    return cases


def invalid_cases() -> list[ParameterCase]:
    """Declare malformed shapes and smaller budgets independently before parser execution."""
    raw = parameter_bytes(830010, 2, tuple(literal_texts()[:4]))
    cases = [ParameterCase(830010, raw[:end], accepted=False) for end in range(len(raw))]
    cases.append(ParameterCase(830010, raw + b'\0', accepted=False))
    for value in (-1, 0x7fffffff):
        cases.append(ParameterCase(830010, word(value) + raw[4:], maximum_table_items=0x7fffffff, accepted=False))
        cases.append(ParameterCase(830010, raw[:28] + word(value) + raw[32:], maximum_records=0x7fffffff, accepted=False))
    cases += [ParameterCase(830010, raw, maximum_records=3, accepted=False), ParameterCase(830010, raw, maximum_table_items=1, accepted=False), ParameterCase(830010, raw, maximum_text_bytes=2, accepted=False)]
    cases.append(ParameterCase(0, parameter_bytes(0, 100, ()), maximum_table_items=99, accepted=False))
    cases.append(ParameterCase(830010, parameter_bytes(830010, 10001, (literal_texts()[2],)), accepted=False))
    return cases


def native_cases() -> list[ParameterCase]:
    """Retain all forty-three candidate contexts, with every field edit on accepted native bodies."""
    cases = []
    for row in carriers():
        for candidate in row['layout_candidates']:
            case = ParameterCase(candidate['version'], bytes.fromhex(row['raw_hex']), accepted=candidate['success'])
            cases.append(case)
            cases.extend(edits(case) if case.accepted else ())
    return cases
