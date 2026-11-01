"""C856 explicit-field hidden-parts migration, distinct from stored words and visibility (#166)."""
from collections.abc import Callable
from typing import cast
import importlib
import json
import struct
import unittest
from native_carriers import RESEARCH
from dialog_evidence import dialog_baseline
from legend_word_sources import legend_word_source_transitions


def hidden_migration_report() -> dict[str, object]:
    """Retain all116 original migration observations and the complete reader interval; e.g. hidden_migration_report()."""
    report = cast(dict[str, object], json.loads((RESEARCH / 'cosworks-post-style-legacy-hidden-policy-native-evidence.json').read_bytes()))
    assert report['evidence'] == 'C-856' and report['target_executed'] is False and report['coverage_credit'] == 0
    assert report['streams'] == len(cast(list[object], report['rows'])) == 116
    assert report['stored_values'] == {'0': 116} and report['derived_pairs'] == {'(0, 0)': 116}
    assert cast(dict[str, object], report['tests'])['run'] == 2
    dialog_baseline('hidden-migration-c856-reference.json', report)
    return report


def verify_hidden_migration_sources() -> None:
    """Admit only C856's exact historical color-report transition beside full current replay; e.g. verify_hidden_migration_sources()."""
    historical = 'a1a672e78546d9dbddc7ee55b6315916003c8776dfe7acfd7ea1907495ccb67c'
    transitions = legend_word_source_transitions(hidden_migration_report(), historical)
    dialog_baseline('hidden-migration-source-transitions.json', dict(transitions=transitions))


def hidden_instruction_pair(word: int) -> tuple[int, int]:
    """Emulate the bound little-endian stores and unsigned DEC/JBE gate; e.g. hidden_instruction_pair(2)."""
    assert type(word) is int and -2147483648 <= word <= 2147483647, word
    pair = bytearray(struct.pack('<II', 0xdeadbeef, word & 0xffffffff))
    if word == 0: pair[:4] = bytes(4)
    elif ((word - 1) & 0xffffffff) <= 2: pair[:] = struct.pack('<Q', 1)
    else: pair[:] = struct.pack('<II', 1, 1)
    return cast(tuple[int, int], struct.unpack('<ii', pair))


def require_hidden_model_rejection(word: object, version: object) -> None:
    """Require explicit signed32 input and the exact migration version interval; e.g. require_hidden_model_rejection(True, 700006)."""
    model = importlib.import_module('cosworks_post_style_legacy_hidden_policy')
    try:
        model.migrate(word, version)
    except ValueError:
        return
    raise AssertionError((word, version, 'expected signed32 word and explicit version700006..840007 rejection'))


def verify_hidden_store_policy() -> None:
    """Check all migration branches, adjacent-word erasure and input boundaries independently; e.g. verify_hidden_store_policy()."""
    model = importlib.import_module('cosworks_post_style_legacy_hidden_policy')
    cases = ((-2147483648, (1, 1)), (-1, (1, 1)), (0, (0, 0)), (1, (1, 0)),
        (2, (1, 0)), (3, (1, 0)), (4, (1, 1)), (2147483647, (1, 1)))
    for word, expected in cases:
        assert hidden_instruction_pair(word) == expected
        for version in (700006, 800000, 840007):
            assert model.migrate(word, version) == dict(stored_word_1f0=word,
                derived_hidden_parts_option=expected[0], runtime_word_1f0=expected[1])
    for word, version in ((0, 700005), (0, 840008), (0, True), (True, 700006),
        (-2147483649, 700006), (2147483648, 700006), (1.0, 700006), (None, 700006)):
        require_hidden_model_rejection(word, version)
    dialog_baseline('hidden-migration-store-reference.json', dict(cases=[dict(word=word, pair=list(pair)) for word, pair in cases],
        qword_one_bytes=struct.pack('<Q', 1).hex(), native_target_executed=False,
        original_word_preserved=True, nonzero_native_migrations_observed=False))


def hidden_migration_functions() -> tuple[Callable[[], None], ...]:
    """Keep complete reader provenance, native qualifications and byte-store semantics together; e.g. hidden_migration_functions()."""
    from hidden_migration_bindings import verify_hidden_migration_bindings, verify_hidden_migration_rea_capture
    from hidden_migration_projection import verify_hidden_migration_native, verify_hidden_migration_owned
    from hidden_migration_packets import verify_hidden_operation_packets, verify_hidden_projection_packets
    return (verify_hidden_migration_sources, verify_hidden_migration_bindings, verify_hidden_migration_rea_capture,
        verify_hidden_store_policy, verify_hidden_migration_native, verify_hidden_migration_owned,
        verify_hidden_operation_packets, verify_hidden_projection_packets)


def run_hidden_migration_checks() -> unittest.TestResult:
    """Retain the original migration and eight catalog/style/color tests; e.g. run_hidden_migration_checks()."""
    from dialog_evidence import verify_dialog_sources
    from dialog_field_evidence import verify_dialog_field_sources
    checks = (verify_dialog_sources, verify_dialog_field_sources) + hidden_migration_functions()
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for name in ('test_cosworks_post_style_legacy_hidden_policy', 'test_cosworks_post_style_field_catalog',
        'test_cosworks_post_style', 'test_cosworks_post_color_fields'):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(importlib.import_module(name)))
    return unittest.TextTestRunner(verbosity=2).run(suite)
