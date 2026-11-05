"""Read-only complete installed plot-loader and bounded wide-helper bindings (#174)."""
from pathlib import Path
from typing import cast
import json
import os
import subprocess
import pefile
from native_carriers import RESEARCH, digest
from dialog_bindings import dialog_export, dialog_import
from style_preferences_disassembly import parsed_style_instructions, canonical_style_address_operands, StylePreferenceInstruction
from legacy_plot_evidence import FEATURE, legacy_plot_reports, legacy_plot_baseline
from legend_body_evidence import legend_body_capture

SIMULATION = Path('C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/Simulation')


def legacy_plot_methods() -> list[dict[str, object]]:
    """Join all complete original probe intervals to their image/hash identities; e.g. legacy_plot_methods()."""
    original, wide = legacy_plot_reports()
    probes = [row for name in ('cosworks-legacy-plot-loader-probe.json', 'cosworks-legacy-plot-record-probe.json')
        for row in json.loads((RESEARCH / name).read_bytes())]
    functions = cast(list[dict[str, object]], original['functions'])
    methods: list[dict[str, object]] = []
    for function, probe in zip(functions, probes, strict=True):
        assert function['address'] == probe['address'] and function['end'] == probe['end'], (function, probe)
        methods.append(function | dict(instructions=probe['instructions'], module='cosworks.dll'))
    methods.append(cast(dict[str, object], wide['function']) | dict(module='cwCommFuncs.dll'))
    return methods


def disassemble_legacy_plot_method(module: Path, start: int, end: int) -> list[StylePreferenceInstruction]:
    """Decode only a selected installed interval with the existing tool; e.g. disassemble_legacy_plot_method(module, start, end)."""
    executable = Path(os.environ.get('BESSEMER_OBJDUMP', 'C:/msys64/mingw64/bin/objdump.exe'))
    assert executable.is_file(), f'{executable}; expected existing GNU objdump or BESSEMER_OBJDUMP'
    arguments = [str(executable), '-d', '-w', '-Mintel', '--insn-width=16',
        f'--start-address={start:#x}', f'--stop-address={end:#x}', str(module)]
    output = subprocess.run(arguments, check=True, capture_output=True, text=True, encoding='utf-8').stdout
    return parsed_style_instructions(output)


def verify_legacy_plot_method(image: pefile.PE, method: dict[str, object]) -> int:
    """Match every original instruction and all contiguous original bytes; e.g. verify_legacy_plot_method(image, method)."""
    start, end = (int(cast(str, method[key]), 16) for key in ('address', 'end'))
    raw = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
    if 'raw_sha256' in method: assert digest(raw) == method['raw_sha256'], method['address']
    decoded = disassemble_legacy_plot_method(SIMULATION / cast(str, method['module']), start, end)
    normalize = canonical_style_address_operands
    actual = [(item.address, item.mnemonic, normalize(item.mnemonic, ','.join(item.operands))) for item in decoded]
    parts = [line.split(' ', 2) for line in cast(list[str], method['instructions'])]
    expected = [(int(row[0], 16), row[1], normalize(row[1], row[2] if len(row) == 3 else '')) for row in parts]
    assert actual == expected, [(left, right) for left, right in zip(actual, expected) if left != right]
    assert b''.join(item.raw for item in decoded) == raw, f'{method["address"]}; expected full contiguous interval'
    assert decoded[0].address == start and decoded[-1].address + len(decoded[-1].raw) == end
    return len(decoded)


def verify_legacy_wide_import(caller: pefile.PE, provider: pefile.PE) -> None:
    """Bind the conversion export and keep the later ordinal call unresolved; e.g. verify_legacy_wide_import(caller, provider)."""
    symbol = dialog_export(provider, b'?LPUSHORT2STRING@@', 0x180005bf0)
    dialog_import(caller, b'cwCommFuncs.dll', symbol, 0x181f33c68)
    provider.parse_data_directories(directories=[1])
    trim = [item for entry in provider.DIRECTORY_ENTRY_IMPORT if entry.dll.lower() == b'mfc140u.dll'
        for item in entry.imports if item.address == 0x180010588]
    assert len(trim) == 1, f'{len(trim)}; expected one MFC import at 0x180010588'
    assert trim[0].name is None and trim[0].ordinal == 14039, (trim[0].name, trim[0].ordinal)
    legacy_plot_baseline('wide-import-reference.json', dict(symbol=symbol.decode(), iat=hex(0x181f33c68),
        later_mfc_iat=hex(0x180010588), later_mfc_ordinal=14039, rea_display_name='TrimRight',
        actual_runtime_mfc_provider_resolved=False, raw_prefix_is_final_native_string=False, target_executed=False))


def verify_legacy_plot_installed() -> None:
    """Require both complete image hashes before interval and symbol checks; e.g. verify_legacy_plot_installed()."""
    original, wide = legacy_plot_reports()
    expected = {'cosworks.dll': original['module_sha256'], 'cwCommFuncs.dll': wide['provider_sha256']}
    images: dict[str, pefile.PE] = {}
    try:
        for name, digest_expected in expected.items():
            raw = (SIMULATION / name).read_bytes()
            assert digest(raw) == digest_expected, f'{name}; expected {digest_expected}'
            images[name] = pefile.PE(data=raw, fast_load=True)
        methods = legacy_plot_methods()
        counts = [verify_legacy_plot_method(images[cast(str, method['module'])], method) for method in methods]
        assert counts == [419, 1371, 62, 46], counts
        verify_legacy_wide_import(images['cosworks.dll'], images['cwCommFuncs.dll'])
        legacy_plot_baseline('installed-reference.json', dict(images=expected, instruction_counts=counts,
            instructions=sum(counts), target_executed=False, native_writer_equivalence=False))
    finally:
        for image in images.values(): image.close()


def verify_legacy_wide_rea() -> None:
    """Check all46 fresh helper locations and preserve both earlier capture schemas; e.g. verify_legacy_wide_rea()."""
    _, wide = legacy_plot_reports()
    method = cast(dict[str, object], wide['function'])
    capture = legend_body_capture(FEATURE / 'wide-helper-rea.json', method | dict(start=method['address']),
        cast(str, wide['provider_sha256']))
    assert capture['instructions'] == 46, capture
    legacy_plot_baseline('wide-rea-reference.json', capture)
    previous = FEATURE.parent / 'simulation-post-dialog'
    probes = json.loads((RESEARCH / 'cosworks-post-style-compare-probe.json').read_bytes())
    report = json.loads((RESEARCH / 'cosworks-post-style-compare-native-evidence.json').read_bytes())
    controls = [legend_body_capture(previous / f'style-comparison-rea-{name}.json', probe | dict(start=probe['address']),
        report['module_sha256']) for name, probe in zip(('diff', 'copy'), probes, strict=True)]
    assert [control['instructions'] for control in controls] == [107, 38], controls
