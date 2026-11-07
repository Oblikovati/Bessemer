"""Independent signed-word MeshControl header literals for both explicit wire profiles (#161)."""
from typing import TypedDict
import struct

HEADER_VERSIONS = (0, 800026, 800027, 800028, 840012, 840013, 840014, 2147483647)
SIGNED_PATTERNS = (-2147483648, -1, 0, 1, 2147483647)


class OwnedMeshHeader(TypedDict):
    version: int
    profile: int
    words: list[int]
    raw: bytes


def owned_mesh_headers() -> list[OwnedMeshHeader]:
    """Cross both threshold neighbors with every signed extreme and profile; e.g. owned_mesh_headers()."""
    cases: list[OwnedMeshHeader] = []
    for version in HEADER_VERSIONS:
        for profile, pattern in ((profile, pattern) for profile in (0, 1) for pattern in range(5)):
            count = 3 if profile == 1 else 1 + int(version > 800027) + int(version >= 840013)
            words = [SIGNED_PATTERNS[(pattern + ordinal) % 5] for ordinal in range(count)]
            cases.append(OwnedMeshHeader(version=version, profile=profile, words=words,
                                         raw=b''.join(struct.pack('<i', word) for word in words)))
    return cases


def mesh_header_spans(words: list[int]) -> list[dict[str, object]]:
    """Bind independent physical destinations and raw signed patterns; e.g. mesh_header_spans([-1, 0, 1])."""
    assert 1 <= len(words) <= 3, f'{len(words)}; expected one to three header words'
    return [dict(label=label, offset=ordinal * 4, end=ordinal * 4 + 4, raw_hex=struct.pack('<i', word).hex())
            for ordinal, (label, word) in enumerate(zip(('word08', 'word50', 'word60'), words, strict=False))]
