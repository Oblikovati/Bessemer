"""Complete omitted-visibility layouts with populated shaded buckets and face caches."""
from functools import partial
from legacy_visibility_wire import visibility_root
from complete_external_cases import write_documents


if __name__=="__main__":
    write_documents(partial(visibility_root,cached=True),[2650,2688],"legacy-visibility-external")
