"""Recover complete C-716 streams through the approved document adapters (#134)."""
from pathlib import Path
from typing import TypedDict, cast
import hashlib
import json
import struct

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
SOURCE = ROOT.parent / 'inventor-interface'
RESEARCH = SOURCE / 'solidworks/research'


class ThermalCarrier(TypedDict):
    path: str
    stream: str
    bytes: int
    sha256: str
    attempts: list[dict[str, object]]


def digest(raw: bytes) -> str:
    """Identify every byte of a retained image; e.g. digest(b'')."""
    return hashlib.sha256(raw).hexdigest()


def retained_carriers() -> list[ThermalCarrier]:
    """Read the complete retained candidate set; e.g. retained_carriers()."""
    path = RESEARCH / 'cosworks-binary-thermal-transform-native-evidence.json'
    inventory = json.loads(path.read_bytes())
    assert inventory['evidence'] == 'C-716' and not inventory['target_executed']
    return cast(list[ThermalCarrier], inventory['rows'])


def pinned_auxiliaries(pin: Path, hashes: dict[str, str]) -> None:
    """Freeze the complete external CFB adapter and existing key cache; e.g. pinned_auxiliaries(pin, hashes)."""
    original = SOURCE / 'reader/cfb.py'
    raw = original.read_bytes()
    hashes['cfb.py'] = digest(raw)
    (pin / 'cfb.py').write_bytes(raw)
    assert digest(original.read_bytes()) == hashes['cfb.py']
    cache = SOURCE / 'solidworks/cache/keytable.json'
    cached = cache.read_bytes()
    (pin / 'keytable.json').write_bytes(cached)
    assert cache.read_bytes() == cached


def recover_sample(path: str, pin: Path) -> dict[str, bytes]:
    """Verify the complete outer envelope before extracting streams; e.g. recover_sample(path, pin)."""
    from third_party_storage_corpus import Document, load_key_table, CFB
    inventory = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    sample = next(row for row in inventory['samples'] if row['path'] == path)
    table = load_key_table(cache=str(pin / 'keytable.json'))
    outer = Document(path, table).read('ThirdPtyStore/ThirdPtySTGStore')
    assert digest(outer) == sample['stream_sha256']
    assert struct.unpack_from('<I', outer)[0] == len(outer) - 4
    scratch = pin / 'nested.cfb'
    scratch.write_bytes(outer[4:])
    archive = CFB(scratch)
    return {entry.path: archive.read(entry) for entry in archive.walk() if entry.type == 2}


def recover_carriers(pin: Path) -> list[tuple[ThermalCarrier, bytes]]:
    """Recover every full thermal stream without executing target code; e.g. recover_carriers(pin)."""
    carriers = retained_carriers()
    result: list[tuple[ThermalCarrier, bytes]] = []
    for path in sorted({row['path'] for row in carriers}):
        streams = recover_sample(path, pin)
        for row in (row for row in carriers if row['path'] == path):
            raw = streams[row['stream']]
            assert len(raw) == row['bytes'] and digest(raw) == row['sha256']
            result.append((row, raw))
    assert len(result) == 68 and sum(len(raw) for _, raw in result) == 267820
    return result
