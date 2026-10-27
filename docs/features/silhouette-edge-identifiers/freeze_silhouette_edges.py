"""Freeze complete native-bound PLine silhouette identifiers (#54)."""
from pathlib import Path
import json
import sys

WORKSPACE=Path(__file__).resolve().parents[3]
for folder in ('trim-edge-identifiers','frontier-edge-identifiers'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/folder))
from freeze_trim_edges import ObjectFixtureFamily, REFERENCE, freeze
from frontier_edge_bytes import frontier_edge_object
from trim_edge_identity import verify_identity


def silhouette_object(kind: int, version: int, style: int, edited: bool=False,
                      first_slot: int=0) -> tuple[bytes,int]:
    """Reuse the native-proven direct-source layout with its exact class identity; e.g. silhouette_object(0,19000,5)."""
    return frontier_edge_object(1,version,style,edited,first_slot,'moPLineSilhIdRep_c')


FAMILY=ObjectFixtureFamily(build=silhouette_object,verify=verify_identity,magic=b'SIL1',
                          kinds=1,edited_kinds=(0,))


if __name__=='__main__':
    facts=freeze(REFERENCE,WORKSPACE/'tests/fixtures/silhouette-edge-identifiers',FAMILY)
    Path(__file__).with_name('reference.json').write_text(json.dumps(facts,indent=2)+'\n',encoding='utf-8',newline='\n')
