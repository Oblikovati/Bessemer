"""Freeze complete native-bound C-274 edge representations (#54)."""
from pathlib import Path
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(WORKSPACE/'docs/features/trim-edge-identifiers'))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from trim_edge_bytes import trim_edge_object
from trim_edge_identity import verify_identity

CLASSES=('moCopyEdgeIdRep_c','moTrimExtEdgeIdRep_c','moEdgeRefIdRep_c')


def copy_edge_object(kind: int, version: int, style: int, edited: bool=False,
                     first_slot: int=0) -> tuple[bytes,int]:
    """Reuse the proven one-word layouts with complete parents; e.g. copy_edge_object(2,19000,5)."""
    return trim_edge_object(0,version,style,edited,first_slot,CLASSES[kind],kind<2)


def verify_copy_identity(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Check distinct roots and shared direct representations when present; e.g. verify_copy_identity(source,raw,19000,0,5)."""
    verify_identity(reference,raw,version,kind,style,kind<2)


FAMILY=ObjectFixtureFamily(build=copy_edge_object,verify=verify_copy_identity,magic=b'CEI1',
                          kinds=3,edited_kinds=(0,1,2))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/copy-trim-edge-identifiers',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
