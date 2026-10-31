"""Read-only exact caller import/provider export and graph reader byte bindings (#165)."""
from pathlib import Path
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from graph_evidence import graph_report


def verify_graph_provider(image: pefile.PE, report: dict[str, object]) -> None:
    """Bind complete original reader/helper spans and exported entry; e.g. verify_graph_provider(image, report)."""
    base = image.OPTIONAL_HEADER.ImageBase
    packet = json.loads((RESEARCH / 'cosworks-post-graph-export.json').read_bytes())
    image.parse_data_directories(directories=[0])
    exported = packet['exports'][0]
    assert any(symbol.name == exported['name'].encode() and base + symbol.address == int(exported['address'], 16)
               for symbol in image.DIRECTORY_ENTRY_EXPORT.symbols), exported
    for function in cast(list[dict[str, str]], report['functions']):
        start, end = int(function['address'], 16), int(function['end'], 16)
        actual = digest(image.get_data(start - base, end - start))
        assert actual == function['raw_sha256'], f'{start:x}/{actual}; expected complete original graph function bytes'


def verify_graph_caller(image: pefile.PE) -> None:
    """Bind the exact named provider import at its recorded IAT slot; e.g. verify_graph_caller(image)."""
    imports = json.loads((RESEARCH / 'cosworks-post-parameter-imports.json').read_bytes())
    expected = next(row for row in imports if row['module'] == 'cwPostCommon.dll')
    image.parse_data_directories(directories=[1])
    matches = [symbol for descriptor in image.DIRECTORY_ENTRY_IMPORT
               for symbol in descriptor.imports if descriptor.dll.lower() == b'cwpostcommon.dll'
               and symbol.name == expected['symbol'].encode()]
    assert len(matches) == 1 and matches[0].address == int(expected['iat'], 16), expected


def verify_graph_installed_bindings() -> None:
    """Read both installed images without executing provider code; e.g. verify_graph_installed_bindings()."""
    folder = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation')
    provider, caller = (folder / 'cwPostCommon.dll').read_bytes(), (folder / 'cosworks.dll').read_bytes()
    report = graph_report()
    assert digest(provider) == report['provider_sha256'], digest(provider)
    assert digest(caller) == 'f4cf8adfc0f6e2e36c3a29765d28bd0d073f952e5078c5dcc7cc0d5a0d409c36', digest(caller)
    provider_image, caller_image = pefile.PE(data=provider, fast_load=True), pefile.PE(data=caller, fast_load=True)
    try:
        verify_graph_provider(provider_image, report)
        verify_graph_caller(caller_image)
    finally:
        provider_image.close()
        caller_image.close()
