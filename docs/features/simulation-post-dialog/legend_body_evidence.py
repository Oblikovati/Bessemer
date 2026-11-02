"""Common exact Ghidra function/body checks for stored legend-control consumers (#166)."""
from pathlib import Path
from typing import cast
import json
from native_carriers import digest


def legend_body_capture(path: Path, method: dict[str, object], module_sha256: str) -> dict[str, object]:
    """Require every original location and the complete contiguous body; e.g. legend_body_capture(path, method, module_hash)."""
    record = json.loads(path.read_bytes())
    assert record['subject']['digest']['sha256'] == module_sha256
    assert record['provider'] == dict(id='ghidra', name='Ghidra', version='12.1.4')
    assert record['authority'] == 'shipped-artifact' and record['confidence'] == 'observed'
    result = record['normalized_result']
    assert record['raw_result']['value'] == result
    instructions = result['instructions'] if 'instructions' in result else result['assembly']
    expected = ['0x' + line.split()[0] for line in cast(list[str], method['instructions'])]
    assert [line.split(':', 1)[0] for line in instructions] == expected
    body = result['procedure']['body']
    assert body['available'] and body['contains_entry'] and not body['non_contiguous']
    start, end = (int(cast(str, method[key]), 16) for key in ('start', 'end'))
    assert body['ranges'] == [dict(start=hex(start), end=hex(end - 1))]
    assert body['total_bytes'] == body['span_bytes'] == end - start
    return dict(path=path.name, sha256=digest(path.read_bytes()), evidence_id=record['evidence_id'],
        instructions=len(instructions), limitations=record['limitations'])
