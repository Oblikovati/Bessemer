"""Complete implicit-width layouts with explicitly identified external face caches."""
from legacy_width_wire import width_root
from complete_external_cases import write_documents


if __name__=="__main__":
    write_documents(width_root,[2771,2795],"legacy-width-external")
