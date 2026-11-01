"""Original unit/context alternatives and complete stored parent projections (#166)."""
from collections import Counter
from itertools import product
from typing import cast
import struct
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from offset_fringe_projection import parent_decision_spans, native_parent_decisions
from vector_projection import original_vector_identities
from unit_evidence import unit_reports
from unit_packets import UnitArguments, unit_case, unit_result, unit_packet


def parent_unit_arguments(spans: dict[str, dict[str, object]], be8: int | None, flag: int | None) -> UnitArguments:
    """Read stored BEC but never reinterpret a similarly named BE8 reference as loaded context; e.g. parent_unit_arguments(spans, None, None)."""
    header = bytes.fromhex(cast(str, spans['fourteen_words0c_28_30_3c_518_51c']['raw_hex']))
    assert len(header) == 56, len(header)
    words = struct.unpack('<14i', header)
    span = spans.get('dialog.word_bec')
    bec = None if span is None else struct.unpack('<i', bytes.fromhex(cast(str, span['raw_hex'])))[0]
    return words[0], words[1], words[3], be8, bec, flag


def original_unit_arguments(row: dict[str, object], spans: dict[str, dict[str, object]]) -> UnitArguments:
    """Require every original raw/native input and retain the original unknown runtime words; e.g. original_unit_arguments(row, spans)."""
    arguments = parent_unit_arguments(spans, None, None)
    expected = cast(dict[str, int | None], row['inputs'])
    assert arguments == tuple(expected[key] for key in ('analysis', 'plot_type', 'component', 'word_be8', 'word_bec', 'global_57b68')), row
    status, code = unit_result(arguments)
    assert (status == 0) == (row['status'] == 'modeled'), row
    assert (code if status == 0 else None) == row['unit_type_code'], row
    return arguments


def verify_unit_native() -> None:
    """Replay all224 native states and retain all56 context stops plus explicit alternatives; e.g. verify_unit_native()."""
    numeric, flag_report, registry = unit_reports()
    original_vector_identities([numeric])
    available = {(digest(raw), version): parent_decision_spans(version, raw) for version, raw in native_parent_decisions()}
    rows = cast(list[dict[str, object]], numeric['rows'])
    arguments = [original_unit_arguments(row, available[(cast(str, row['sha256']), cast(int, row['selected_version']))]) for row in rows]
    missing = [row for row, args in zip(rows, arguments, strict=True) if unit_result(args)[0] != 0]
    assert len(missing) == flag_report['retained_missing_context_records'] == registry['retained_missing_context_records'] == 56
    alternatives = unit_flag_alternatives(flag_report, missing)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'target', {
        'simulation-unit-type-native.bin': unit_packet([unit_case(args) for args in arguments]),
        'simulation-unit-flag-alternatives.bin': unit_packet(alternatives)})
    dialog_baseline('unit-native-reference.json', dict(native_contexts=224, modeled=168, unresolved=56,
        explicit_flag_alternatives=len(alternatives), actual_runtime_flag=None, additional_stored_byte_credit=0, packets=summaries))


def unit_flag_alternatives(report: dict[str, object], missing: list[dict[str, object]]) -> list[bytes]:
    """Keep conditional0/1/2 flag choices separate from unknown actual per-file runtime state; e.g. unit_flag_alternatives(report, missing)."""
    rows = cast(list[dict[str, object]], report['rows'])
    assert Counter(unit_identity(row) for row in rows) == Counter(unit_identity(row) for row in missing)
    stored_inputs = {unit_identity(row): row['inputs'] for row in missing}
    cases: list[bytes] = []
    for row in rows:
        assert row['actual_runtime_flag'] is None, row
        stored = cast(dict[str, int | None], row['stored_inputs'])
        assert stored == stored_inputs[unit_identity(row)], row
        assert stored['global_57b68'] is None
        for alternative in cast(list[dict[str, object]], row['conditional_alternatives']):
            args = dict(stored, global_57b68=cast(int, alternative['actual_freq_disp_flag']))
            arguments = cast(UnitArguments, tuple(args[key] for key in ('analysis', 'plot_type', 'component', 'word_be8', 'word_bec', 'global_57b68')))
            status, code = unit_result(arguments)
            assert status == 0 and code == alternative['unit_type_code'] and alternative['status'] == 'modeled', alternative
            cases.append(unit_case(arguments))
    assert len(cases) == 168
    return cases


def unit_identity(row: dict[str, object]) -> tuple[str, str, str, int]:
    """Normalize each original identity once without merging same-byte files or streams; e.g. unit_identity(row)."""
    return cast(str, row['path']), cast(str, row['stream']), cast(str, row['sha256']), cast(int, row['selected_version'])


def unit_parent_projection(version: int, raw: bytes) -> bytes:
    """Capture raw parent controls under16 explicit runtime-context alternatives, never a guessed default; e.g. unit_parent_projection(890620, raw)."""
    spans = parent_decision_spans(version, raw)
    contexts = list(product((None, 0, 2, -1), (None, 0, 1, 2)))
    rows = [unit_case(parent_unit_arguments(spans, be8, flag)) for be8, flag in contexts]
    return struct.pack('<II', version, len(raw)) + raw + unit_packet(rows)


def verify_unit_projection() -> None:
    """Preserve every complete parent and every supplied/missing BE8/global choice without changing stored BEC or bytes; e.g. verify_unit_projection()."""
    owned = [(version, raw) for family, version, _, raw in dialog_owned_cases() if family == 0]
    native = native_parent_decisions()
    assert len(owned) == 225
    packets = [unit_packet([unit_parent_projection(*case) for case in cases]) for cases in (owned, native)]
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'unit-parent-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-unit-parent-projection-native.bin': packets[1]}))
    dialog_baseline('unit-projection-reference.json', dict(owned_contexts=225, native_contexts=224,
        explicit_context_choices=16, additional_stored_byte_credit=0, packets=summaries))
