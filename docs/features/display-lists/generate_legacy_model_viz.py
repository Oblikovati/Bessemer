"""Build complete early model, pane and old-light roots with independent field oracles."""
from pathlib import Path
from functools import partial
from legacy_model_viz_wire import early_model_viz_root,write_early_model_viz_facets,VERSIONS
from legacy_text_reference import early_text_reference,node_scalars
from generate_unsigned_versions import compact_manifest
from generate_legacy_visuals import visual_scalars
from generate_earliest_component_buckets import raw_points_reader
from complete_root_case import frame_case
from generate_earlier_range import write_group
from generate_earliest_components import write_documents
import bucket_corpus

def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    raw,identities = early_model_viz_root(version,kind,scenario)
    root = early_text_reference(raw,version,kind,identities)
    original,labels = bucket_corpus.walk,bucket_corpus.POLY_LISTS
    bucket_corpus.walk,bucket_corpus.POLY_LISTS = raw_points_reader(original),labels|{"polys"}
    try:return frame_case(version,kind,scenario,raw,identities,root,visual_scalars(root)+node_scalars(root))
    finally:bucket_corpus.walk,bucket_corpus.POLY_LISTS = original,labels


if __name__=="__main__":
    cases = [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(3)]
    cases += [generate(v,"assembly",s) for v in VERSIONS for s in (3,4)]
    cases += [generate(989,k,16+light) for k in ["part","assembly"] for light in range(5)]
    cases += [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in (32,33,34,48,49,50,64,65,80,81)]
    cases += [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in (96,97,112,113)]
    cases += [generate(v,k,s) for v in VERSIONS for k in ["part","assembly"] for s in range(144,154)]
    print(write_group("legacy-model-viz-roots",cases,("displaylists.py",)))
    compact_manifest(Path(__file__).with_name("legacy-model-viz-roots.json"))
    write_documents(partial(generate,scenario=112),989,"legacy-model-viz")
    write_documents(partial(generate,scenario=113),989,"legacy-model-viz-edited")
    write_early_model_viz_facets()
    write_documents(partial(generate,scenario=112),998,"legacy-model-viz-rich")
    write_documents(partial(generate,scenario=113),998,"legacy-model-viz-rich-edited")
