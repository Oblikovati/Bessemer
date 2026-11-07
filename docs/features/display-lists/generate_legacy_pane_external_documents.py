"""Complete cache-backed older line/pane/symbol layouts, with independently stored caches."""
from legacy_pane_wire import root_bytes
from complete_external_cases import write_documents


if __name__=="__main__":
    write_documents(root_bytes,[2999,3064],"legacy-pane-external")
