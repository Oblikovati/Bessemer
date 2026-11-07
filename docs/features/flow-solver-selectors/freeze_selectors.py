"""Freeze solver selector decisions independently from Bessemer."""
from pathlib import Path
import hashlib
import json
import struct
import sys
ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / 'inventor-interface' / 'solidworks'
sys.path.insert(0, str(SOURCE))
import flowresults
import flowsolverselectors
from swarchive import ArchiveError


def build_carrier() -> tuple[bytes, list[int]]:
    identifiers = sorted(set(range(256)) | set(range(0xffffff00, 0x100000000)) |
                         {1 << bit for bit in range(32)} | {0xffffffff ^ (1 << bit) for bit in range(32)})
    raw = struct.pack('<H', 8)
    for identifier in identifiers + list(reversed(identifiers)):
        payload = struct.pack('<I', identifier)
        raw += struct.pack('<IQ', identifier, len(payload)) + payload
    return raw, identifiers


def select_decisions(raw: bytes, identifiers: list[int]) -> list[str]:
    document = flowresults.read(raw)
    decisions = []
    for identifier in identifiers + [0x12345678]:
        try:
            selected = flowsolverselectors.latest(document, identifier)
            ordinal = document['records'].index(selected) if selected is not None else -1
            # Equal duplicate records have different physical offsets, so index is unambiguous.
            decisions.append(f'{identifier}\t{ordinal}')
        except ArchiveError:
            decisions.append(f'{identifier}\treserved')
    return decisions


def freeze(root: Path = ROOT) -> None:
    raw, identifiers = build_carrier()
    document = flowresults.read(raw)
    decisions = select_decisions(raw, identifiers)
    folder = root / 'tests/fixtures/flow-solver-selectors'
    folder.mkdir(parents=True, exist_ok=True)
    (folder / 'carrier.bin').write_bytes(raw)
    (folder / 'decisions.tsv').write_text('\n'.join(decisions) + '\n', encoding='utf-8', newline='\n')
    proof = dict(records=len(document['records']), queries=len(decisions), carrier_sha256=hashlib.sha256(raw).hexdigest(),
                 source_lf_sha256={name: hashlib.sha256((SOURCE / name).read_bytes().replace(b'\r\n', b'\n')).hexdigest()
                                   for name in ('flowresults.py', 'flowsolverselectors.py')})
    (root / 'docs/features/flow-solver-selectors/baseline-verification.json').write_text(json.dumps(proof, indent=2) + '\n', encoding='utf-8', newline='\n')


if __name__ == '__main__':
    freeze()
