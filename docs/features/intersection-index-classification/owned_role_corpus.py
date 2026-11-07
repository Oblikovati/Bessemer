"""Persist complete owned roots and independent Source primitive oracles (#185)."""
from dataclasses import dataclass, field
from pathlib import Path
import hashlib
import json
import struct
import zlib
from freeze_owned_features import ReferenceDocument
from intersection_matrix_oracles import read_intersection_matrix_root
from owned_intersection_roles import OwnedIntersectionRoles, role_matrix_inputs, role_matrix_root, role_full_selector_root
from role_source_oracles import verify_role_source_graph, verify_role_source_primitives, role_scalar_rows, role_primitive_rows


@dataclass
class RoleCorpusPayloads:
    roots: bytearray = field(default_factory=bytearray)
    scalars: bytearray = field(default_factory=bytearray)
    primitives: bytearray = field(default_factory=bytearray)
    cases: list[dict[str, int]] = field(default_factory=list)


def append_role_corpus_case(payloads: RoleCorpusPayloads, document: ReferenceDocument, owned: OwnedIntersectionRoles) -> None:
    """Accept one complete root before publication; e.g. append_role_corpus_case(out, doc, owned)."""
    index = len(payloads.cases)
    parsed, physical = read_intersection_matrix_root(document, owned.raw)
    verify_role_source_graph(parsed, owned)
    verify_role_source_primitives(owned, physical)
    payloads.roots.extend(struct.pack('<I', len(owned.raw)) + owned.raw)
    payloads.scalars.extend(role_scalar_rows(index, owned))
    payloads.primitives.extend(role_primitive_rows(index, physical))
    payloads.cases.append(dict(index=index, bytes=len(owned.raw), indices=len(owned.indices), groups=len(owned.references), primitives=len(physical)))
    print(f'INTERSECTION_ROLE_SOURCE\t{index}\t{len(owned.indices)}\t{len(owned.references)}', flush=True)


def build_role_corpus(document: ReferenceDocument) -> RoleCorpusPayloads:
    """Keep the Cartesian matrix and full u16 owner complete; e.g. build_role_corpus(doc)."""
    payloads = RoleCorpusPayloads()
    for values in role_matrix_inputs():
        append_role_corpus_case(payloads, document, role_matrix_root(*values))
    assert len(payloads.cases) == 264
    domain = role_full_selector_root()
    assert [words[0] for _, words in domain.references[:65536]] == list(range(65536))
    append_role_corpus_case(payloads, document, domain)
    assert sum(row['indices'] for row in payloads.cases) == 16913
    assert sum(row['groups'] for row in payloads.cases) == 67652
    assert sum(row['primitives'] for row in payloads.cases) == 676537
    return payloads


def persist_role_corpus(feature: Path, fixture: Path, payloads: RoleCorpusPayloads, initialize: bool) -> dict[str, object]:
    """Initialize explicitly, then reject every baseline change; e.g. persist_role_corpus(feature, fixture, out, False)."""
    artifacts = {'roots.bin': bytes(payloads.roots), 'scalars.tsv': bytes(payloads.scalars),
                 'primitives.tsv.zlib': zlib.compress(bytes(payloads.primitives), 9)}
    records = {name: dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest()) for name, raw in artifacts.items()}
    reference = dict(cases=payloads.cases, complete_roots=795, indices=16913, groups=67652,
                     literal_scalars=287521, primitive_reads=676537, primitive_bytes=len(payloads.primitives), artifacts=records)
    paths = {fixture / name: raw for name, raw in artifacts.items()}
    paths[feature / 'owned-role-reference.json'] = (json.dumps(reference, indent=2) + '\n').encode()
    for path, raw in paths.items():
        if initialize:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected unchanged complete Source-accepted role artifact'
    return reference
