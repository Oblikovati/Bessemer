"""Complete buckets before and after temporary edge records and view triples disappeared."""
from pathlib import Path
from functools import partial
from legacy_edge_roots import edge_root
from legacy_edge_reference import edge_reference
from generate_unsigned_versions import compact_manifest
from generate_legacy_visuals import visual_scalars
from generate_earliest_component_buckets import raw_points_reader
from complete_root_case import frame_case
from generate_earlier_range import write_group
from generate_earliest_components import write_documents
import bucket_corpus

VERSIONS = [2606,2626,2627,2634,2635,2649,2650]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw,identities = edge_root(version,kind,scenario)
    root = edge_reference(raw,version,kind,identities)
    original,labels = bucket_corpus.walk,bucket_corpus.POLY_LISTS
    bucket_corpus.walk,bucket_corpus.POLY_LISTS = raw_points_reader(original),labels|{"polys"}
    try:return frame_case(version,kind,scenario,raw,identities,root,visual_scalars(root))
    finally:bucket_corpus.walk,bucket_corpus.POLY_LISTS = original,labels


if __name__=="__main__":
    cases = [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(3)]
    cases += [generate(v,"assembly",s) for v in VERSIONS for s in (3,4)]
    cases += [generate(2606,k,16+light) for k in ["part","assembly"] for light in range(5)]
    cases += [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in (32,33,34,48,49,50,64,65,80,81)]
    print(write_group("legacy-edge-roots",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("legacy-edge-roots.json"))
    write_documents(partial(generate,scenario=32),2606,"legacy-edge")
