"""Independent temporary edge records and discarded view triples for old buckets."""
from generate_component_vectors import integers
from legacy_region_wire import region_poly_reader
from legacy_thread_wire import thread_poly
from bucket_vector_wire import poly_list


def edge_item(version: int, dimensions: int, edited: bool, populated: bool) -> bytes:
    raw = integers(-2147483648,2147483647)
    if version<0xa4b:raw += integers(-2147483648+int(edited))
    raw += integers(-8+int(edited),-12)
    if not populated:return raw+integers(-3)
    encode = region_poly_reader(thread_poly,edited)
    positive = encode(version,dimensions,2,3,False,0 if edited else -17)
    negative = encode(version,dimensions,-5,9,False,0)
    return raw+poly_list([positive,negative])


def edge_array(version: int, dimensions: int, empty: bool, edited: bool) -> bytes:
    if empty:return integers(-2)
    first = edge_item(version,dimensions,edited,True)
    second = edge_item(version,dimensions,edited,False)
    return integers(2)+first+second


def view_triples(empty: bool, edited: bool) -> bytes:
    if empty:return integers(-7)
    first = integers(-1+int(edited),2,-3)
    return integers(2)+first+integers(-2147483648,2147483647,0)
