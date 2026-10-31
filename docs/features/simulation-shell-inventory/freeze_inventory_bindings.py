"""Freeze and test the complete original ShellManager denominator before fresh replay (#160)."""
from typing import cast
import json
import sys
import unittest

from shell_inventory_bindings import (
    FEATURE, complete_identity_match, original_shell_rows, retained_shell_inventory,
    shell_inventory_denominator, shell_inventory_identity, shell_inventory_payloads, shell_payload_family,
)


def inventory_baseline(name: str, value: object) -> None:
    """Require an explicit first freeze, then unchanged metadata; e.g. inventory_baseline('summary.json', {})."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete evidence'


def inventory_binding_summary() -> dict[str, object]:
    """Keep incomplete identities and rejected alternatives visible; e.g. inventory_binding_summary()."""
    payloads = shell_inventory_payloads()
    attempts = [attempt for payload in payloads for attempt in payload['attempts']]
    complete = [payload for payload in payloads if any(attempt['success'] for attempt in payload['attempts'])]
    return dict(streams=len(payloads), logical_bytes=sum(payload['size'] for payload in payloads),
                complete_streams=len(complete), complete_bytes=sum(payload['size'] for payload in complete),
                arguments=len(attempts), accepted=sum(cast(bool, attempt['success']) for attempt in attempts),
                failed=sum(not attempt['success'] for attempt in attempts),
                families={str(index): dict(streams=sum(payload['family'] == index for payload in payloads),
                           logical_bytes=sum(payload['size'] for payload in payloads if payload['family'] == index))
                          for index in range(7)}, target_executed=False, fresh_combined_replay=False)


class ShellInventoryBindingTests(unittest.TestCase):
    def test_complete_reconciliation_and_argument_freeze(self) -> None:
        inventory_baseline('initial-reference.json', retained_shell_inventory())
        payloads = shell_inventory_payloads()
        inventory_baseline('candidate-denominator.json', payloads)
        inventory_baseline('identity-manifest-reference.json', [shell_inventory_identity(payload) for payload in payloads])
        inventory_baseline('binding-reference.json', inventory_binding_summary())

    def test_every_corpus_identity_is_bound(self) -> None:
        self.assertEqual(len(shell_inventory_denominator()), 936)
        self.assertEqual(len(original_shell_rows()), 936)

    def test_identity_substitutions_are_rejected(self) -> None:
        rows = [shell_inventory_identity(payload) for payload in shell_inventory_payloads()]
        altered = [rows[:-1], rows + [rows[0]], rows + [dict(rows[0], path='unbound-document')]]
        altered.extend([dict(rows[0], **{key: value}), *rows[1:]] for key, value in
                       (('path', 'renamed'), ('stream', 'renamed'), ('bytes', -1), ('sha256', '0' * 64)))
        for candidate in altered:
            with self.subTest(candidate_count=len(candidate)), self.assertRaises(AssertionError):
                complete_identity_match(rows, candidate)

    def test_supported_storage_shapes_have_explicit_families(self) -> None:
        root = 'Cosworks/StudyManager/StudyNumber-0/ShellManager/'
        suffixes = ('Property', 'ShellManagerNew/ShellManagerGroup', 'Shells/Shell 1/Property',
                    'MidSurfShells/MidSurfShell 1/Property', 'Shells/Shell 1/Material',
                    'Shells/Shell 1/CompositeFlipTurn', 'Shells/Shell 1/CompositeExtra')
        self.assertEqual([shell_payload_family(root + suffix) for suffix in suffixes], list(range(7)))
        self.assertEqual(shell_payload_family(root + 'MidSurfShells/MidSurfShell 1/Material'), 4)

    def test_unknown_or_malformed_storage_shapes_fail(self) -> None:
        root = 'Cosworks/StudyManager/StudyNumber-0/ShellManager/'
        suffixes = ('Unknown', 'ShellManagerGroup', 'Shells//Property', 'Shells/Shell/Unknown',
                    'Shells/Shell/Property/Extra', 'MidSurfShells/Shell/CompositeExtra',
                    'Solid Bodies/Body/Property', 'Shells/Shell/Material/ShellManager/Property')
        for stream in ('missing-storage', *(root + suffix for suffix in suffixes)):
            with self.subTest(stream=stream), self.assertRaises(AssertionError):
                shell_payload_family(stream)

    def test_incomplete_extra_keeps_all_original_failures(self) -> None:
        incomplete = [payload for payload in shell_inventory_payloads()
                      if not any(attempt['success'] for attempt in payload['attempts'])]
        self.assertEqual(len(incomplete), 1)
        self.assertEqual((incomplete[0]['family'], incomplete[0]['size']), (6, 4))
        self.assertTrue(all(attempt['version'] == 840030 for attempt in incomplete[0]['attempts']))
        self.assertEqual([attempt['wire_profile'] for attempt in incomplete[0]['attempts']], ['reader', 'writer'])

    def test_writer_success_does_not_erase_reader_failure(self) -> None:
        alternatives = [payload['attempts'] for payload in shell_inventory_payloads() if payload['family'] == 6
                        and any(attempt['success'] for attempt in payload['attempts'])]
        self.assertEqual(len(alternatives), 8)
        for attempts in alternatives:
            self.assertEqual([(attempt['wire_profile'], attempt['success']) for attempt in attempts],
                             [('reader', False), ('writer', True)])

    def test_payload_order_is_unique_and_total(self) -> None:
        identities = [(payload['document'], payload['stream']) for payload in shell_inventory_payloads()]
        self.assertEqual(identities, sorted(set(identities)))


if __name__ == '__main__':
    unittest.main(argv=[sys.argv[0]], verbosity=2)
