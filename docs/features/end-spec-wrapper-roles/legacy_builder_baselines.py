"""Owned byte baselines for the existing end/body fixture builders (#46)."""
from pathlib import Path
import hashlib
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(WORKSPACE/'docs/features/model-archives'))
from body_feature_fixture_bytes import CLASSES, WireMap, tagged, end_spec, revolution_end
VERSIONS=(0x27cc,0x2b72,0x2b73,0x2faf,0x2fb0,0x3b00,0x3b01,0x3b02,19000,0xffffffff)


def current_baselines() -> dict[str,dict[str,object]]:
    """Pin original default arguments independently of the new wrapper hook; e.g. current_baselines()."""
    rows={}
    for version in VERSIONS:
        for rich in (False,True):
            for name in CLASSES:
                record(rows,f'{version}/{rich}/{name}',tagged(version,name,rich))
            record(rows,f'{version}/{rich}/end',end_spec(WireMap(),version,rich))
            record(rows,f'{version}/{rich}/rev-end',revolution_end(WireMap(),version,rich))
    return rows


def record(rows: dict[str,dict[str,object]], label: str, raw: bytes) -> None:
    """Retain original byte length and hash; e.g. record(rows,'old',raw)."""
    rows[label]={'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()}
