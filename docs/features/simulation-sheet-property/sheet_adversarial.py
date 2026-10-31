"""Independent inherited-sheet mutations, budgets and sufficient blob-cap controls (#151)."""
import struct
from collections.abc import Callable
from body_adversarial import BodyParser, body_mutation_packet, body_budget_packet
from owned_sheet import owned_sheets, sheet_literal
from sheet_projection import sheet_shape, sheet_embedded_fields


def sheet_mutations(parser: BodyParser) -> tuple[bytes, int, int]:
    """Keep every actual outcome for this exact inherited grammar; e.g. sheet_mutations(parser)."""
    return body_mutation_packet(parser, owned_sheets())


def sheet_budgets(parser: BodyParser) -> bytes:
    """Measure all outer/embedded requirements with the exact sheet callback; e.g. sheet_budgets(parser)."""
    return body_budget_packet(parser, owned_sheets(), sheet_shape, sheet_embedded_fields)


def sheet_blob_cap(parser: BodyParser, save: Callable[[str, object], None]) -> None:
    """Supply a complete payload at and above the fixed persist-ID cap; e.g. sheet_blob_cap(parser, save)."""
    original = sheet_literal(0, 0)
    results: list[dict[str, object]] = []
    for count in (1000000, 1000001):
        raw = original[:8] + struct.pack('<i', count) + b'\xaa' * count + original[12:]
        accepted = parser(raw, 0)['success']
        assert accepted == (count == 1000000)
        results.append(dict(count=count, accepted=accepted))
    save('blob-cap-reference.json', results)
