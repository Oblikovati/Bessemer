"""Reuse verified saved-manager fixture construction for both hem layouts (#54)."""
from pathlib import Path
from active_edge_fixtures import freeze_active
from hem_fixture_bytes import hem_graph


def freeze_active_hems(reference: Path, destination: Path) -> list[dict[str,object]]:
    """Use the same real CMgr-derived context for corner and hem roots.
    Example: freeze_active_hems(source,output).
    """
    return freeze_active(reference,destination,kinds=(0,3,4),build=hem_graph)
