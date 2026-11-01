"""Fresh MeshControl header and counted-record reference foundation without target execution (#161)."""
from pathlib import Path
from typing import cast
from functools import partial, update_wrapper
import struct
import json
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
for folder in ('simulation-binary-thermal', 'model-archives', 'simulation-solid-manager', 'simulation-solid-body',
               'simulation-shell-manager', 'simulation-shell-property', 'simulation-solid-inventory', 'simulation-sheet-property',
               'simulation-material'):
    sys.path.insert(0, str(FEATURE.parent / folder))
sys.path.insert(0, str(ROOT / 'target/reference-python'))
import pefile
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from native_carriers import RESEARCH, digest, recover_sample, pinned_auxiliaries
from freeze_body import verify_unimported_boundary, verify_model_bindings
from freeze_solid_manager import verify_image
from freeze_shell import verify_shell_image
from mesh_control_evidence import mesh_baseline, mesh_reports, verify_mesh_source_bindings, verify_mesh_refinement_rejections, verify_mesh_schema_clauses
from solid_inventory_bindings import complete_identity_match
from mesh_header_packets import verify_owned_mesh_header_packets, replay_native_mesh_header_packets
from mesh_record_packets import verify_owned_mesh_record_packets, replay_native_mesh_record_packets, verify_mesh_record_factory_rejections, verify_mesh_record_scope_rejections
from mesh_record_malformed import verify_mesh_record_mutations
from mesh_record_caps import verify_mesh_record_caps
from mesh_record_extents import verify_mesh_record_extents


def verify_mesh_corpus_bindings() -> None:
    """Require every C-590 header/collection identity before replay; e.g. verify_mesh_corpus_bindings()."""
    corpus = json.loads((RESEARCH / 'third-party-storage-corpus.json').read_bytes())
    suffixes = ('/MeshControlManager/Property', '/MeshControlManager/MeshControl')
    identities = [dict(path=sample['path'], stream=row['name'], bytes=row['bytes'], sha256=row['sha256'])
                  for sample in corpus['samples'] for row in sample['nested_streams'] if row['name'].endswith(suffixes)]
    keys = ('path', 'stream', 'bytes', 'sha256')
    actual = [{key: row[key] for key in keys} for report in mesh_reports()
              for row in cast(list[dict[str, object]], report['rows'])]
    complete_identity_match(identities, actual)
    assert len(identities) == 370 and sum(row['bytes'] for row in identities) == 89895
    mesh_baseline('identity-manifest-reference.json', identities)


def actual_mesh_payload(raw: bytes, version: int, records: bool) -> dict[str, object]:
    """Keep direct header and counted record dispatch separate; e.g. actual_mesh_payload(raw, 840013, False)."""
    from cosworks_mesh_control_header import parse as header
    from cosworks_mesh_control_layout import parse as collection
    from cosworks_embedded_const_surface import archive_reader
    if records: return cast(dict[str, object], collection(raw, version, 'MeshControl', archive_reader=archive_reader))
    return cast(dict[str, object], header(raw, version))


def verify_mesh_installed_bindings() -> None:
    """Check installed images and the +8 serialization-subobject slot without loading code; e.g. verify_mesh_installed_bindings()."""
    reports = mesh_reports()
    root = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS')
    for filename, key in (('Simulation/cosworks.dll', 'module_sha256'), ('sldmodu.dll', 'model_sha256')):
        raw = (root / filename).read_bytes()
        image = pefile.PE(data=raw, fast_load=True)
        try:
            if key == 'module_sha256': verify_mesh_module(image, raw, reports)
            else:
                assert digest(raw) == reports[1][key]
                verify_model_bindings(image, reports[1])
        finally:
            image.close()


def verify_mesh_module(image: pefile.PE, raw: bytes, reports: list[dict[str, object]]) -> None:
    """Bind all header literals, thresholds, functions and counted-reader dispatch; e.g. verify_mesh_module(image, raw, reports)."""
    header, records = reports
    assert digest(raw) == header['module_sha256'] == records['module_sha256']
    verify_shell_image(image, header)
    verify_image(image, records)
    slot = int(cast(str, header['record_slot']), 16)
    reader = int(cast(str, header['record_reader']), 16)
    assert struct.unpack('<Q', image.get_data(slot - image.OPTIONAL_HEADER.ImageBase, 8))[0] == reader
    assert header['record_slot'] == records['reader_slot'] and header['record_reader'] == records['reader']


def mesh_native_row(row: dict[str, object], streams: dict[str, bytes], records: bool) -> bytes:
    """Preserve every full original outcome including all 31 header failures; e.g. mesh_native_row(row, streams, False)."""
    raw = streams[cast(str, row['stream'])]
    assert len(raw) == row['bytes'] and digest(raw) == row['sha256'], row['stream']
    packet = bytearray()
    for expected in cast(list[dict[str, object]], row['attempts']):
        version = cast(int, expected['version'])
        parsed = actual_mesh_payload(raw, version, records)
        assert dict(version=version, **parsed) == expected, (row['stream'], version)
        packet.extend(struct.pack('<III', version, int(cast(bool, parsed['success'])), len(raw)) + raw)
    return bytes(packet)


def mesh_document_packets(document: str, streams: dict[str, bytes], reports: list[dict[str, object]]) -> tuple[bytes, bytes]:
    """Keep every occurrence rather than deduplicating equal payloads; e.g. mesh_document_packets(document, streams, reports)."""
    rows = [cast(list[dict[str, object]], report['rows']) for report in reports]
    header = b''.join(mesh_native_row(row, streams, False) for row in rows[0] if row['path'] == document)
    records = b''.join(mesh_native_row(row, streams, True) for row in rows[1] if row['path'] == document)
    return header, records


def replay_mesh_native(pin: Path) -> None:
    """Freshly recover all 370 identities and preserve the DocumentInfo rejection; e.g. replay_mesh_native(pin)."""
    reports = mesh_reports()
    packets = [bytearray(), bytearray()]
    documents = sorted({cast(str, row['path']) for report in reports for row in cast(list[dict[str, object]], report['rows'])})
    for document in documents:
        fragments = mesh_document_packets(document, recover_sample(document, pin), reports)
        for packet, fragment in zip(packets, fragments, strict=True): packet.extend(fragment)
    summaries: list[dict[str, object]] = []
    for name, report, packet in zip(('header', 'records'), reports, packets, strict=True):
        (ROOT / f'target/simulation-mesh-control-native-{name}.bin').write_bytes(packet)
        rows = cast(list[dict[str, object]], report['rows'])
        attempts = [attempt for row in rows for attempt in cast(list[dict[str, object]], row['attempts'])]
        summaries.append(dict(evidence=report['evidence'], streams=len(rows), logical_bytes=sum(cast(int, row['bytes']) for row in rows),
                         arguments=len(attempts), accepted=sum(cast(bool, attempt['success']) for attempt in attempts),
                         failed=sum(not attempt['success'] for attempt in attempts), packet_bytes=len(packet), packet_sha256=digest(packet)))
    mesh_baseline('native-reference.json', summaries)


def run_mesh_checks(pin: Path) -> unittest.TestResult:
    """Keep original source tests beside complete native and binding checks; e.g. run_mesh_checks(pin)."""
    import test_cosworks_mesh_control_header
    import test_cosworks_mesh_control
    replay = update_wrapper(partial(replay_mesh_native, pin), replay_mesh_native)
    headers = update_wrapper(partial(replay_native_mesh_header_packets, pin), replay_native_mesh_header_packets)
    records = update_wrapper(partial(replay_native_mesh_record_packets, pin), replay_native_mesh_record_packets)
    checks = (verify_mesh_corpus_bindings, verify_mesh_source_bindings, verify_mesh_refinement_rejections, verify_mesh_schema_clauses,
              verify_mesh_installed_bindings, verify_owned_mesh_header_packets, headers, verify_owned_mesh_record_packets,
              verify_mesh_record_factory_rejections, verify_mesh_record_scope_rejections, verify_mesh_record_mutations,
              verify_mesh_record_caps, verify_mesh_record_extents, records, replay, verify_unimported_boundary)
    suite = unittest.TestSuite(unittest.FunctionTestCase(check) for check in checks)
    for module in (test_cosworks_mesh_control_header, test_cosworks_mesh_control):
        suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(module))
    return unittest.TextTestRunner(verbosity=2).run(suite)


def run() -> bool:
    """Freeze parsers and native carrier adapters before imports; e.g. run()."""
    with tempfile.TemporaryDirectory(prefix='bessemer-mesh-control-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        pinned_auxiliaries(pin, hashes)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin / 'research'))
        result = run_mesh_checks(pin)
        verify_unimported_boundary()
        readers = ('cosworks_mesh_control_header', 'cosworks_mesh_control_layout', 'cosworks_embedded_const_surface',
                   'cosworks_embedded_body_ids', 'cfb', 'swcontainer')
        record_snapshot(pin, hashes, result, FEATURE, readers, ())
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'source-snapshot.json')
        return result.wasSuccessful()


if __name__ == '__main__':
    raise SystemExit(0 if run() else 1)
