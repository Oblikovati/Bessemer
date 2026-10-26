"""Independent complete feature-stream construction and expected raw-field edit (#46)."""
from collections.abc import Callable
from pathlib import Path
from types import ModuleType
from typing import Protocol
import hashlib
import struct
import sys
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives, verify_primitive_coverage

WORKSPACE = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(WORKSPACE/'docs/features/resolved-features'))
from freeze_owned_features import ReplacedFeatureDocument, STREAM


class FixtureMap(Protocol):
    def new(self, name: str, body: Callable[[], bytes]) -> bytes: ...


def axis_pointer(mapping: FixtureMap, kind: str) -> bytes:
    """Choose a complete independently built pointer; e.g. axis_pointer(mapping, 'temporary')."""
    from freeze_geometric_references import wire as geometry
    from sketch_reference_fixture_bytes import temporary_axis
    from display_dimension_fixture_bytes import body
    if kind == 'null':
        return bytes(2)
    if kind not in ('line', 'temporary'):
        raise ValueError(f'axis kind {kind!r}; expected null, line or temporary')
    name = 'moLineRef_w' if kind == 'line' else 'moTempAxisRef_w'
    raw = geometry(name, 19000, False, 0) if kind == 'line' else temporary_axis(19000, False)
    return mapping.new(name, lambda: body(raw))


def revolution_body(mapping: FixtureMap, producer: ModuleType, kind: str) -> bytes:
    """Replace only the independently declared axis tag; e.g. revolution_body(mapping, producer, 'line')."""
    inherited = producer.header(mapping, 19000, False)+producer.atom_fields(mapping, False, 2)
    inherited += producer.body_suffix(False)
    suffix = producer.suffix(mapping, 19000, 'moRevolution_c', False)
    return inherited+suffix[:-5]+axis_pointer(mapping, kind)+suffix[-3:]


def owned_stream(kind: str) -> bytes:
    """Build a complete one-root stream with mandatory MBSM; e.g. owned_stream('temporary-edited')."""
    import body_feature_fixture_bytes as producer
    mapping = producer.WireMap()
    actual = 'temporary' if kind == 'temporary-edited' else kind
    raw = struct.pack('<iH', -123, 1)+mapping.new('moRevolution_c', lambda:
        revolution_body(mapping, producer, actual))+bytes(4)
    if kind != 'temporary-edited':
        return raw
    name = b'moTempAxisRef_w'
    offset = raw.index(name)+len(name)+62
    if raw[offset:offset+8] != struct.pack('<d', -0.0):
        raise ValueError(f'axis bytes {raw[offset:offset+8].hex()}; expected negative zero at {offset}')
    return raw[:offset]+struct.pack('<d', 3.5)+raw[offset+8:]


def verify_owned_read(parsed: dict[str, object], kind: str, width: int) -> None:
    """Require EOF and the independently expected edit; e.g. verify_owned_read(parsed, 'temporary-edited', 567)."""
    if parsed['end'] != width or parsed['tail'] != b'':
        raise ValueError(f'{kind} end/tail {(parsed["end"], parsed["tail"])}; expected {(width, b"")}')
    if kind != 'temporary-edited':
        return
    values = [entry.get('unknown_a8') for entry in getattr(parsed['archive'], 'map')
              if isinstance(entry, dict) and entry.get('_class') == 'moTempAxisRef_w']
    if values != [3.5]:
        raise ValueError(f'edited axis values {values}; expected one temporary-axis parameter equal to 3.5')


def freeze_owned_case(reference: Path, destination: Path, kind: str) -> dict[str, object]:
    """Require independent full framing; e.g. freeze_owned_case(source, output, 'temporary-edited')."""
    container, features, archive = (source_module(name, reference) for name in
                                    ('swcontainer', 'resolvedfeatures', 'swarchive'))
    baseline = (WORKSPACE/'tests/fixtures/blank-part.SLDPRT').read_bytes()
    raw = owned_stream(kind)
    document = container.Document(baseline, table={})
    with observe_primitives(archive.Archive, raw) as reads:
        parsed = features.read_supported(ReplacedFeatureDocument(document, raw), STREAM)
    verify_owned_read(parsed, kind, len(raw))
    reads = [read for read in reads if read.end > read.start]
    verify_primitive_coverage(raw, reads)
    oracle = ''.join(f'{read.start}\t{read.end}\t{read.kind}\t{read.raw_hex}\n' for read in reads).encode()
    (destination/('owned-'+kind+'.bin')).write_bytes(raw)
    (destination/('owned-'+kind+'.tsv')).write_bytes(oracle)
    return dict(kind=kind, bytes=len(raw), reads=len(reads), sha256=hashlib.sha256(raw).hexdigest(),
                oracle_sha256=hashlib.sha256(oracle).hexdigest(), baseline_sha256=hashlib.sha256(baseline).hexdigest())


def freeze_owned(reference: Path, destination: Path) -> list[dict[str, object]]:
    """Construct control, regression and edit streams; e.g. freeze_owned(source, owned_root)."""
    return [freeze_owned_case(reference, destination, kind)
            for kind in ['null', 'line', 'temporary', 'temporary-edited']]
