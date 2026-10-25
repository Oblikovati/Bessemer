"""Complete populated bucket layouts and non-null models with explicit face caches."""
from legacy_thread_wire import thread_cached_root
from complete_external_cases import write_documents


if __name__=="__main__":
    write_documents(thread_cached_root,[2749,2771],"legacy-thread-external")
