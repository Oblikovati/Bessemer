"""Embed independently built named-view bodies in existing complete owned roots."""
from functools import lru_cache
from pathlib import Path
import struct
from owned_root_inventory import OwnedRoot,inventory_roots
from generate_named_vectors import collection,plane
from generate_version_range import apply_patches
from legacy_component_reference import component_reference
import dl_doc

VERSIONS = [8193,8194,8195,8196,8199,8200,8205,8206,19000,0xffffffff]


@lru_cache(maxsize=1)
def owned_sources() -> list[OwnedRoot]:
    return inventory_roots(Path(__file__).parent,Path("tests/fixtures/display-lists"))


def named_source(version: int, kind: str) -> OwnedRoot:
    group = f"window-boundary-{kind}" if version<9000 else "component-properties" if version==19000 else "unsigned-versions"
    candidates = [root for root in owned_sources() if root.version==version and root.kind==kind
        and root.file_version==19000 and not root.external_ids and root.label.startswith(group+"[")]
    if not candidates:raise ValueError(f"named source {version}, {kind}; expected a complete owned root in {group}")
    return candidates[0]


def named_payload(version: int, variant: int) -> bytes:
    if variant<4:return collection(version,variant)
    original = plane(version)
    replacement = original[:56]+struct.pack("<i",23)+original[60:]
    complete = collection(version,3)
    assert complete.count(original)==4
    return complete.replace(original,replacement,1)


def named_body_reader(original: object, replacement: bytes, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        first,before = archive.p,len(archive.map)
        result = original(archive)
        assert before==len(archive.map)
        patches.append((first,archive.p,replacement))
        return result
    return read


def named_branch_root(version: int, kind: str, scenario: int) -> tuple[OwnedRoot,bytes]:
    variant = scenario-192
    if variant not in range(5):raise ValueError(f"named scenario {scenario}; expected 192..196")
    source,patches = named_source(version,kind),[]
    original = dl_doc.read_NamedViewTransform
    dl_doc.read_NamedViewTransform = named_body_reader(original,named_payload(version,variant),patches)
    try:component_reference(source.payload,version,kind,source.external_ids)
    finally:dl_doc.read_NamedViewTransform = original
    assert len(patches)==1
    return source,apply_patches(source.payload,patches)
