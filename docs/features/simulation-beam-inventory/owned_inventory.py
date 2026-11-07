"""Owned six-family corpus with retained ambiguous/failed header contexts for CI (#146)."""
from owned_components import signed_words
from owned_beam import owned_beam_cases
from owned_material import material_literal
from owned_groups import group_body, joint_body, list_body
from inventory_bindings import BoundPayload
from beam_projection import beam_parser
from material_reference import actual_material_parser, retained_outcome
from group_projection import actual_parser

ROOT_STREAM = 'Cosworks/StudyManager/StudyNumber-0/BeamManager/'


def owned_payload(family: int, suffix: str, raw: bytes, attempts: list[dict[str, object]], count: int | None = None) -> BoundPayload:
    """Keep a synthetic identity distinct from every original native document; e.g. owned_payload(0, 'Property', raw, attempts)."""
    from native_carriers import digest
    return BoundPayload(document='owned/beam-inventory.sldprt', stream=ROOT_STREAM + suffix,
                        size=len(raw), sha256=digest(raw), family=family,
                        expected_joint_count=count, attempts=attempts)


def header_case() -> tuple[BoundPayload, bytes]:
    """Retain two matching argument profiles and a failed historical alternative; e.g. header_case()."""
    raw = signed_words(-1, 2, -3)
    attempts: list[dict[str, object]] = [dict(version=810023, success=False)]
    attempts.extend(dict(version=version, success=True, values=[-1, 2, -3], destinations=[0x14c, 0x168, 0x148])
                    for version in (850036, 890022))
    return owned_payload(0, 'Property', raw, attempts), raw


def beam_case() -> tuple[BoundPayload, bytes]:
    """Use a complete independent reference/triple body; e.g. beam_case()."""
    version = 890022
    raw = next(raw for profile, candidate, raw in owned_beam_cases() if profile == 3 and candidate == version)
    expected = dict(version=version, **beam_parser(raw, version))
    assert expected['success']
    return owned_payload(1, 'Beams/Beam owned/Property', raw, [expected]), raw


def material_case() -> tuple[BoundPayload, bytes]:
    """Retain populated direct and delegated tables without physical interpretation; e.g. material_case()."""
    version = 891107
    raw = material_literal(version, 1)
    expected = retained_outcome(version, actual_material_parser(raw, version))
    assert expected['success']
    return owned_payload(2, 'Beams/Beam owned/Material', raw, [expected]), raw


def group_cases() -> list[tuple[BoundPayload, bytes]]:
    """Keep separate group, joint and checked-list payloads; e.g. group_cases()."""
    version = 890022
    count, listed = list_body(3)
    definitions = ((3, 0, 'JointGroup', group_body(version, 1), None),
                   (4, 2, 'JointList', listed, count),
                   (5, 1, 'Joint owned/Joint', joint_body(version, 2), None))
    cases: list[tuple[BoundPayload, bytes]] = []
    for family, kind, suffix, raw, expected_count in definitions:
        expected = dict(version=version, **actual_parser(raw, version, kind, expected_count))
        assert expected['success']
        payload = owned_payload(family, 'JointGroups/JointGroup owned/' + suffix, raw, [expected], expected_count)
        cases.append((payload, raw))
    return cases


def owned_inventory_cases() -> list[tuple[BoundPayload, bytes]]:
    """Exercise all native-supported families together without adding a folder fixture; e.g. owned_inventory_cases()."""
    cases = [header_case(), beam_case(), material_case(), *group_cases()]
    return sorted(cases, key=lambda case: (case[0]['document'], case[0]['stream']))
