"""Complete independent Results literal reconstruction and current-writer controls (#164)."""
from typing import cast
from itertools import product
from native_carriers import digest
from results_evidence import results_baseline
from results_literals import (ResultsBlocks, results_root_blocks, results_category_blocks,
                              results_plot_blocks, results_writer_blocks)
from results_native import actual_results

RESULTS_VERSIONS = (0, 800037, 800038, 800043, 800044, 800045, 810009, 810010,
                   810011, 830034, 830035, 830036, 840012, 840013, 840014,
                   890401, 890402, 890403, 890604, 890605, 890606, 2147483647)


def results_literal_spans(blocks: ResultsBlocks) -> list[dict[str, object]]:
    """Compute all physical boundaries before calling the reference; e.g. results_literal_spans([('word', b'1234')])."""
    position = 0
    spans: list[dict[str, object]] = []
    for label, fragment in blocks:
        spans.append(dict(label=label, offset=position, end=position + len(fragment), raw_hex=fragment.hex()))
        position += len(fragment)
    return spans


def results_owned_context(level: int, version: int, rotation: int, null_only: bool) -> dict[str, object]:
    """Compare complete independently framed images and all reference spans; e.g. results_owned_context(0, 840013, 1, False)."""
    builders = (results_root_blocks, results_category_blocks, results_plot_blocks)
    blocks = builders[level](version, rotation, null_only)
    raw = b''.join(fragment for _, fragment in blocks)
    actual = actual_results(raw, version, level)
    assert actual['success'] and actual['end'] == len(raw), (level, version, actual)
    assert actual['spans'] == results_literal_spans(blocks), (level, version, actual)
    assert actual['reconstruction_sha256'] == digest(raw), (level, version)
    return dict(level=level, version=version, rotation=rotation, null_only=null_only,
                bytes=len(raw), sha256=digest(raw), result=actual)


def verify_results_owned() -> None:
    """Exercise every threshold, scalar extreme and raw text profile; e.g. verify_results_owned()."""
    rows = [results_owned_context(level, version, rotation, null_only)
            for level in range(3) for version in RESULTS_VERSIONS
            for rotation in range(5) for null_only in (False, True)]
    assert len(rows) == 660, len(rows)
    results_baseline('owned-reference.json', dict(contexts=rows, count=len(rows),
                     logical_bytes=sum(cast(int, row['bytes']) for row in rows)))


def verify_results_writer_controls() -> None:
    """Check only documented current writers against compatible readers; e.g. verify_results_writer_controls()."""
    for level in (0, 1):
        compatible = 890605 if level == 0 else 810010
        for rotation, null_only in product(range(5), (False, True)):
            blocks = results_writer_blocks(level, rotation, null_only)
            raw = b''.join(fragment for _, fragment in blocks)
            assert actual_results(raw, compatible, level)['success'], (level, rotation)
            assert not actual_results(raw, 0, level)['success'], (level, rotation)
    for invalid in (-1, 2, 3, True):
        try: results_writer_blocks(invalid, 0)
        except ValueError: continue
        raise AssertionError(f'{invalid}; expected rejection outside documented writer levels')
