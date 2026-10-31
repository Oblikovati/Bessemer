"""Independent literal nested chain and external-file archives (#134)."""
import struct
from owned_components import signed_words


def tagged_component(name: str, body: bytes) -> bytes:
    """Emit a complete new-class record; e.g. tagged_component('moCStringHandle_c', b'\0')."""
    encoded = name.encode('ascii')
    return struct.pack('<HHH', 0xffff, 7, len(encoded)) + encoded + body


def shared_file_bytes(version: int) -> bytes:
    """Retain each physical shared-file threshold; e.g. shared_file_bytes(14000)."""
    raw = tagged_component('moCStringHandle_c', b'\x04path')
    raw += tagged_component('moCStringHandle_c', b'\x04name')
    raw += struct.pack('<HBI', 0xabcd, 0xff, 0xfedcba98)
    if 0x1427 < version < 0x146c:
        raw += b'\x9a'
    for threshold, text in ((0x1fb3, b'\x03raw'), (0x23d9, b'\x03imp'), (0x2f82, b'\x03pdm')):
        if version > threshold:
            raw += text
    if version >= 0x3330:
        raw += b'\xfe'
    return raw


def external_feature_bytes(version: int) -> bytes:
    """Encode one complete feature with a nonnull name handle; e.g. external_feature_bytes(6000)."""
    raw = shared_file_bytes(version) + b'\x03Cfg' + signed_words(-3)
    raw += b'\xff' + signed_words(-44)
    raw += tagged_component('moCStringHandle_c', b'\x04feat') + b'\0\0'
    if version >= 0x11eb:
        raw += signed_words(-58)
    return raw


def external_object_bytes(version: int) -> bytes:
    """Encode complete external-object fields without interpreting timestamps; e.g. external_object_bytes(19000)."""
    raw = shared_file_bytes(version) + struct.pack('<BI', 0xff, 0xfedcba98)
    raw += signed_words(-48, -76, -120) + b'\x03Cfg' + signed_words(-3, -84)
    raw += struct.pack('<I', 0xffffffff)
    if version > 0x3f12:
        raw += signed_words(-80)
    return raw


def nested_chain_bodies(version: int) -> dict[str, bytes]:
    """Provide complete bodies with real nested class definitions; e.g. nested_chain_bodies(6000)."""
    feature = external_feature_bytes(version)
    listing = signed_words(2) + tagged_component('moExtFeature_c', feature) * 2
    reference = struct.pack('<H', 0xffff) + tagged_component('moExtFeatureList_c', listing)
    reference += signed_words(-32) + b'\x03Cfg' + signed_words(-1)
    chain = b'\0\0' + tagged_component('moReferenceChain_c', reference) + struct.pack('<I', 0xffffffff)
    return {'moExtFeature_c': feature, 'moExtFeatureList_c': listing,
            'moReferenceChain_c': reference, 'moChainComponent_c': chain,
            'moExtObject_c': external_object_bytes(version)}


def owned_chain_cases() -> list[tuple[int, str, bytes]]:
    """Cover all shared-file and feature thresholds with complete nested roots; e.g. owned_chain_cases()."""
    thresholds = (0x11eb, 0x1427, 0x146c, 0x1fb3, 0x23d9, 0x2f82, 0x3330, 0x3f12)
    versions = sorted({4281, 4307, 6000, 19000, *(value + delta for value in thresholds for delta in (-1, 0, 1))})
    return [(version, name, struct.pack('<I', version) + tagged_component(name, body))
            for version in versions for name, body in nested_chain_bodies(version).items()]
