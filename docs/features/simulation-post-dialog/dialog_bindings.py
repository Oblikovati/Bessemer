"""Read-only complete dialog provider, helper and exact named import/export bindings (#166)."""
from pathlib import Path
from typing import cast
import json
import pefile
from native_carriers import RESEARCH, digest
from dialog_evidence import dialog_reports, dialog_baseline


def dialog_images() -> dict[str, pefile.PE]:
    """Hash all five installed modules before reading native bytes; e.g. dialog_images()."""
    folder = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation')
    reports = dialog_reports()
    color = json.loads((RESEARCH / 'cosworks-post-color-binding.json').read_bytes())
    persist = json.loads((RESEARCH / 'cosworks-persist-exports.json').read_bytes())
    graph = json.loads((RESEARCH / 'cosworks-post-graph-export.json').read_bytes())
    hashes = {'cosworks.dll': reports[5]['caller_sha256'], 'dgxpost.dll': reports[4]['provider_sha256'],
              'dspogl.dll': color['provider_sha256'], 'cosutilities.dll': persist['sha256'], 'cwpostcommon.dll': graph['sha256']}
    images: dict[str, pefile.PE] = {}
    for name, expected in hashes.items():
        raw = (folder / name).read_bytes()
        assert digest(raw) == expected, f'{name}/{digest(raw)}; expected complete module hash {expected}'
        images[name] = pefile.PE(data=raw, fast_load=True)
    return images


def dialog_export(image: pefile.PE, prefix: bytes, address: int) -> bytes:
    """Resolve one exact named exported helper at its documented address; e.g. dialog_export(image, prefix, address)."""
    image.parse_data_directories(directories=[0])
    matches = [symbol for symbol in image.DIRECTORY_ENTRY_EXPORT.symbols
               if symbol.name and symbol.name.startswith(prefix) and image.OPTIONAL_HEADER.ImageBase + symbol.address == address]
    assert len(matches) == 1, (prefix, hex(address), len(matches))
    return cast(bytes, matches[0].name)


def dialog_import(image: pefile.PE, module: bytes, symbol: bytes, iat: int | None = None) -> None:
    """Bind the same exact symbol to the caller and optional recorded IAT; e.g. dialog_import(image, module, symbol)."""
    image.parse_data_directories(directories=[1])
    matches = [item for descriptor in image.DIRECTORY_ENTRY_IMPORT if descriptor.dll.lower() == module.lower()
               for item in descriptor.imports if item.name == symbol]
    assert len(matches) == 1 and (iat is None or matches[0].address == iat), (module, symbol, iat)


def verify_dialog_function_bytes(images: dict[str, pefile.PE]) -> None:
    """Check all original direct/delegated function spans in their explicitly bound modules; e.g. verify_dialog_function_bytes(images)."""
    for report in dialog_reports():
        for function in cast(list[dict[str, str]], report['functions']):
            module = 'dgxpost.dll'
            if 'module' in function:
                module = function['module'].replace('\\', '/').rsplit('/', 1)[1].lower()
                assert module == 'cwpostcommon.dll', module
            if function['address'] == '0x1814aabe0': module = 'cosworks.dll'
            image = images[module]
            start, end = int(function['address'], 16), int(function['end'], 16)
            actual = digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start))
            assert actual == function['raw_sha256'], (module, function['address'], actual)


def verify_dialog_entry_bindings(images: dict[str, pefile.PE]) -> None:
    """Keep modern dialog and historical helper routes distinct; e.g. verify_dialog_entry_bindings(images)."""
    imports = json.loads((RESEARCH / 'cosworks-post-parameter-imports.json').read_bytes())
    modern = next(row for row in imports if row['module'] == 'DGXPOST.dll')
    name = dialog_export(images['dgxpost.dll'], modern['symbol'].encode(), 0x1800953d0)
    dialog_import(images['cosworks.dll'], b'dgxpost.dll', name, int(modern['iat'], 16))
    for label, address, target in (('DspPostInfo', 0x182066110, 0x180044950), ('DspPostStyle', 0x1820660f0, 0x180044ae0)):
        caller = images['cosworks.dll']
        raw = (label + '\0').encode('utf-16le')
        assert caller.get_data(address - caller.OPTIONAL_HEADER.ImageBase, len(raw)) == raw, label
        symbol = dialog_export(images['dgxpost.dll'], b'?ReadFromStream@C' + label.encode() + b'@@', target)
        dialog_import(caller, b'dgxpost.dll', symbol)


def verify_dialog_delegates(images: dict[str, pefile.PE]) -> None:
    """Bind color, reference, shared text and equation providers without executing them; e.g. verify_dialog_delegates(images)."""
    color = json.loads((RESEARCH / 'cosworks-post-color-binding.json').read_bytes())
    symbol = dialog_export(images['dspogl.dll'], color['exports'][0]['name'].encode(), 0x180003dd0)
    dialog_import(images['dgxpost.dll'], b'dspogl.dll', symbol, int(color['imports'][0]['iat'], 16))
    symbol = dialog_export(images['cosutilities.dll'], b'?Serialization@CSWPersistReferenceContainer@@', 0x180079a20)
    dialog_import(images['dgxpost.dll'], b'cosutilities.dll', symbol)
    symbol = dialog_export(images['cwpostcommon.dll'], b'?ReadFromStream@CcwEquationPlotParam@@', 0x180019320)
    dialog_import(images['dgxpost.dll'], b'cwpostcommon.dll', symbol, 0x1802cc370)
    image = images['cwpostcommon.dll']
    image.parse_data_directories(directories=[0])
    names = [item.name for item in image.DIRECTORY_ENTRY_EXPORT.symbols if image.OPTIONAL_HEADER.ImageBase + item.address == 0x180028cf0]
    assert len(names) == 1 and b'CStringReadFromStream' in names[0], names
    dialog_import(images['dgxpost.dll'], b'cwpostcommon.dll', names[0])


def verify_dialog_installed_bindings() -> None:
    """Require complete provider/caller hashes and every serializer binding; e.g. verify_dialog_installed_bindings()."""
    images = dialog_images()
    try:
        verify_dialog_function_bytes(images)
        verify_dialog_entry_bindings(images)
        verify_dialog_delegates(images)
        prior = json.loads((RESEARCH / 'cosworks-reference-container-native-evidence.json').read_bytes())
        image = images['cosutilities.dll']
        for function in prior['functions']:
            start, end = int(function['address'], 16), int(function['end'], 16)
            assert digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)) == function['raw_sha256']
        dialog_baseline('installed-reference.json', dict(images={name: digest(image.__data__) for name, image in images.items()}, target_executed=False))
    finally:
        for image in images.values(): image.close()
