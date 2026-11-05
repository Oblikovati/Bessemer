"""Diagnostic current-source corpus measurement; no audit expectations are replaced (#177)."""
from pathlib import Path
from typing import cast
import hashlib
import json

FEATURE = Path(__file__).resolve().parent


def capture_current_resolved_row(row: dict[str, object]) -> dict[str, object]:
    """Retain exact inputs and complete or failed source outcomes; e.g. capture_current_resolved_row(row)."""
    import resolvedfeatures, swcontainer, swcover
    from swarchive import ArchiveError
    path = Path(swcover.SAMPLES) / cast(str, row['path'])
    configuration = cast(str, row['configuration'])
    document = swcontainer.Document(path)
    raw = document.read(configuration + '-ResolvedFeatures')
    actual = hashlib.sha256(raw).hexdigest()
    assert actual == row['source_sha256'] and len(raw) == row['bytes'], row
    inputs = [hashlib.sha256(document.read(name)).hexdigest() for name in
              (configuration + '-ModelHeader', configuration, 'Contents/CMgr', configuration + '-ResolvedFeatures')]
    result = dict(path=str(path), configuration=configuration, bytes=len(raw), input_hashes=inputs,
                  audit_known_bytes=cast(int, row['bytes']) - cast(int, row['unknown_bytes']))
    try:
        parsed = resolvedfeatures.read_supported(document, configuration + '-ResolvedFeatures')
    except (ArchiveError, UnicodeDecodeError, KeyError, ValueError) as cause:
        return result | dict(completed=False, cause_type=type(cause).__name__, cause=str(cause))
    assert parsed['end'] == len(raw) and 0 <= parsed['unknown_bytes'] <= len(raw), result
    return result | dict(completed=True, end=parsed['end'], unknown_bytes=parsed['unknown_bytes'],
                         known_bytes=parsed['end'] - parsed['unknown_bytes'])


def measure_current_resolved_corpus(pin: Path) -> None:
    """Measure every retained row independently of the failing original aggregate; e.g. measure_current_resolved_corpus(pin)."""
    report = json.loads((pin / 'research/resolved-library-audit-0.382.json').read_bytes())
    rows = [capture_current_resolved_row(row) for row in report['files']]
    assert len(rows) == 906, f'{len(rows)}; expected all906 original audit rows'
    complete = [row for row in rows if row['completed']]
    disagreements = [row for row in complete if row['known_bytes'] != row['audit_known_bytes']]
    measurement = dict(rows=rows, cases=len(rows), completed=len(complete), failed=len(rows) - len(complete),
                       current_known_bytes=sum(cast(int, row['known_bytes']) for row in complete),
                       accounting_disagreements=len(disagreements), original_test_expectations_unchanged=True,
                       qualification='Diagnostic measurement only; the original source accounting test still fails.')
    (FEATURE / 'current-source-outcomes.json').write_bytes((json.dumps(measurement, indent=2) + '\n').encode())
