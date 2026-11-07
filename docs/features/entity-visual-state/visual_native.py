"""Corroborate the retained visual bindings against installed images without loading CAD code (#181)."""
from pathlib import Path
from typing import cast
import hashlib
import json
import sys
from visual_state_evidence import FEATURE
from visual_native_catalog import visual_native_catalog
from visual_native_image import inspect_visual_image


def visual_provider_links(modules: list[dict[str, object]], exports: dict[str, dict[str, int]]) -> list[dict[str, object]]:
    """Bind the renderer's two calls to captured provider entries; e.g. visual_provider_links(modules, exports)."""
    links: list[dict[str, object]] = []
    for module in modules:
        for binding in cast(list[dict[str, str]], module['imports']):
            provider = binding['dll'].casefold()
            if provider not in exports:
                continue
            address = exports[provider][binding['symbol']]
            captured = next(row for row in modules if Path(cast(str, row['module'])).name.casefold() == provider)
            assert any(int(span['start'], 16) == address for span in cast(list[dict[str, str]], captured['intervals'])), binding
            links.append(dict(caller=module['module'], instruction=binding['instruction'], provider=provider,
                              symbol=binding['symbol'], address=hex(address), captured_provider_entry=True))
    assert len(links) == 2, f'{len(links)}; expected both original renderer-to-provider links'
    return links


def verify_visual_installed() -> None:
    """Check all recorded modules, bytes, names and links without executing them; e.g. verify_visual_installed()."""
    modules: list[dict[str, object]] = []
    exports: dict[str, dict[str, int]] = {}
    for module, expected in visual_native_catalog().items():
        raw = Path(module).read_bytes()
        digest = hashlib.sha256(raw).hexdigest()
        assert digest == expected['sha256'], f'{module}/{digest}; expected pinned installed image {expected["sha256"]}'
        observed, names = inspect_visual_image(raw, expected)
        modules.append(dict(module=module, **observed))
        exports[Path(module).name.casefold()] = names
    links = visual_provider_links(modules, exports)
    result = dict(modules=modules, renderer_provider_links=links, target_executed=False, coverage_credit=0,
                  provider='Existing pefile reader; ordinary static observations, not fresh REA Evidence')
    path = FEATURE / 'installed-reference.json'
    raw = (json.dumps(result, indent=2) + '\n').encode()
    if '--write' in sys.argv:
        path.write_bytes(raw)
    assert path.read_bytes() == raw, f'{path}; expected unchanged complete installed visual evidence'
