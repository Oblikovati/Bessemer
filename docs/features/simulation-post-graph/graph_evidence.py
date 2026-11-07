"""Original complete C-765 graph evidence and exact reader source pins (#165)."""
from pathlib import Path
from typing import cast
import json
import sys
from native_carriers import SOURCE, RESEARCH, digest

FEATURE = Path(__file__).resolve().parent
READER = 'cosworks_post_graph_layout'


def graph_baseline(name: str, value: object) -> None:
    """Require unchanged complete evidence except explicit initialization; e.g. graph_baseline('native.json', rows)."""
    path = FEATURE / name
    raw = (json.dumps(value, indent=2) + '\n').encode()
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged graph evidence'


def graph_report() -> dict[str, object]:
    """Retain the original full graph report, including failed candidates; e.g. graph_report()."""
    name = 'cosworks-post-graph-native-evidence.json'
    report = cast(dict[str, object], json.loads((RESEARCH / name).read_bytes()))
    assert report['evidence'] == 'C-765' and not report['target_executed'], report['evidence']
    graph_baseline(name, report)
    return report


def verify_graph_sources() -> None:
    """Check every original source/decompilation pin without hash exceptions; e.g. verify_graph_sources()."""
    for original, expected in cast(dict[str, str], graph_report()['sources']).items():
        relative = original.replace('\\', '/').split('/inventor-interface/')[-1]
        assert relative.startswith('solidworks/'), f'{original}; expected an approved SolidWorks source path'
        actual = digest((SOURCE / relative).read_bytes())
        assert actual == expected, f'{relative}/{actual}; expected original source hash {expected}'


def verify_graph_schema() -> None:
    """Retain complete thresholds, count policy and provider caveats; e.g. verify_graph_schema()."""
    text = (SOURCE / 'docs/solidworks-file-format/specification.md').read_bytes().decode().replace('\r\n', '\n')
    start = text.index('### 14.385 ')
    raw = (text[start:text.index('\n### ', start + 1)].rstrip() + '\n').encode()
    path = FEATURE / 'schema-14.385.md'
    if '--write' in sys.argv: path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete graph clause'
