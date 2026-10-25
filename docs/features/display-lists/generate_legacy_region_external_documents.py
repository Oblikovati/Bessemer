"""Complete region-word layouts with populated shaded buckets and explicit face caches."""
from functools import partial
from legacy_region_wire import region_root
from complete_external_cases import write_documents


if __name__=="__main__":
    write_documents(partial(region_root,cached=True),[2725,2749],"legacy-region-external")
