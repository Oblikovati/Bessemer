"""Complete independent Source roots and primitive/scalar goldens (#188)."""
from dataclasses import dataclass, field
from pathlib import Path
from typing import cast
import hashlib
import itertools
import json
import struct
import zlib
from freeze_owned_features import ReferenceDocument, ReplacedFeatureDocument, STREAM
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage
from lookup_key_owned import LOOKUP_KEY_VALUES, OwnedSketchLookupKeys, owned_lookup_key_root

@dataclass
class LookupKeyCorpus:
    roots: bytearray = field(default_factory=lambda: bytearray(b'SKK1' + struct.pack('<I', 400)))
    scalars: bytearray = field(default_factory=bytearray)
    primitives: bytearray = field(default_factory=bytearray)
    cases: list[dict[str, int]] = field(default_factory=list)


def lookup_key_source_read(document: ReferenceDocument, owned: OwnedSketchLookupKeys) -> tuple[dict[str, object], list[PrimitiveRead]]:
    """Observe complete original grammar, never a role adapter; e.g. lookup_key_source_read(doc, owned)."""
    from resolvedfeatures import read_supported
    from swarchive import Archive
    with observe_primitives(Archive, owned.raw) as reads:
        parsed = read_supported(ReplacedFeatureDocument(document, owned.raw), STREAM)
    physical = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(owned.raw, physical)
    return parsed, physical


def verify_lookup_key_graph(parsed: dict[str, object], owned: OwnedSketchLookupKeys) -> None:
    """Check exact EOF, physical owners and alias identity; e.g. verify_lookup_key_graph(parsed, owned)."""
    assert parsed['end'] == len(owned.raw) and parsed['list_complete']
    items = cast(list[dict[str, object]], parsed['items'])
    assert len(items) == 3 and items[0] is items[2] and items[0] is not items[1]
    assert len(getattr(parsed['archive'], 'map')) == 100
    for item, keys, profile in zip(items[:2], owned.keys, owned.profiles, strict=True):
        verify_lookup_key_owner(item, keys, profile)


def verify_lookup_key_owner(item: dict[str, object], keys: tuple[int, int], profile: int) -> None:
    """Keep saved keys separate from all signed neighbors; e.g. verify_lookup_key_owner(item, (-12345, 1), 0)."""
    assert item['_class'] == 'moSketchExtRef_w'
    assert (item['unknown_6c'], item['unknown_70']) == keys
    assert (item['unknown_64'], item['unknown_08']) == (-2147483646 + profile, 1879048192 + profile)
    assert (item['unknown_0c'], item['unknown_20'], item['unknown_74']) == (-2147483647 + profile, 2147483647 - profile, -13)
    assert tuple(item['unknown_' + offset] for offset in ('4c', '68', '34', '30', '90', '38')) == (-31 - profile, 27 + profile, -39 - profile, -43 - profile, -47 - profile, -53 - profile)
    assert item['component_reference'] is None and item['component'] is None and item['backup'] is None
    assert item['unknown_60'] == 65535 and item['mode'] == (-2, -1, 0, 0)[profile]


def verify_lookup_key_primitives(owned: OwnedSketchLookupKeys, physical: list[PrimitiveRead]) -> None:
    """Bind both physical key pairs to signed primitive spans; e.g. verify_lookup_key_primitives(owned, reads)."""
    expected = [(offset, offset + 4, 'i', struct.pack('<i', value).hex()) for offsets, keys in zip(owned.key_offsets, owned.keys, strict=True) for offset, value in zip(offsets, keys, strict=True)]
    selected = [(read.start, read.end, read.kind, read.raw_hex) for read in physical if read.start in {span[0] for span in expected}]
    assert selected == expected, (selected, expected)


def append_lookup_key_case(corpus: LookupKeyCorpus, document: ReferenceDocument, owned: OwnedSketchLookupKeys) -> None:
    """Retain accepted raw roots and complete Source primitives; e.g. append_lookup_key_case(corpus, doc, owned)."""
    case = len(corpus.cases)
    parsed, physical = lookup_key_source_read(document, owned)
    verify_lookup_key_graph(parsed, owned)
    verify_lookup_key_primitives(owned, physical)
    corpus.roots.extend(struct.pack('<I', len(owned.raw)) + owned.raw)
    for owner, (offsets, keys) in enumerate(zip(owned.key_offsets, owned.keys, strict=True)):
        corpus.scalars.extend(lookup_key_scalar_rows(case, owner, offsets, keys))
    corpus.primitives.extend(''.join(f'{case}\t{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\t{read.source_file}\t{read.source_function}\t{read.source_line}\n' for read in physical).encode())
    corpus.cases.append(dict(case=case, bytes=len(owned.raw), primitives=len(physical), first_profile=owned.profiles[0], second_profile=owned.profiles[1]))


def lookup_key_scalar_rows(case: int, owner: int, offsets: tuple[int, int], keys: tuple[int, int]) -> bytes:
    """Retain independently selected values and physical offsets; e.g. lookup_key_scalar_rows(0, 0, (54, 58), (-12345, 1))."""
    return ''.join(f'{case}\t{owner}\t{role}\t{offset}\t{value}\n' for role, offset, value in zip(('configuration_object_key', 'configuration_record_key'), offsets, keys, strict=True)).encode()


def build_lookup_key_corpus(document: ReferenceDocument) -> LookupKeyCorpus:
    """Use the full signed-pair/presence product; e.g. build_lookup_key_corpus(doc)."""
    corpus = LookupKeyCorpus()
    for object_key, record_key, profile in itertools.product(LOOKUP_KEY_VALUES, LOOKUP_KEY_VALUES, range(4)):
        append_lookup_key_case(corpus, document, owned_lookup_key_root(object_key, record_key, profile))
    assert len(corpus.cases) == 400
    return corpus


def persist_lookup_key_corpus(feature: Path, fixture: Path, corpus: LookupKeyCorpus, initialize: bool) -> dict[str, object]:
    """Initialize only explicitly, then reject every baseline change; e.g. persist_lookup_key_corpus(feature, fixture, corpus, False)."""
    artifacts = {'roots.bin': bytes(corpus.roots), 'scalars.tsv': bytes(corpus.scalars), 'primitives.tsv.zlib': zlib.compress(bytes(corpus.primitives), 9)}
    reference = dict(issue=188, behavior_version=1, cases=corpus.cases, complete_roots=1200, physical_owners=800, literal_keys=1600, primitive_reads=sum(case['primitives'] for case in corpus.cases), primitive_bytes=len(corpus.primitives), artifacts={name: dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest()) for name, raw in artifacts.items()})
    paths = {fixture / name: raw for name, raw in artifacts.items()}
    paths[feature / 'owned-fixture-reference.json'] = (json.dumps(reference, indent=2) + '\n').encode()
    for path, raw in paths.items():
        if initialize:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected unchanged complete Source-owned key fixtures'
    return reference
