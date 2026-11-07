"""Observe complete owned unit graphs from actual saved current-part context (#54)."""
from pathlib import Path
import sys,json
ROOT=Path(__file__).resolve().parents[3]
for name in ('model-archives','trim-edge-identifiers'):
    sys.path.insert(0,str(ROOT/'docs/features'/name))
from freeze_trim_edges import owned,ObjectFixtureFamily,REFERENCE
from unit_bytes import unit_object


def generate() -> None:
    """Save complete reference-validated streams, including the independent configuration-ID edit."""
    destination=ROOT/'tests/fixtures/external-unit-components';destination.mkdir(exist_ok=True)
    family=ObjectFixtureFamily(build=unit_object)
    rows=[owned(REFERENCE,destination,0,style,False,f'owned-{style}',family) for style in range(6)]
    rows.append(owned(REFERENCE,destination,0,2,True,'owned-2-edited',family))
    Path(__file__).with_name('reference.json').write_text(json.dumps(dict(owned=rows),indent=2)+'\n')
    print([(r['label'],r['slots']) for r in rows])
