"""Complete earlier add-in and body-property layouts with independently versioned caches."""
from legacy_addin_wire import addin_root
from complete_external_cases import write_documents


if __name__=="__main__":
    write_documents(addin_root,[2796,2890],"legacy-addin-external")
