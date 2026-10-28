"""Owned pre-change valued-dimension builder bytes for feature #54."""
from pathlib import Path
import hashlib
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(WORKSPACE/'docs/features/model-archives'))
from sketch_dimension_fixture_bytes import G1, G2, OFFSETS, valued_body
VERSIONS=(0x27cc,0x2b71,0x2b72,0x2b98,0x2b99,0x2bdf,0x2be0,0x2f79,0x2f7a,0x2f9f,0x2fa0,0x42ee,0x42ef,0x46cb,0x46cc,19000,0xffffffff)


def current_baselines() -> dict[str,dict[str,object]]:
    """Retain every existing class/default across its inherited gates; e.g. current_baselines()."""
    result: dict[str,dict[str,object]]={}
    for version in VERSIONS:
        for rich in (False,True):
            for name in G1+G2+OFFSETS+['sgValG3Dim']:
                raw=valued_body(name,version,rich)
                result[f'{version}/{rich}/{name}']={'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()}
    return result
