"""Freeze independent primitive facts for the owned complete CMgr (#54)."""
from pathlib import Path
import hashlib
import json
import sys

WORKSPACE = Path(__file__).resolve().parents[3]
SHARED = WORKSPACE / 'docs/features/model-archives'
sys.path.insert(0, str(SHARED))
from freeze_blank_part import reference_snapshot, source_module
from primitive_inventory import PrimitiveRead, observe_primitives, verify_primitive_coverage


def observe_manager(reference: Path, fixture: Path) -> tuple[bytes, list[PrimitiveRead], dict[str, str]]:
    """Require a complete stable reference read; e.g. observe_manager(source, owned_fixture)."""
    before = reference_snapshot(reference)
    sys.path.insert(0, str(reference))
    container = source_module('swcontainer', reference)
    archive = source_module('swarchive', reference)
    manager = source_module('configurationmanager', reference)
    owned = fixture.read_bytes()
    document = container.Document(owned, table={})
    raw = document.read('Contents/CMgr')
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = manager.read_supported(document)
    if parsed['end'] != len(raw) or before != reference_snapshot(reference):
        raise ValueError('CMgr read/source mismatch; expected exact EOF and unchanged sources')
    if fixture.read_bytes() != owned:
        raise ValueError('owned fixture changed; expected unchanged input bytes')
    nonempty = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, nonempty)
    return raw, nonempty, before


def write_manager_oracle(reference: Path, fixture: Path, destination: Path) -> dict[str, object]:
    """Write caller-owned evidence; e.g. write_manager_oracle(source, fixture, temporary_root)."""
    raw, reads, sources = observe_manager(reference, fixture)
    rows = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    destination.mkdir(parents=True, exist_ok=True)
    (destination / 'owned-manager.bin').write_bytes(raw)
    (destination / 'owned-manager-primitives.tsv').write_bytes(rows)
    return {'feature_issue': 54, 'scope': 'Independent complete owned CMgr primitive partition; no geometry certification.',
            'fixture_sha256': hashlib.sha256(fixture.read_bytes()).hexdigest(),
            'stream_sha256': hashlib.sha256(raw).hexdigest(), 'stream_bytes': len(raw),
            'primitive_reads': len(reads), 'oracle_sha256': hashlib.sha256(rows).hexdigest(),
            'source_hashes': sources, 'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}


if __name__ == '__main__':
    report = write_manager_oracle(WORKSPACE.parent / 'inventor-interface/solidworks',
        WORKSPACE / 'tests/fixtures/blank-part.SLDPRT', WORKSPACE / 'tests/fixtures/resolved-features')
    Path(__file__).with_name('owned-manager-reference.json').write_bytes((json.dumps(report, indent=2)+'\n').encode())
    print({key: value for key, value in report.items() if key != 'source_hashes'})
