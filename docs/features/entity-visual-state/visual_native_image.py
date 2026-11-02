"""Read-only PE checks for complete visual code captures, exports, imports and constants (#181)."""
from typing import cast
import hashlib
import struct
import pefile
from dimension_state_native import dimension_export_name
from visual_native_catalog import VisualImageInputs


def visual_export_map(image: pefile.PE) -> dict[str, int]:
    """Read every full export name without the parser's preview/count truncation; e.g. visual_export_map(image)."""
    image.parse_data_directories(directories=[0, 1])
    exports = {dimension_export_name(image, row): image.OPTIONAL_HEADER.ImageBase + row.address
               for row in image.DIRECTORY_ENTRY_EXPORT.symbols if row.name}
    declared = image.DIRECTORY_ENTRY_EXPORT.struct.NumberOfNames
    assert len(exports) == declared, f'{len(exports)}; expected all{declared} declared full export names'
    return exports


def visual_interval_match(image: pefile.PE, row: dict[str, str], exports: dict[str, int]) -> dict[str, object]:
    """Check the whole reported interval rather than one convenient return path; e.g. visual_interval_match(image, row, exports)."""
    start, end = int(row['start'], 16), int(row['end'], 16)
    expected = bytes.fromhex(row['bytes'])
    observed = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
    assert len(expected) == end - start and observed == expected, f'{row["evidence"]}/{start:x}; expected the complete original interval'
    digest = hashlib.sha256(observed).hexdigest()
    assert digest == row['sha256'], f'{start:x}/{digest}; expected interval hash {row["sha256"]}'
    return dict(evidence=row['evidence'], start=row['start'], end=row['end'], bytes=len(observed), sha256=digest,
                exports=sorted(name for name, address in exports.items() if address == start))


def visual_import_match(image: pefile.PE, row: dict[str, str]) -> dict[str, str]:
    """Resolve the recorded RIP-relative call through its actual IAT symbol; e.g. visual_import_match(image, row)."""
    address = int(row['instruction'], 16)
    raw = image.get_data(address - image.OPTIONAL_HEADER.ImageBase, 6)
    assert raw[:2] in (b'\xff\x15', b'\xff\x25'), f'{address:x}/{raw.hex()}; expected an indirect call or jump through the IAT'
    target = address + 6 + struct.unpack('<i', raw[2:])[0]
    matches = [(descriptor.dll.decode(), symbol.name.decode()) for descriptor in image.DIRECTORY_ENTRY_IMPORT
               for symbol in descriptor.imports if symbol.address == target and symbol.name]
    assert len(matches) == 1 and matches[0][1] == row['name'], f'{address:x}/{matches}; expected exact import {row["name"]}'
    if 'iat' in row:
        assert target == int(row['iat'], 16) and matches[0][0].casefold() == row['dll'].casefold(), row
    return dict(instruction=hex(address), iat=hex(target), dll=matches[0][0], symbol=matches[0][1])


def visual_normalizer_match(image: pefile.PE, row: dict[str, object]) -> dict[str, object]:
    """Keep the exact native normalization constant without applying it to stored RGB; e.g. visual_normalizer_match(image, row)."""
    address = int(cast(str, row['address']), 16)
    raw = image.get_data(address - image.OPTIONAL_HEADER.ImageBase, 4)
    assert raw.hex() == row['bytes'], f'{address:x}/{raw.hex()}; expected raw normalizer {row["bytes"]}'
    value = struct.unpack('<f', raw)[0]
    assert value == row['value'], f'{address:x}/{value}; expected original native float {row["value"]}'
    return dict(address=hex(address), bytes=raw.hex(), value=value)


def inspect_visual_image(raw: bytes, expected: VisualImageInputs) -> tuple[dict[str, object], dict[str, int]]:
    """Close each parsed image even on failure and publish no native execution; e.g. inspect_visual_image(raw, expected)."""
    image = pefile.PE(data=raw, fast_load=True, max_symbol_exports=200000)
    try:
        exports = visual_export_map(image)
        for name, address in expected['exports'].items():
            assert exports.get(name) == address, f'{name}/{exports.get(name)}; expected named export {address:x}'
        spans = [visual_interval_match(image, row, exports) for row in expected['spans']]
        imports = [visual_import_match(image, row) for row in expected['imports']]
        constants = [visual_normalizer_match(image, row) for row in expected['normalizers']]
        return dict(sha256=expected['sha256'], named_exports=len(exports), all_declared_export_names_parsed=True,
                    named_bindings_checked=len(expected['exports']), intervals=spans, imports=imports, normalizers=constants), exports
    finally:
        image.close()
