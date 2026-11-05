"""Persist the complete bounded root matrix beside independent Source primitive oracles (#184)."""
from dataclasses import dataclass, field
from pathlib import Path
import hashlib
import json
import struct
import zlib
from freeze_owned_features import ReferenceDocument
from owned_intersection_match import ReferenceMatchWords, intersection_match_stream
from owned_intersection_matrix import intersection_match_matrix_inputs, intersection_match_matrix_groups
from intersection_matrix_oracles import read_intersection_matrix_root, verify_intersection_matrix_graph, verify_intersection_matrix_spans, intersection_matrix_primitive_rows

MATRIX_GROUP_STARTS: tuple[int, ...] = (440, 460, 480, 500, 1187, 1207, 1227, 1247)


@dataclass
class IntersectionMatrixPayloads:
    roots: bytearray = field(default_factory=bytearray)
    words: list[str] = field(default_factory=list)
    primitives: bytearray = field(default_factory=bytearray)


def intersection_matrix_literal(original: bytes, groups: tuple[ReferenceMatchWords, ...]) -> bytes:
    """Patch only the reviewed reference prefixes independently; e.g. intersection_matrix_literal(control, groups)."""
    assert len(original) == 1518 and len(groups) == 8, 'Expected complete1518-byte control and eight groups'
    expected = bytearray(original)
    for start, words in zip(MATRIX_GROUP_STARTS, groups, strict=True):
        struct.pack_into('<H3i', expected, start, *words)
    return bytes(expected)


def append_intersection_matrix_case(payloads: IntersectionMatrixPayloads, document: ReferenceDocument, original: bytes, index: int, inputs: tuple[int, int, int, int]) -> None:
    """Bind constructor output to literal bytes and the complete reader; e.g. append_intersection_matrix_case(out, doc, control, 0, inputs)."""
    first, second = intersection_match_matrix_groups(*inputs)
    raw, starts = intersection_match_stream(first, second)
    assert starts == list(MATRIX_GROUP_STARTS), starts
    assert raw == intersection_matrix_literal(original, first + second), index
    parsed, physical = read_intersection_matrix_root(document, raw)
    verify_intersection_matrix_graph(parsed, len(raw), (first, second))
    verify_intersection_matrix_spans(raw, starts, first + second, physical)
    payloads.roots.extend(struct.pack('<I', len(raw)) + raw)
    payloads.primitives.extend(intersection_matrix_primitive_rows(index, physical))
    for ordinal, (start, words) in enumerate(zip(starts, first + second, strict=True)):
        payloads.words.append(f'{index}\t{ordinal}\t{start}\t' + '\t'.join(map(str, words)) + '\n')


def build_intersection_matrix(document: ReferenceDocument, original: bytes) -> IntersectionMatrixPayloads:
    """Run every declared case without state/value exclusions; e.g. build_intersection_matrix(document, control)."""
    expected = 'e75d60beec167841c83bb9054c026720f564a02782ddc21a9a50112560920100'
    assert hashlib.sha256(original).hexdigest() == expected, 'Expected unchanged Source-accepted complete control'
    payloads = IntersectionMatrixPayloads()
    for index, inputs in enumerate(intersection_match_matrix_inputs()):
        append_intersection_matrix_case(payloads, document, original, index, inputs)
        print(f'INTERSECTION_MATRIX_SOURCE\t{index}\t8\t32', flush=True)
    assert len(payloads.words) == 10752 and len(payloads.roots) == 2045568
    return payloads


def persist_intersection_matrix(fixture: Path, feature: Path, payloads: IntersectionMatrixPayloads, initialize: bool) -> dict[str, object]:
    """Keep initialized literals immutable on every replay; e.g. persist_intersection_matrix(fixtures, feature, payloads, False)."""
    artifacts = {'matrix.bin': bytes(payloads.roots), 'matrix-words.tsv': ''.join(payloads.words).encode(),
                 'matrix-primitives.tsv.zlib': zlib.compress(bytes(payloads.primitives), 9)}
    records = {name: dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest()) for name, raw in artifacts.items()}
    reference = dict(cases=1344, roots=4032, physical_sketches=2688, groups=10752, literal_scalars=43008,
                     primitive_reads=606144, primitive_bytes=len(payloads.primitives), artifacts=records)
    encoded = (json.dumps(reference, indent=2) + '\n').encode()
    paths = {fixture / name: raw for name, raw in artifacts.items()}
    paths[feature / 'owned-matrix-reference.json'] = encoded
    for path, raw in paths.items():
        if initialize:
            path.write_bytes(raw)
        assert path.read_bytes() == raw, f'{path}; expected unchanged complete matrix artifact'
    return reference
