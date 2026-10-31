"""Complete actual Material parser and original retained-outcome projections (#140)."""
from typing import cast


def actual_material_parser(raw: bytes, version: int, prefix: bool = False) -> dict[str, object]:
    """Execute the unmodified copied body reader with an explicit boundary; e.g. actual_material_parser(raw, 830036)."""
    from cosworks_material_layout import parse
    return cast(dict[str, object], parse(raw, version, require_eof=not prefix))


def retained_outcome(version: int, parsed: dict[str, object]) -> dict[str, object]:
    """Preserve exactly the original C-724 success/failure summary fields; e.g. retained_outcome(830036, parsed)."""
    if not parsed['success']:
        return dict(version=version, success=False, end=parsed['end'], error=parsed['error'],
                    prefix_reconstruction_sha256=parsed['prefix_reconstruction_sha256'])
    records = cast(list[dict[str, object]], parsed['records'])
    curves = cast(list[dict[str, object]], parsed['curves'])
    return dict(version=version, success=True, end=parsed['end'],
                reconstruction_sha256=parsed['reconstruction_sha256'],
                span_count=len(cast(list[object], parsed['spans'])), property_count=len(records),
                delegated_count=len(curves), populated_property_tables=sum(bool(row['curve_present'] and row['pair_count']) for row in records),
                populated_delegated_tables=sum(bool(row['pair_count']) for row in curves))
