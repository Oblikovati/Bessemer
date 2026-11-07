"""Independent complete embedded reference-family images and historical suffixes (#134)."""
import struct
from owned_components import holder_words, holder_strings, holder_suffix, signed_words
from owned_embedded import literal_wrapper, patterned_doubles
from owned_chains import tagged_component

LOCAL_WORDS: dict[str, int] = {
    'moFromSktEntSurfIdRep_c': 1, 'moFaceBlendSurfIdRep_c': 1, 'moTopEdgeIdRep_c': 1,
    'moSegmEdgeIdRep_c': 1, 'moSimpleSurfIdRep_c': 1, 'moBottomEdgeIdRep_c': 1,
    'moStartEdgeIdRep_c': 1, 'moEndFaceSurfIdRep_c': 2, 'moSegmRightTrimEdgeIdRep_c': 2,
    'moFilletSurfIdRep_c': 1, 'moThinClosingSurfIdRep_c': 2, 'moFromSktEnt3IntSurfIdRep_c': 3,
    'moRadiateEdgeIdRep_c': 2, 'moEndEdgeIdRep_c': 1, 'moEndFace3IntSurfIdRep_c': 3,
    'moSweepSideSurfIdRep_c': 2, 'moWzdHoleSurfIdRep_c': 2, 'moSideFace3IntSurfIdRep_c': 3,
    'moSimpleDoubleEdgeIdRep_c': 2, 'moSimpleDoubleSurfIdRep_c': 2, 'moThinOffsetSurfIdRep_c': 1,
}
SOURCE_WORDS: dict[str, int] = {
    'moPLineSurfIdRep_c': 1, 'moIndirectSurfIdRep_c': 0, 'moPLineProjIdRep_c': 1,
    'moDerivedEdgeIdRep_c': 1, 'moShellSurfIdRep_c': 0, 'moDerivedSurfIdRep_c': 1,
    'moMirPatternSurfIdRep_c': 0, 'moCirPatternSurfIdRep_c': 1, 'moLPatternSurfIdRep_c': 2,
    'moDerivedSurfIdRep2_c': 2, 'moPLineInterSurfIdRep_c': 2, 'moPttnBodyEdgeIdRep_c': 2,
    'moPLineSilhIdRep_c': 2, 'moDerivedEdgeIdRep2_c': 2,
}
MODERN_REPRESENTATIONS = {
    'moEndEdgeIdRep_c', 'moEndFace3IntSurfIdRep_c', 'moSweepSideSurfIdRep_c', 'moWzdHoleSurfIdRep_c',
    'moSideFace3IntSurfIdRep_c', 'moCirPatternSurfIdRep_c', 'moLPatternSurfIdRep_c',
    'moDerivedSurfIdRep2_c', 'moSimpleDoubleEdgeIdRep_c', 'moSimpleDoubleSurfIdRep_c',
    'moThinOffsetSurfIdRep_c', 'moPLineInterSurfIdRep_c', 'moPttnBodyEdgeIdRep_c',
    'moPLineSilhIdRep_c', 'moDerivedEdgeIdRep2_c',
}


def representation_bodies(version: int) -> dict[str, bytes]:
    """Use explicit literal suffix counts independently of the Rust table; e.g. representation_bodies(6000)."""
    result: dict[str, bytes] = {}
    for name, count in {**LOCAL_WORDS, **SOURCE_WORDS}.items():
        if version == 4281 and name in MODERN_REPRESENTATIONS:
            continue
        prefix = b'\0' * (6 if name in SOURCE_WORDS else 4)
        body = prefix + signed_words(*(-value - 1 for value in range(count)))
        if name == 'moCirPatternSurfIdRep_c' and 0x23eb < version < 0x23ef:
            body += signed_words(-123)
        result[name] = body
    result['moFR_c'] = b'\0\0' + signed_words(-1) + struct.pack('<I', 0xffffffff)
    return result


def boundary_body(version: int) -> bytes:
    """Encode two direct entity IDs and independently gated boundary tails; e.g. boundary_body(14000)."""
    raw = struct.pack('<5I', 0xffffffff, 0, 2, 3, 4) + patterned_doubles(2)
    raw += b'\0' * 6 + struct.pack('<I', 0xffffffff)
    if version > 0x23bb:
        raw += signed_words(0)
    if version > 0x332c:
        raw += signed_words(-236, 0)
    return raw


def boundary_bodies(version: int) -> dict[str, bytes]:
    """Include populated raw face records and guarded empty vertex points; e.g. boundary_bodies(6000)."""
    base = boundary_body(version)
    records = signed_words(-1, 2, -3, 4, -5, 6, -7, 8)
    return {'moFaceRef_c': base + signed_words(2) + records,
            'moEdgeRef_c': base, 'moVertexRef_c': base + signed_words(0)}


def component_reference_body(version: int) -> bytes:
    """Reuse independent historical literal sections, including present text; e.g. component_reference_body(14000)."""
    return holder_words(version) + holder_strings(version, True) + holder_suffix(version, True)


def component_bodies(version: int) -> dict[str, bytes]:
    """Keep face, edge, feature and solid continuations distinct; e.g. component_bodies(6000)."""
    prefix = component_reference_body(version)
    boundary = prefix + b'\xff\x03ali' + b'\0' * 4
    feature = prefix + signed_words(-1) + struct.pack('<I', 0xffffffff)
    result = {'moCompFace_c': boundary, 'moCompVertex_c': boundary, 'moCompFeature_c': feature}
    edge = patterned_doubles(6) + struct.pack('<I', 0xffffffff) + signed_words(2) + b'\0' * 4
    result['moCompEdge_c'] = boundary + struct.pack('<I', 8) + edge + b'\0' * 4
    if version >= 4307:
        result['moCompSolidBody_c'] = prefix + b'\0\0' + signed_words(3, -1, 2, -3, -4, 5, -6)
        result['moCompSketchEntHandle_c'] = feature + b'\0\0' + struct.pack('<I', 0xffffffff)
        result['sgLineHandle'] = struct.pack('<H', 0x777f) + signed_words(-1, -2)
        if version >= 0x17d7:
            result['sgLineHandle'] += signed_words(-3)
    return result


def wrapper_bodies(version: int) -> dict[str, bytes]:
    """Pin endpoint's single parameter double independently of line's two; e.g. wrapper_bodies(6000)."""
    base = literal_wrapper(version)
    result = {
        'moSingleFaceRef_w': base + patterned_doubles(4) + signed_words(0, -1, -2),
        'moGeneralCurveRef_w': base + patterned_doubles(2) + b'\0' + signed_words(-1, -2) + b'\0' * 6,
        'moLineRef_w': base + patterned_doubles(8) + b'\xff',
        'moEndPointRef_w': base + patterned_doubles(4) + signed_words(-1),
        'moFeatureRefWrapper_c': base + struct.pack('<I', 0xffffffff),
    }
    if version >= 4307:
        word = struct.pack('<I', 0xffffffff) if version > 0x1bbd else b''
        result['moSolidRef_w'] = base + word + signed_words(0)
    return result


def owned_reference_cases() -> list[tuple[int, str, bytes]]:
    """Cover the complete declared reference registry across its suffix thresholds; e.g. owned_reference_cases()."""
    thresholds = (0x17d7, 0x1bbd, 0x23bb, 0x23eb, 0x23ef, 0x332c)
    versions = sorted({4281, 4307, 6000, 19000, *(value + delta for value in thresholds for delta in (-1, 0, 1))})
    result: list[tuple[int, str, bytes]] = []
    for version in versions:
        bodies = {**representation_bodies(version), **boundary_bodies(version), **component_bodies(version), **wrapper_bodies(version)}
        bodies['moReferenceChain_c'] = shared_alias_body()
        bodies['moUnitComponent_c'] = b'\0' * 3
        bodies['moCStringHandle_c'] = b'\x05raw\0x'
        for name, body in bodies.items():
            result.append((version, name, struct.pack('<I', version) + tagged_component(name, body)))
    return result


def shared_alias_body() -> bytes:
    """Pin both an object alias and cached class against literal MFC slots; e.g. shared_alias_body()."""
    strings = tagged_component('moCStringHandle_c', b'\x04same') + struct.pack('<HH', 6, 0x8005) + b'\x03two'
    listing = tagged_component('moExtFeatureList_c', signed_words(3) + strings)
    return struct.pack('<H', 7) + listing + signed_words(-1) + b'\0' + signed_words(-2)


def nested_representation_bytes(depth: int) -> bytes:
    """Build nested source pointers independently of either parser; e.g. nested_representation_bytes(128)."""
    image = b'\0\0'
    for _ in range(depth):
        image = tagged_component('moIndirectSurfIdRep_c', b'\0' * 4 + image)
    return struct.pack('<I', 19000) + image
