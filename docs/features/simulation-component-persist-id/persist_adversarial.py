"""Actual standalone persist-ID malformed outcomes and independent budgets (#152)."""
import struct
from collections.abc import Callable
from body_adversarial import BodyParser, body_mutation_packet, body_budget_packet
from body_projection import body_shape
from owned_persist import owned_component_persist_ids
from sheet_projection import sheet_embedded_fields


def persist_id_mutations(parser: BodyParser) -> tuple[bytes, int, int]:
    """Retain every complete actual outcome including positive controls; e.g. persist_id_mutations(parser)."""
    return body_mutation_packet(parser, owned_component_persist_ids())


def persist_id_budgets(parser: BodyParser) -> bytes:
    """Measure the exact standalone fields/archives without suffix assumptions; e.g. persist_id_budgets(parser)."""
    return body_budget_packet(parser, owned_component_persist_ids(), body_shape, sheet_embedded_fields)


def persist_id_blob_cap(parser: BodyParser, save: Callable[[str, object], None]) -> None:
    """Check populated blobs at and above the declared hard cap; e.g. persist_id_blob_cap(parser, save)."""
    results: list[dict[str, object]] = []
    for count in (1000000, 1000001):
        raw = struct.pack('<iii', -2147483648, 2147483647, count) + b'\xaa' * count
        accepted = parser(raw, 0)['success']
        assert accepted == (count == 1000000)
        results.append(dict(count=count, accepted=accepted))
    save('blob-cap-reference.json', results)
