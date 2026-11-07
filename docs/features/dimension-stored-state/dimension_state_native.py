"""Corroborate recorded dimension bindings using read-only PE bytes (#179)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import struct
import sys
import pefile
from dimension_state_evidence import FEATURE, SOURCE


def verify_dimension_interval(image: pefile.PE, row: dict[str, str], exports: dict[str, int]) -> dict[str, object]:
    """Check complete recorded bytes and their named export entry; e.g. verify_dimension_interval(image, row, exports)."""
    start, end = int(row['start'], 16), int(row['end'], 16)
    expected = bytes.fromhex(row['bytes'])
    observed = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
    assert len(expected) == end - start and observed == expected, row['start']
    actual = hashlib.sha256(observed).hexdigest()
    assert actual == row['sha256'], f'{start:x}/{actual}; expected interval {row["sha256"]}'
    names = sorted(name for name, address in exports.items() if address == start)
    internal = start in (0x4c828a10, 0x4c829e20)
    assert bool(names) != internal, f'{start:x}/{names}; expected seven named entries and two internal constructor/copy ranges'
    assert 'export' not in row or row['export'] in names, row['start']
    return dict(start=row['start'], end=row['end'], bytes=len(observed), sha256=actual, exports=names)


def dimension_export_name(image: pefile.PE, symbol: pefile.ExportData) -> str:
    """Read the full bounded PE name instead of the parser's 512-byte preview; e.g. dimension_export_name(image, symbol)."""
    offset = symbol.name_offset
    assert isinstance(offset, int), f'{offset}; expected a physical named-export file offset'
    raw = image.get_string_at_rva(image.get_rva_from_offset(offset), 8192)
    assert 0 < len(raw) < 8192, f'{offset}/{len(raw)}; expected a terminated export name below8192 bytes'
    return raw.decode('ascii')


def verify_dimension_exports(image: pefile.PE) -> dict[str, int]:
    """Require the entire declared table, avoiding pefile's default truncation; e.g. verify_dimension_exports(image)."""
    image.parse_data_directories(directories=[0])
    exports = {dimension_export_name(image, row): image.OPTIONAL_HEADER.ImageBase + row.address
               for row in image.DIRECTORY_ENTRY_EXPORT.symbols if row.name}
    recorded = json.loads((SOURCE / 'research/sldmodu-complete-export-index.json').read_bytes())
    expected = {row['name']: int(row['address'], 16) for row in recorded['exports']}
    declared = image.DIRECTORY_ENTRY_EXPORT.struct.NumberOfNames
    assert len(exports) == declared == recorded['named_exports'] == 45766, declared
    assert exports == expected, f'{len(exports)}; expected the complete original named-export index'
    return exports


def capture_dimension_native(image: pefile.PE, reports: list[dict[str, object]], digest: str) -> None:
    """Keep typed resolver provenance beside full interval checks; e.g. capture_dimension_native(image, reports, digest)."""
    exports = verify_dimension_exports(image)
    intervals = [verify_dimension_interval(image, row, exports)
                 for report in reports for row in cast(list[dict[str, str]], report['functions'])]
    resolver = cast(dict[str, str], reports[1]['resolver_binding'])
    slot = int(resolver['vftable'], 16) + int(resolver['slot'], 16)
    observed = struct.unpack('<Q', image.get_data(slot - image.OPTIONAL_HEADER.ImageBase, 8))[0]
    assert observed == int(resolver['target'], 16), f'{observed:x}; expected typed resolver {resolver}'
    result = dict(module_sha256=digest, named_exports=len(exports), all_declared_export_names_parsed=True,
        intervals=intervals, recorded_bytes=sum(cast(int, row['bytes']) for row in intervals),
        resolver_binding=resolver, resolver_vftable_target_matches=True, target_executed=False,
        coverage_credit=0, provider='Existing pefile reader; ordinary static observation, not REA Evidence')
    path = FEATURE / 'installed-reference.json'
    raw = (json.dumps(result, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected complete unchanged installed-image evidence'


def verify_dimension_installed() -> None:
    """Read the pinned image without executing CAD code; e.g. verify_dimension_installed()."""
    names = ('part-dimension-base-mask-native-evidence.json', 'part-dimension-redundant-id-native-evidence.json')
    reports = [json.loads((FEATURE / 'reports' / name).read_bytes()) for name in names]
    raw = Path(reports[0]['module']).read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    assert all(report['sha256'] == digest for report in reports), f'{digest}; expected both native module pins'
    # #179: the default 8,192-symbol cap truncates this pinned 45,766-name table.
    image = pefile.PE(data=raw, fast_load=True, max_symbol_exports=50000)
    try:
        capture_dimension_native(image, reports, digest)
    finally:
        image.close()
