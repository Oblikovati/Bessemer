"""Compare shared codec extractions with the frozen candidate's display code."""
from pathlib import Path
import hashlib
import json
import re
import subprocess

ROOT = Path(__file__).resolve().parents[3]
BASE = '8ad332a57da6bd727dbe7ac36b51b341fed22865'


def original(name: str) -> str:
    return subprocess.check_output(
        ['git', 'show', f'{BASE}:src/display_lists/{name}.rs'], cwd=ROOT,
        text=True, encoding='utf-8')


def normalized_light(source: str, shared: bool) -> str:
    marker = 'pub(crate) fn ambient' if shared else 'pub(super) fn ambient'
    source = source[source.index(marker):]
    if not shared:
        start = source.index('pub(super) fn user_properties')
        end = source.index('fn current_light_base', start)
        source = source[:start]+source[end:]
    source = source.replace('pub(super)', 'pub(crate)')
    source = source.replace("&mut DisplayDecoder<'_>", '&mut impl MaterialRead')
    source = source.replace('archive.context.version.value()', 'archive.version()')
    source = source.replace('super::modern_scene::double_vector', 'double_vector')
    source = re.sub(r',\s*branch: LightPayloadBranch', '', source)
    for function in ('ambient', 'light_names', 'ambient_rendering'):
        source = source.replace(function+'(archive, branch)', function+'(archive)')
    source = source.replace('(archive.version() > 0x453 && branch == LightPayloadBranch::OldDisplay)', 'archive.version() > 0x453')
    source = source.replace(' && branch == LightPayloadBranch::OldDisplay', '')
    source = re.sub(r'\s+', '', source)
    return source.replace(',)', ')')


def verify() -> None:
    old_light = original('modern_lights')
    shared_light = (ROOT/'src/archive_codec/light_payload.rs').read_text()
    if normalized_light(old_light, False) != normalized_light(shared_light, True):
        raise ValueError('OldDisplay light body changed; expected adapter-only extraction')
    for name in ('transform_codec', 'transform_types'):
        shared = (ROOT/f'src/archive_codec/{name}.rs').read_text()
        if original(name).replace('pub(super)', 'pub(crate)') != shared:
            raise ValueError(f'{name} changed; expected visibility-only extraction')
    report = {'base': BASE, 'light_old_display_equivalent': True,
              'transform_visibility_only': True,
              'light_sha256': hashlib.sha256(shared_light.encode()).hexdigest()}
    Path(__file__).with_name('shared-codec-verification.json').write_text(json.dumps(report, indent=2)+'\n')
    print(report)


if __name__ == '__main__':
    verify()
