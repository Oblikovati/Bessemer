"""Build complete roots across the remaining early-version layouts down to zero."""
from pathlib import Path
from functools import partial
from legacy_zero_wire import early_zero_root,VERSIONS
from legacy_component_reference import component_reference,activity_scalars
from generate_unsigned_versions import compact_manifest
from generate_legacy_visuals import visual_scalars
from generate_earliest_component_buckets import raw_points_reader
from complete_root_case import frame_case
from generate_earlier_range import write_group
from generate_earliest_components import write_documents
import bucket_corpus

def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw,identities = early_zero_root(version,kind,scenario)
    root = component_reference(raw,version,kind,identities)
    original,labels = bucket_corpus.walk,bucket_corpus.POLY_LISTS
    bucket_corpus.walk,bucket_corpus.POLY_LISTS = raw_points_reader(original),labels|{"polys"}
    try:return frame_case(version,kind,scenario,raw,identities,root,visual_scalars(root)+activity_scalars(root))
    finally:bucket_corpus.walk,bucket_corpus.POLY_LISTS = original,labels


if __name__=="__main__":
    cases = [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(3)]
    cases += [generate(v,"assembly",s) for v in VERSIONS for s in (3,4)]
    cases += [generate(0,k,16+light) for k in ["part","assembly"] for light in range(5)]
    cases += [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in (32,33,34,48,49,50,64,65,80,81)]
    cases += [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in (96,97,112,113)]
    cases += [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(144,154)]
    cases += [generate(v,k,s) for v in VERSIONS if v<773 for k in ["part","assembly"] for s in (160,161,162,163,164,165,176,177)]
    cases += [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in (180,181)]
    cases += [generate(v,k,s) for v in (0,561,562,596) for k in ["part","assembly"] for s in (188,189)]
    print(write_group("legacy-zero-roots",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("legacy-zero-roots.json"))
    write_documents(partial(generate,scenario=112),0,"legacy-zero")
    write_documents(partial(generate,scenario=113),0,"legacy-zero-edited")
    write_documents(partial(generate,scenario=112),596,"legacy-zero-rich")
    write_documents(partial(generate,scenario=113),596,"legacy-zero-rich-edited")
