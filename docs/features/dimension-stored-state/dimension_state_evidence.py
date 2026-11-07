"""Keep current dimension readers separate from historical semantic replays (#179)."""
from pathlib import Path
from typing import cast
import hashlib
import json

FEATURE = Path(__file__).resolve().parent
SOURCE = FEATURE.parents[3] / 'inventor-interface/solidworks'


def stage_dimension_assets(pin: Path, hashes: dict[str, str]) -> None:
    """Give unchanged original tests their exact file-relative inputs; e.g. stage_dimension_assets(pin, hashes)."""
    entries = cast(dict[str, dict[str, str]], json.loads((FEATURE / 'asset-sources.json').read_bytes()))
    assert len(entries) == 7, f'{len(entries)}; expected seven complete original asset copies'
    for destination, entry in entries.items():
        original = SOURCE.parent / entry['source']
        raw = original.read_bytes()
        actual = hashlib.sha256(raw).hexdigest()
        assert actual == entry['sha256'], f'{original}/{actual}; expected asset {entry["sha256"]}'
        target = pin / destination
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(raw)
        assert target.read_bytes() == raw, f'{target}; expected complete frozen asset bytes'
        if target.suffix == '.py':
            hashes[destination] = actual


def verify_dimension_readers() -> None:
    """Reject drift in current readers and original tests; e.g. verify_dimension_readers()."""
    pins = cast(dict[str, str], json.loads((FEATURE / 'reader-sources.json').read_bytes()))
    assert len(pins) == 12, f'{len(pins)}; expected twelve current reader/test pins'
    for name, expected in pins.items():
        actual = hashlib.sha256((SOURCE / name).read_bytes()).hexdigest()
        assert actual == expected, f'{name}/{actual}; expected current source {expected}'


def verify_dimension_clauses() -> None:
    """Preserve complete supported-domain caveats beside the aliases; e.g. verify_dimension_clauses()."""
    text = (SOURCE.parent / 'docs/solidworks-file-format/specification.md').read_text().replace('\r\n', '\n')
    for section in ('11.436', '11.437'):
        start = text.index('### ' + section + ' ')
        raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
        expected = (FEATURE / f'schema-{section}.md').read_bytes()
        assert raw == expected, f'{section}; expected the complete approved dimension clause'


def verify_dimension_reports() -> None:
    """Retain original report bytes and historical accounting without replaying writers; e.g. verify_dimension_reports()."""
    pins = cast(dict[str, str], json.loads((FEATURE / 'report-pins.json').read_bytes()))
    assert len(pins) == 5, f'{len(pins)}; expected all five original dimension reports'
    for name, expected in pins.items():
        raw = (SOURCE / 'research' / name).read_bytes()
        actual = hashlib.sha256(raw).hexdigest()
        assert actual == expected, f'{name}/{actual}; expected original report {expected}'
        assert raw == (FEATURE / 'reports' / name).read_bytes(), name
        report = cast(dict[str, object], json.loads(raw))
        assert report['target_executed'] is False, f'{name}; expected read-only original evidence'
        if 'files' in report:
            assert len(cast(list[object], report['files'])) == 906, name
        if 'module' in report:
            verify_dimension_native_dependencies(report)


def verify_dimension_native_dependencies(report: dict[str, object]) -> None:
    """Check native provenance independently of evolving pure reader revisions; e.g. verify_dimension_native_dependencies(report)."""
    sources = cast(dict[str, str], report['sources'])
    assert sources, f'{report["evidence"]}; expected explicit native dependency pins'
    for original, expected in sources.items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/'), f'{original}; expected source-bound native provenance'
        actual = hashlib.sha256((SOURCE.parent / relative).read_bytes()).hexdigest()
        assert actual == expected, f'{relative}/{actual}; expected native dependency {expected}'
