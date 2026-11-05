"""Independent selected native index and physical-role reference checks (#230)."""
from dataclasses import dataclass
from pathlib import Path
import struct
import sys
from hash_reference import default_hash, requested_bucket

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'database-overflow'))
from bucket_fixtures import native_bucket_bytes


@dataclass(frozen=True)
class NativeIndex:
    free_head: int
    split: int
    maximum: int
    map_pages: tuple[int, ...]
    entries: tuple[tuple[int, int], ...]


def page_bytes(raw: bytes, identity: int, extent: int) -> bytes:
    """Check complete supplied physical pages; e.g. page_bytes(raw,1,4096)."""
    assert extent >= 44 and len(raw) % extent == 0
    assert 0 <= identity < len(raw) // extent, f'page {identity}; expected within{len(raw) // extent}'
    return raw[identity * extent:(identity + 1) * extent]


def map_record_page(raw: bytes, identity: int, extent: int) -> tuple[int, tuple[tuple[int, int], ...]]:
    """Require complete declared pairs, unlike native prefix-success; e.g. map_record_page(raw,1,4096)."""
    page = page_bytes(raw, identity, extent)
    count_at = 40 if identity == 1 else 8
    start = count_at + 4
    count = struct.unpack_from('>I', page, count_at)[0]
    assert count <= (extent - start) // 16, f'count {count}; expected complete16-byte pairs in{extent}'
    following = struct.unpack_from('>Q', page, 32 if identity == 1 else 0)[0]
    entries = tuple(struct.unpack_from('>QQ', page, start + index * 16) for index in range(count))
    return following, entries


def read_index(raw: bytes, extent: int) -> NativeIndex:
    """Read explicit default-provider index framing; e.g. read_index(raw,4096)."""
    root = page_bytes(raw, 1, extent)
    assert root[:4] == bytes.fromhex('fa782dcb')
    assert int.from_bytes(root[4:8], 'big') == default_hash(b'chm@symisc')
    free_head, split, maximum = struct.unpack_from('>QQQ', root, 8)
    visited: list[int] = []
    entries: list[tuple[int, int]] = []
    current = 1
    while current:
        assert current not in visited, f'map page {current}; expected acyclic chain'
        visited.append(current)
        current, selected = map_record_page(raw, current, extent)
        entries.extend(selected)
    return NativeIndex(free_head, split, maximum, tuple(visited), tuple(entries))


def claim_role(roles: dict[int, str], identity: int, kind: str, count: int) -> None:
    """Reject cross-role aliases and cycles; e.g. claim_role(roles,2,'bucket',3)."""
    assert 0 <= identity < count, f'page {identity}; expected0..{count - 1}'
    assert identity not in roles, f'page {identity} role{roles.get(identity)}; expected unique{kind}'
    roles[identity] = kind


def check_bucket_chain(raw: bytes, logical: int, first: int, index: NativeIndex,
                       roles: dict[int, str]) -> tuple[int, int]:
    """Bind every stored key hash/request to its selected bucket; e.g. check_bucket_chain(raw,0,2,index,roles)."""
    current = first
    cells = 0
    buckets = 0
    while current:
        claim_role(roles, current, 'bucket', len(raw) // 4096)
        case = native_bucket_bytes(raw, current)
        for cell in case.cells:
            assert default_hash(cell.key) == cell.key_hash
            assert requested_bucket(cell.key_hash, index.split, index.maximum) == logical
        for identity, _ in case.pages:
            claim_role(roles, identity, 'overflow', len(raw) // 4096)
        cells += len(case.cells)
        buckets += 1
        current = struct.unpack_from('>Q', case.bucket, 4)[0]
    return buckets, cells


def check_free_chain(raw: bytes, head: int, roles: dict[int, str]) -> int:
    """Follow BE64 free-page links without invoking allocation; e.g. check_free_chain(raw,head,roles)."""
    current = head
    count = 0
    while current:
        claim_role(roles, current, 'free', len(raw) // 4096)
        current = struct.unpack_from('>Q', page_bytes(raw, current, 4096))[0]
        count += 1
    return count


def inspect_default_file(raw: bytes) -> dict[str, int]:
    """Check selected4096-byte native file roles/requests; e.g. inspect_default_file(raw)."""
    assert raw[:11] == b"unqlite\xdb\x7c\x27\x12" and int.from_bytes(raw[19:23], 'big') == 4096
    index = read_index(raw, 4096)
    check_partition(index)
    roles = {0: 'file-header', 1: 'index'}
    for identity in index.map_pages[1:]:
        claim_role(roles, identity, 'index', len(raw) // 4096)
    buckets = 0
    cells = 0
    for logical, first in index.entries:
        assert first > 1
        observed = check_bucket_chain(raw, logical, first, index, roles)
        buckets += observed[0]
        cells += observed[1]
    free = check_free_chain(raw, index.free_head, roles)
    assert len(roles) == len(raw) // 4096, f'unclassified pages{set(range(len(raw) // 4096)) - set(roles)}'
    return {'pages': len(roles), 'index_pages': len(index.map_pages), 'buckets': buckets,
            'cells': cells, 'free_pages': free, 'overflow_pages': sum(role == 'overflow' for role in roles.values())}


def check_partition(index: NativeIndex) -> None:
    """Admit the selected canonical partition; e.g. check_partition(index)."""
    assert index.maximum > 0 and index.maximum & (index.maximum - 1) == 0
    assert index.split < index.maximum
    if not index.entries and index.split == 0 and index.maximum == 1:
        return
    logical = sorted(identity for identity, _ in index.entries)
    assert logical == list(range(index.split + index.maximum))
