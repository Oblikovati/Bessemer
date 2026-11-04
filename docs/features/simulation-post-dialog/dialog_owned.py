"""Independent complete parent/standalone literals across all reachable wire gates (#166)."""
from collections.abc import Iterator
from dataclasses import asdict
from itertools import product
import importlib
from typing import cast
from native_carriers import digest
from dialog_evidence import dialog_baseline
from dialog_literal_values import DialogBlocks, DialogLiteralProfile
from dialog_initial_literals import initial_dialog_blocks, postinfo_blocks
from dialog_style_literals import style_blocks
from dialog_plane_literals import dialog_plane_blocks
from dialog_tail_literals import dialog_modern_tail, dialog_common_final

DIALOG_GATES = (700335, 800004, 800007, 800008, 800025, 800026, 800046, 800076,
    810009, 810013, 810015, 810019, 810020, 820006, 820008, 820021, 820026,
    830008, 830010, 830012, 830017, 830021, 830022, 830024, 830026, 830030,
    840002, 840008, 840010, 850003, 850010, 850017, 850021, 850022, 850029,
    860007, 870007, 870009, 870015, 880002, 880003, 880005, 880017, 880018,
    890004, 890008, 890014, 890021, 890103, 890107, 890109, 890118, 890206,
    890212, 890307, 890311, 890319, 890504, 890507, 890508, 890510, 890511, 890513, 890611, 890702)
POPULATED_GATES = (810009, 810013, 810015, 820026, 840002, 850017, 850022,
    860007, 870007, 870009, 890021, 890109, 890212, 890611)
STYLE_VERSIONS = (0, 700005, 700006, 700329, 700330, 810008, 810009, 810018, 810019,
    820005, 820006, 840007, 840008, 840009, 840010, 890102, 890103, 890701, 890702, 2147483647)
INFO_VERSIONS = (0, 700001, 700002, 700003, 700334, 700335, 2147483647)


def dialog_body_blocks(version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Assemble a complete parent without deriving any endpoint from parser output; e.g. dialog_body_blocks(890620, profile)."""
    blocks = initial_dialog_blocks(profile) + postinfo_blocks(version, profile) + style_blocks(version, profile)
    return blocks + dialog_plane_blocks(version, profile) + dialog_modern_tail(version, profile) + dialog_common_final(version, profile)


def dialog_owned_blocks(family: int, version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Select an explicit standalone or complete-parent grammar before encoding; e.g. dialog_owned_blocks(1, 700002, profile)."""
    builders = (dialog_body_blocks, postinfo_blocks, style_blocks)
    assert type(family) is int and 0 <= family < 3, f'{family}; expected dialog/PostInfo/PostStyle family 0..2'
    return builders[family](version, profile)


def dialog_parent_profiles() -> Iterator[tuple[int, DialogLiteralProfile]]:
    """Cover every adjacent gate plus populated delegated branch boundaries; e.g. list(dialog_parent_profiles())."""
    versions = sorted({value for gate in DIALOG_GATES for value in (gate - 1, gate, gate + 1) if value >= 700335} | {2147483647})
    for version in versions: yield version, DialogLiteralProfile()
    populated = sorted({value for gate in POPULATED_GATES for value in (gate - 1, gate, gate + 1)})
    for index, version in enumerate(populated):
        yield version, DialogLiteralProfile(rotation=index % 5, populated=True, empty_texts=False, equation=True)
    for rotation, empty, equation in product(range(5), (False, True), (False, True)):
        yield 891107, DialogLiteralProfile(rotation=rotation, populated=True, duplicate_records=True, empty_texts=empty, equation=equation)


def dialog_owned_cases() -> Iterator[tuple[int, int, DialogLiteralProfile, bytes]]:
    """Keep complete parent and standalone helper contexts in deterministic order; e.g. list(dialog_owned_cases())."""
    for version, profile in dialog_parent_profiles():
        yield 0, version, profile, b''.join(fragment for _, fragment in dialog_body_blocks(version, profile))
    for family, versions in ((1, INFO_VERSIONS), (2, STYLE_VERSIONS)):
        for version, rotation, empty in product(versions, range(5), (False, True)):
            profile = DialogLiteralProfile(rotation=rotation, empty_texts=empty)
            yield family, version, profile, b''.join(fragment for _, fragment in dialog_owned_blocks(family, version, profile))


def dialog_literal_spans(blocks: DialogBlocks) -> list[dict[str, object]]:
    """Derive every physical span before invoking a reference reader; e.g. dialog_literal_spans(blocks)."""
    spans: list[dict[str, object]] = []
    offset = 0
    for label, raw in blocks:
        spans.append(dict(label=label, offset=offset, end=offset + len(raw), raw_hex=raw.hex()))
        offset += len(raw)
    return spans


def dialog_owned_context(family: int, version: int, profile: DialogLiteralProfile, raw: bytes) -> dict[str, object]:
    """Require exact complete framing and every independent literal span; e.g. dialog_owned_context(0, 890620, profile, raw)."""
    module = ('cosworks_post_dialog_layout', 'cosworks_post_info_layout', 'cosworks_post_style_layout')[family]
    parsed = importlib.import_module(module).parse(raw, version)
    assert parsed['success'] and parsed['end'] == len(raw), (family, version, asdict(profile), parsed)
    expected = dialog_literal_spans(dialog_owned_blocks(family, version, profile))
    assert parsed['spans'] == expected, (family, version, asdict(profile), parsed['spans'], expected)
    key = 'consumed_prefix_sha256' if family == 0 else 'reconstruction_sha256'
    assert parsed[key] == digest(raw), (family, version)
    return dict(family=family, version=version, profile=asdict(profile), bytes=len(raw), sha256=digest(raw), result=parsed)


def verify_dialog_owned() -> None:
    """Freeze complete independent owned outcomes for every reached gate and delegated branch; e.g. verify_dialog_owned()."""
    rows = [dialog_owned_context(*case) for case in dialog_owned_cases()]
    counts = [sum(row['family'] == family for row in rows) for family in range(3)]
    assert (len(rows), counts) == (495, [225, 70, 200]), (len(rows), counts)
    assert sum(cast(int, row['bytes']) for row in rows) == 1462773
    dialog_baseline('owned-reference.json', dict(contexts=len(rows), family_contexts=counts,
        logical_bytes=sum(cast(int, row['bytes']) for row in rows), rows=rows))
