"""Complete caller graphs for the native-proven curve-point wrapper ancestry (#54)."""
from pathlib import Path
from functools import partial
import sys
WORKSPACE=Path(__file__).resolve().parents[3]
for name in ('solid-references','sketch-wrapper-roles','projected-line-subtypes','end-spec-wrapper-roles','intersection-wrapper-roles','cone-references'):
    sys.path.insert(0,str(WORKSPACE/'docs/features'/name))
from fillet_fixture_bytes import WireMap
from sweep_fixture_bytes import shared_object,endpoint_body
from cone_wrapper_bytes import wrapper_caller_object
from freeze_intersection_wrappers import configure as configure_callers
from freeze_blank_part import source_module
from freeze_trim_edges import ObjectFixtureFamily
from object_fixture_observer import ReferenceObjectCursor,object_cursor


def curve_pointer(mapping: WireMap, identities: dict[str,int], version: int, key: str) -> bytes:
    """Use the complete endpoint body under its proved curve-point class."""
    return shared_object(mapping,identities,key,'moCurvePointRef_w',lambda:endpoint_body(mapping,version,identities,False))


def caller_object(kind: int, version: int, style: int, edited: bool=False, first_slot: int=0) -> tuple[bytes,int]:
    """Reuse complete owners and their null/shared branches, changing only the real target."""
    return wrapper_caller_object(kind,version,style,first_slot,curve_pointer)


def configure(cursor: ReferenceObjectCursor, reference: Path) -> None:
    """Select the actual complete caller and curve-point readers locally."""
    configure_callers(cursor,reference)
    source_module('deformations',reference).install(cursor)


def verify(reference: Path, raw: bytes, version: int, kind: int, style: int) -> None:
    """Require the complete containing record to reach exact EOF."""
    cursor=object_cursor(reference,raw,version,partial(configure,reference=reference));cursor.object()
    assert cursor.p==len(raw)


def caller_family(reference: Path) -> ObjectFixtureFamily:
    """Bind all five finite-family callers using fresh and cached actual targets."""
    return ObjectFixtureFamily(build=caller_object,verify=verify,configure=partial(configure,reference=reference),magic=b'CPW1',kinds=5,versions=(19000,),edited_kinds=())
