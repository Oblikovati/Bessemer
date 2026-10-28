"""Owned pre-hook sketch-tail builder baselines for Bug #46."""
from pathlib import Path
import hashlib
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(WORKSPACE/'docs/features/model-archives'))
import sketch_tail_fixture_bytes as wire
VERSIONS=(0x27cc,0x2b71,0x2b72,0x2b73,0x2b98,0x2b99,0x2bdf,0x2be0,0x2faf,0x2fb0,0x371d,0x371e,0x3b01,0x3b02,19000,0xffffffff)


def current_baselines() -> dict[str,dict[str,object]]:
    """Pin every original default and nonpositive-count branch; e.g. current_baselines()."""
    rows: dict[str,dict[str,object]]={}
    for version in VERSIONS:
        for rich in (False,True):
            for first in (0,96):
                for name in ('intersection','styled_intersections','after_patterns','ordinary'):
                    mapping=wire.WireMap();mapping.slot=first
                    raw=getattr(wire,name)(mapping,version,rich)
                    record(rows,f'{version}/{rich}/{first}/{name}',raw,mapping.slot+1)
                for count in (-2147483648,-1,0):
                    mapping=wire.WireMap();mapping.slot=first
                    raw=wire.intersection(mapping,version,rich,count)
                    record(rows,f'{version}/{rich}/{first}/count/{count}',raw,mapping.slot+1)
    return rows


def record(rows: dict[str,dict[str,object]], label: str, raw: bytes, slots: int) -> None:
    """Retain both bytes and map-allocation behavior; e.g. record(rows,'old',raw,3)."""
    rows[label]={'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest(),'slots':slots}
