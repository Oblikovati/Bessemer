"""Freeze complete owned feature bytes and an independent plane-name edit (#54)."""
from pathlib import Path
from typing import Protocol, cast
from types import ModuleType
import hashlib
import json
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE / 'docs/features/model-archives'))
from freeze_blank_part import reference_snapshot, source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage

STREAM = 'Contents/Config-0-ResolvedFeatures'
NEW_NAME = 'Bessemer Plane Ω'


class ReferenceDocument(Protocol):
    def names(self) -> list[str]: ...
    def read(self, name: str) -> bytes: ...


class ReplacedFeatureDocument:
    def __init__(self, original: ReferenceDocument, replacement: bytes) -> None:
        self.original = original
        self.replacement = replacement

    def names(self) -> list[str]:
        return self.original.names()

    def read(self, name: str) -> bytes:
        return self.replacement if name == STREAM else self.original.read(name)


def plane_name(result: dict[str, object]) -> str:
    """Require the owned plane's semantic name; e.g. plane_name(complete_reference_read)."""
    roots = cast(list[dict[str, object]], result['items'])
    root = roots[13]
    if root['_class'] != 'moRefPlane_c':
        raise ValueError(f"root 13 class {root['_class']!r}; expected moRefPlane_c")
    for member in ['model_feature', 'feature', 'node', 'name']:
        root = cast(dict[str, object], root[member])
    value = root['name']
    if not isinstance(value, str):
        raise ValueError(f'plane name {value!r}; expected a decoded string')
    return value


def independent_plane_edit(raw: bytes) -> tuple[bytes, int, int]:
    """Patch one verified CString without Bessemer; e.g. independent_plane_edit(owned_stream)."""
    old = 'Front Plane'
    patterns = [bytes([len(old)])+old.encode('ascii'), b'\xff\xfe\xff'+bytes([len(old)])+old.encode('utf-16le')]
    candidates = [(raw.find(pattern), len(pattern)) for pattern in patterns if raw.count(pattern) == 1]
    if len(candidates) != 1 or sum(raw.count(pattern) for pattern in patterns) != 1:
        raise ValueError(f'plane CString matches {candidates!r}; expected one unique stored name')
    start, width = candidates[0]
    replacement = b'\xff\xfe\xff'+bytes([len(NEW_NAME)])+NEW_NAME.encode('utf-16le')
    return raw[:start]+replacement+raw[start+width:], start, width


def freeze_features(reference: Path, fixture: Path, destination: Path) -> dict[str, object]:
    """Bind both full reference reads; e.g. freeze_features(source, fixture, temporary_root)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    container = source_module('swcontainer', reference)
    archive = source_module('swarchive', reference)
    features = source_module('resolvedfeatures', reference)
    owned = fixture.read_bytes()
    document = container.Document(owned, table={})
    raw = document.read(STREAM)
    with observe_primitives(archive.Archive, raw) as reads:
        original = features.read_supported(document, STREAM)
    edited, start, width = verified_edit(features, document, raw, original)
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    if before != reference_snapshot(reference) or fixture.read_bytes() != owned:
        raise ValueError('source/fixture changed; expected stable read-only inputs')
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in nonempty).encode()
    return write_evidence(destination, raw, edited, rows, start, width, before)


def verified_edit(features: ModuleType, document: ReferenceDocument, raw: bytes,
                  original: dict[str, object]) -> tuple[bytes, int, int]:
    """Require the independent edit's semantic result; e.g. verified_edit(reader, doc, raw, parsed)."""
    if plane_name(original) != 'Front Plane':
        raise ValueError('owned plane name changed; expected Front Plane')
    edited, start, width = independent_plane_edit(raw)
    checked = features.read_supported(ReplacedFeatureDocument(document, edited), STREAM)
    if plane_name(checked) != NEW_NAME or checked['end'] != len(edited):
        raise ValueError('edited plane/read mismatch; expected replacement name and exact EOF')
    return edited, start, width


def write_evidence(destination: Path, raw: bytes, edited: bytes, rows: bytes,
                   start: int, width: int, sources: dict[str, str]) -> dict[str, object]:
    """Persist owned artifacts; e.g. write_evidence(root, raw, edited, rows, offset, width, hashes)."""
    destination.mkdir(parents=True, exist_ok=True)
    for name, payload in [('owned-features.bin', raw), ('owned-features-plane-edited.bin', edited), ('owned-features-primitives.tsv', rows)]:
        (destination / name).write_bytes(payload)
    return {'scope': 'Independent complete owned feature read and plane-name edit; no geometry certification.',
        'stream_bytes': len(raw), 'stream_sha256': hashlib.sha256(raw).hexdigest(),
        'edited_bytes': len(edited), 'edited_sha256': hashlib.sha256(edited).hexdigest(),
        'primitive_reads': len(rows.splitlines()), 'oracle_sha256': hashlib.sha256(rows).hexdigest(),
        'edit_start': start, 'original_cstring_bytes': width, 'replacement': NEW_NAME,
        'source_hashes': sources, 'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


if __name__ == '__main__':
    report = freeze_features(WORKSPACE.parent / 'inventor-interface/solidworks',
        WORKSPACE / 'tests/fixtures/blank-part.SLDPRT', WORKSPACE / 'tests/fixtures/resolved-features')
    Path(__file__).with_name('owned-features-reference.json').write_bytes((json.dumps(report,indent=2)+'\n').encode())
    print({key: value for key, value in report.items() if key != 'source_hashes'})
