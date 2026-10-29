"""Read the owned assembly through complete immutable reference serializers (#99)."""
from pathlib import Path
import hashlib
import json
import struct
import sys
import tempfile
import unittest

FEATURE = Path(__file__).resolve().parent
ROOT = FEATURE.parents[2]
sys.path.insert(0, str(FEATURE.parent / 'model-archives'))
from pinned_reference_tests import FrozenSourceImports, copy_sources, record_snapshot
from primitive_inventory import observe_primitives, verify_primitive_coverage
from primitive_inventory import PrimitiveRead
from freeze_string_maps import ConfigurationDocument, SelectedConfigurationDocument


def freeze_bytes(path: Path, raw: bytes) -> None:
    """Require a reviewed baseline unless fixture creation is explicitly selected."""
    if '--write' in sys.argv:
        path.write_bytes(raw)
        return
    assert path.read_bytes() == raw, f'{path}; expected unchanged reference baseline'


def freeze_renamed(document: ConfigurationDocument, raw: bytes,
                   reads: list[PrimitiveRead]) -> dict[str, object]:
    """Change one Source-identified CString and its containing section length."""
    import modelprefix
    start = next(i for i, r in enumerate(reads) if r.source_function == 'r_moNodeName_c')
    end = next(i for i in range(start, len(reads)) if reads[i].kind == 'raw')
    name = reads[start:end + 1]
    assert len(name) == 4 and name[-1].kind == 'raw'
    replacement = 'Bessemer Ω'.encode('utf-16-le')
    replacement = struct.pack('<BHB', 255, 65534, len(replacement) // 2) + replacement
    changed = bytearray(raw[:name[0].start] + replacement + raw[name[-1].end:])
    section = next(r for r in reads if r.source_file == 'modelprefix.py' and r.source_function == 'model_prefix')
    assert section.kind == 'i' and section.end - section.start == 4
    struct.pack_into('<i', changed, section.start, struct.unpack_from('<i', raw, section.start)[0] + len(changed) - len(raw))
    root, cursor = modelprefix.read(SelectedConfigurationDocument(document, bytes(changed)),
                                   root_class='moAssembly_c', assembly_resolved_stage='assembly_tail')
    assert cursor.p == len(changed) and root['node']['name']['name'] == 'Bessemer Ω'
    assert root['unit_component']['owner'] is root
    freeze_bytes(ROOT / 'tests/fixtures/assembly-configurations/owned-assembly-name-edited.bin', bytes(changed))
    return dict(bytes=len(changed), map_slots=len(cursor.map), sha256=hashlib.sha256(changed).hexdigest())


def freeze_owned() -> None:
    """Require real root ownership, both complete sections and physical EOF."""
    from swcontainer import Document
    from swarchive import Archive
    import modelprefix
    document = Document((ROOT / 'tests/fixtures/blank-assembly.SLDASM').read_bytes(), table={})
    raw = document.read('Contents/Config-0')
    with observe_primitives(Archive, raw) as reads:
        root, cursor = modelprefix.read(document, root_class='moAssembly_c',
                                        assembly_resolved_stage='assembly_tail')
    assert cursor.p == len(raw) and root['_class'] == 'moAssembly_c'
    assert root['unit_component']['owner'] is root
    assert root['resolved']['prefix_end'] == root['resolved']['end'] == len(raw)
    verify_primitive_coverage(raw, reads)
    facts = dict(bytes=len(raw), version=cursor.version, map_slots=len(cursor.map),
                 lightweight_end=root['lightweight_end'], primitive_reads=len(reads),
                 sha256=hashlib.sha256(raw).hexdigest(), reader_functions_replaced=False,
                 independent_name_edit=freeze_renamed(document, raw, reads))
    freeze_bytes(FEATURE / 'owned-assembly-reference.json', (json.dumps(facts, indent=2) + '\n').encode())
    oracle = ''.join(f'{r.start}\t{r.end}\t{r.kind}\t{r.raw_hex}\t{r.source_file}:{r.source_function}:{r.source_line}\n' for r in reads)
    freeze_bytes(ROOT / 'tests/fixtures/assembly-configurations/owned-assembly.txt', oracle.encode())


def main() -> None:
    """Pin all Python sources and retain exact serializer origins."""
    with tempfile.TemporaryDirectory(prefix='bessemer-owned-assembly-') as temporary:
        pin = Path(temporary)
        hashes = copy_sources(pin)
        sys.meta_path.insert(0, FrozenSourceImports(pin))
        sys.path.insert(0, str(pin))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.FunctionTestCase(freeze_owned))
        assert result.wasSuccessful() and result.testsRun == 1 and not result.skipped
        record_snapshot(pin, hashes, result, FEATURE,
                        ('assemblylightweight', 'assemblyresolved', 'assemblyresolvedtail'),
                        ('swcontainer', 'modelprefix', 'mfcobjects'))
        (FEATURE / 'fixture-reference-snapshot.json').replace(FEATURE / 'owned-assembly-source-snapshot.json')


if __name__ == '__main__':
    main()
