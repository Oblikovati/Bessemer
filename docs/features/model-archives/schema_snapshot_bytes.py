"""Hash-guarded schema snapshot output and one-layer Windows newline repair (#48)."""
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[3]
PAIRS = [
    ('owned-part-schema-snapshot.md', 'owned-part-schema-reference.json'),
    ('reference-schema-snapshot.md', 'reference-schema-snapshot.json'),
    ('entity-external-schema-snapshot.md', 'entity-external-schema-reference.json'),
    ('groups-relations-schema-snapshot.md', 'groups-relations-schema-reference.json'),
]


def section_digests(snapshot: bytes, expected: dict[str, str]) -> dict[str, str]:
    try:
        text = snapshot.decode('utf-8')
    except UnicodeDecodeError as error:
        raise ValueError(f'snapshot encoding {error}; expected UTF-8 schema text') from error
    result: dict[str, str] = {}
    for section in expected:
        match = re.search(r'^### '+re.escape(section)+r' .*?(?=^### |\Z)', text, re.M | re.S)
        result[section] = hashlib.sha256(match.group().encode()).hexdigest() if match else '<missing>'
    return result


def verified_snapshot(snapshot: bytes, expected: dict[str, str]) -> bytes:
    if not expected:
        raise ValueError(f'section manifest {expected!r}; expected at least one section digest')
    if section_digests(snapshot, expected) == expected:
        return snapshot
    candidate = snapshot.replace(b'\r\n', b'\n')
    observed = section_digests(candidate, expected)
    if observed != expected:
        raise ValueError(f'snapshot section hashes {observed!r}; expected {expected!r}')
    return candidate


def write_verified_snapshot(path: Path, snapshot: bytes, expected: dict[str, str]) -> bytes:
    verified = verified_snapshot(snapshot, expected)
    # #48: text-mode output translates CRLF again on Windows and invalidates provenance.
    path.write_bytes(verified)
    return verified


def section_manifest(value: object) -> dict[str, str]:
    sections = value.get('sections') if isinstance(value, dict) else None
    if not isinstance(sections, dict) or not sections:
        raise ValueError(f'section manifest {sections!r}; expected a nonempty section-to-SHA256 map')
    result: dict[str, str] = {}
    for key, digest in sections.items():
        if not isinstance(key, str) or not isinstance(digest, str) or re.fullmatch('[0-9a-f]{64}', digest) is None:
            raise ValueError(f'section entry {(key, digest)!r}; expected a text key and lowercase SHA256')
        result[key] = digest
    return result


def repair(root: Path = ROOT) -> None:
    base = root/'docs/features/model-archives'
    rows: list[dict[str, object]] = []
    for snapshot_name, manifest_name in PAIRS:
        path = base/snapshot_name
        original = path.read_bytes()
        expected = section_manifest(json.loads((base/manifest_name).read_text()))
        verified = write_verified_snapshot(path, original, expected)
        rows.append({'snapshot': snapshot_name, 'changed': original != verified,
                     'before_sha256': hashlib.sha256(original).hexdigest(),
                     'after_sha256': hashlib.sha256(verified).hexdigest(),
                     'section_hashes': expected})
    report = base/'schema-snapshot-newlines.json'
    # Preserve the first repair's before/after evidence when the verifier is rerun.
    if not report.exists():
        report.write_text(json.dumps({'issue': 48, 'repairs': rows}, indent=2)+'\n')
    print({'issue': 48, 'checked': len(rows), 'repaired': sum(bool(row['changed']) for row in rows)})


if __name__ == '__main__':
    repair()
