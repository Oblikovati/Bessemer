"""Reuse owned complete C857 wire fixtures with independent C858 C0 observations (#166)."""
from pathlib import Path
import struct
from dialog_evidence import FEATURE, dialog_baseline
from native_carriers import digest
from legend_grouping_projection import grouping_observation


def grouping_fixture_words(path: Path, count: int) -> list[int]:
    """Check every complete parent's stored C0 pair without consulting its reset events; e.g. grouping_fixture_words(path, 557)."""
    raw = path.read_bytes(); assert struct.unpack_from('<II', raw) == (1, count)
    offset = 8; words: list[int] = []
    for _ in range(count):
        family, version, size = struct.unpack_from('<III', raw, offset); offset += 12
        parent = raw[offset:offset + size]; assert len(parent) == size; offset += size
        mask, word = struct.unpack_from('<II', raw, offset); offset += 15 * 4
        fields = grouping_observation(family, version, parent)
        assert len(fields) == mask & 1
        assert word == (int(fields[0]['signed_value']) & 0xffffffff if fields else 0)
        words.extend((mask & 1, word))
    assert offset == len(raw), (offset, len(raw))
    return words


def verify_grouping_fixture_projection() -> None:
    """Bind all557 owned/mutant and276 native complete fixture parents to C858; e.g. verify_grouping_fixture_projection()."""
    root = FEATURE.parents[2]
    paths = ((root / 'tests/fixtures/simulation-post-dialog/legend-resets-projection.bin', 557),
        (root / 'target/simulation-legend-resets-projection-native.bin', 276))
    packets = [grouping_fixture_words(path, count) for path, count in paths]
    assert len(packets[0]) == 1114 and len(packets[1]) == 552
    assert sum(packets[0][::2]) == 356 and sum(packets[1][::2]) == 160
    dialog_baseline('legend-grouping-fixture-reference.json', dict(owned_contexts=557, native_contexts=276,
        words=1666, packets=[dict(path=str(path.relative_to(root)), bytes=path.stat().st_size, sha256=digest(path.read_bytes()))
        for path, _ in paths], reset_expectations_consumed=False, oracle='Complete Source C858 C0 spans, independent of derived reset events'))
