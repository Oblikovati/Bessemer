"""Actual unmodified parser adapters for all six BeamManager payload families (#146)."""
from typing import cast
from inventory_bindings import BoundPayload
from freeze_manager_property import ManagerAttempt, check_attempt
from material_reference import actual_material_parser, retained_outcome
from beam_projection import beam_parser
from group_projection import actual_parser


def verify_attempt(payload: BoundPayload, raw: bytes, expected: dict[str, object]) -> None:
    """Match each complete original outcome without inferring native version dispatch; e.g. verify_attempt(payload, raw, expected)."""
    version = cast(int, expected['version'])
    family = payload['family']
    if family == 0:
        check_attempt('BeamManager', raw, cast(ManagerAttempt, expected))
        return
    if family == 1:
        assert dict(version=version, **beam_parser(raw, version)) == expected
        return
    if family == 2:
        assert retained_outcome(version, actual_material_parser(raw, version)) == expected
        return
    group_kind = {3: 0, 4: 2, 5: 1}[family]
    parsed = actual_parser(raw, version, group_kind, payload['expected_joint_count'])
    assert dict(version=version, **parsed) == expected
