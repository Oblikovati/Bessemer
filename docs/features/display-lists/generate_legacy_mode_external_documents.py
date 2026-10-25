"""Complete earlier viewer-mode layouts with populated buckets and explicit face caches."""
from functools import partial
from legacy_mode_wire import mode_root
from complete_external_cases import write_documents


if __name__=="__main__":
    write_documents(partial(mode_root,cached=True),[2711,2725],"legacy-mode-external")
