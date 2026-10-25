"""Complete temporary edge arrays with populated shaded buckets and explicit face caches."""
from functools import partial
from legacy_edge_roots import edge_root
from complete_external_cases import write_documents


if __name__=="__main__":
    write_documents(partial(edge_root,cached=True),[2606,2634,2635,2649,2650],"legacy-edge-external")
