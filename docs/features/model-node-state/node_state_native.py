"""Read-only PE corroboration of the recorded node and parameter bindings (#178)."""
from pathlib import Path
from typing import cast
import hashlib
import importlib
import struct
from operator import itemgetter
import pefile
from node_state_evidence import node_state_reports, node_state_baseline


def recorded_node_intervals(reports: list[dict[str, object]]) -> list[dict[str, object]]:
    """Retain every reported range, including complete multi-return captures; e.g. recorded_node_intervals(reports)."""
    unique: dict[str, dict[str, object]] = {}
    for report in reports:
        for row in cast(list[dict[str, object]], report.get('functions', [])):
            prior = unique.get(cast(str, row['start']))
            assert prior is None or prior['bytes'] == row['bytes'], row['start']
            unique[cast(str, row['start'])] = row
    getter = cast(dict[str, object], next(report for report in reports if report['evidence'] == 'C-2191')['getter'])
    exported = cast(dict[str, str], getter['export'])
    start, raw = int(exported['address'], 16), bytes.fromhex(cast(str, getter['bytes']))
    unique[hex(start)] = dict(start=hex(start), end=hex(start + len(raw)), bytes=raw.hex(), export=exported['name'])
    return sorted(unique.values(), key=itemgetter('start'))


def verify_recorded_node_interval(image: pefile.PE, row: dict[str, object], exports: dict[str, int]) -> dict[str, object]:
    """Match complete interval bytes and the exact exported entry; e.g. verify_recorded_node_interval(image, row, exports)."""
    start, end = int(cast(str, row['start']), 16), int(cast(str, row['end']), 16)
    expected = bytes.fromhex(cast(str, row['bytes']))
    observed = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
    assert len(expected) == end - start and observed == expected, row['start']
    name = cast(str, row['export'])
    assert exports[name] == start, f'{name}/{exports.get(name)}; expected exact export {start:x}'
    return dict(start=hex(start), end=hex(end), bytes=len(observed), export=name,
                sha256=hashlib.sha256(observed).hexdigest())


def verify_parameter_node_bases(image: pefile.PE, report: dict[str, object]) -> None:
    """Reuse the frozen RTTI locator and retain exact zero-offset parameter bases; e.g. verify_parameter_node_bases(image, report)."""
    actual = importlib.import_module('rtti').locate(report['module'], ['moParameter_c'])
    assert actual == report['rtti'], f'{actual}; expected original primary parameter RTTI'
    table = next(row for row in actual['tables'] if row['object_offset'] == 0)
    locator = int(table['locator'], 16) - image.OPTIONAL_HEADER.ImageBase
    hierarchy = struct.unpack('<I', image.get_data(locator + 16, 4))[0]
    header = struct.unpack('<4I', image.get_data(hierarchy, 16))
    bases: list[dict[str, object]] = []
    for index in range(header[2]):
        address = struct.unpack('<I', image.get_data(header[3] + 4 * index, 4))[0]
        fields = struct.unpack('<IIiiiII', image.get_data(address, 28))
        bases.append(dict(name=image.get_string_at_rva(fields[0] + 16).decode('ascii'),
                          member_displacement=fields[2], pointer_displacement=fields[3]))
    assert bases == report['bases'], f'{bases}; expected exact nonvirtual zero-offset parameter hierarchy'


def persist_node_state_installed_reference(module_sha: str, intervals: list[dict[str, object]], named_exports: int) -> None:
    """Record bounded static checks with explicit execution and coverage exclusions; e.g. persist_node_state_installed_reference(sha, rows, count)."""
    node_state_baseline('installed-reference.json', dict(module_sha256=module_sha, intervals=intervals,
        unique_intervals=len(intervals), recorded_bytes=sum(cast(int, row['bytes']) for row in intervals),
        parameter_primary_rtti_and_bases_match=True, target_executed=False, coverage_credit=0,
        provider='Existing pefile reader and frozen source RTTI locator; not REA Evidence',
        named_exports=named_exports, all_declared_export_names_parsed=True))


def verify_node_state_installed() -> None:
    """Bind the installed image, exports, complete bytes and parameter RTTI without execution; e.g. verify_node_state_installed()."""
    reports = node_state_reports()
    native = next(report for report in reports if report['evidence'] == 'C-1918')
    module = Path(cast(str, native['module']))
    raw = module.read_bytes()
    actual = hashlib.sha256(raw).hexdigest()
    assert all(report['sha256'] == actual for report in reports if 'module' in report), f'{module}/{actual}; expected all original native module hashes'
    # #178: pefile's default export limit truncates this pinned 45,766-name table.
    image = pefile.PE(data=raw, fast_load=True, max_symbol_exports=50000)
    try:
        image.parse_data_directories(directories=[0])
        exports = {row.name.decode(): image.OPTIONAL_HEADER.ImageBase + row.address
                   for row in image.DIRECTORY_ENTRY_EXPORT.symbols if row.name}
        declared = image.DIRECTORY_ENTRY_EXPORT.struct.NumberOfNames
        assert len(exports) == declared, f'{len(exports)}; expected all {declared} declared named exports'
        intervals = [verify_recorded_node_interval(image, row, exports) for row in recorded_node_intervals(reports)]
        verify_parameter_node_bases(image, next(report for report in reports if report['evidence'] == 'C-2191'))
        persist_node_state_installed_reference(actual, intervals, len(exports))
    finally:
        image.close()
