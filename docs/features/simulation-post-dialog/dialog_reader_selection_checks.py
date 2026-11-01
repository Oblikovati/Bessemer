"""Owned pre-refactor callback routing across all old modes and ordered pairs (#166)."""
from dataclasses import dataclass
from typing import cast
import json
from dialog_evidence import FEATURE
from dialog_reader_selection import select_dialog_snapshot_readers


@dataclass(frozen=True)
class OwnedDialogReaderProvider:
    """Replay a captured provider result without Source imports; e.g. OwnedDialogReaderProvider(('cfb',))()."""
    readers: tuple[str, ...]

    def __call__(self) -> tuple[str, ...]:
        """Return the immutable captured callback input; e.g. provider()."""
        return self.readers


def verify_dialog_reader_selection() -> None:
    """Keep all owned routing outputs unchanged as future providers gain declared dependencies; e.g. verify_dialog_reader_selection()."""
    path = FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog/reader-selection.json'
    packet = cast(dict[str, object], json.loads(path.read_bytes()))
    assert packet['schema_version'] == 1 and len(cast(list[object], packet['modes'])) == 37
    cases = cast(list[dict[str, object]], packet['cases'])
    selections = cast(list[list[str]], packet['selections'])
    providers = cast(list[dict[str, int]], packet['provider_sets'])
    base = tuple(cast(list[str], packet['base_readers']))
    assert len(cases) == 2776 and len(selections) == 27 and len(providers) == 2
    for case in cases:
        check_dialog_reader_case(base, selections, providers, case)


def check_dialog_reader_case(base: tuple[str, ...], selections: list[list[str]], providers: list[dict[str, int]], case: dict[str, object]) -> None:
    """Replay captured callback inputs rather than inferring them from current provider code; e.g. check_dialog_reader_case(base, selections, providers, case)."""
    arguments = cast(list[str], case['arguments'])
    selected = cast(int, case['selection'])
    profile = cast(int, case['providers'])
    assert 0 <= selected < len(selections) and 0 <= profile < len(providers), (selected, profile, 'expected owned selection/provider indices')
    callbacks = {key: OwnedDialogReaderProvider(tuple(selections[index])) for key, index in providers[profile].items()}
    actual = select_dialog_snapshot_readers(['freeze_post_dialog.py', *arguments], base, callbacks)
    assert actual == tuple(selections[selected]), (arguments, actual, selections[selected])
