"""Complete bounded C891 caller/copy/read/clamp fixture matrix (#166)."""
from itertools import product
from collections import Counter
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from aux_triangle_packets import aux_alpha_patterns
from plot54_plan_oracle import plot54_plan_oracle

MODES = ((0, 0),) + tuple((1, value) for value in (0, 1, 2, 3, 4, 5, 0xffffffff, 0x80000000, 0x7fffffff))
PLOTS = ((0, 0),) + tuple((1, value) for value in (0xffffffff, 0, 1, 53, 54, 55, 0x80000000, 0x7fffffff))
FORWARDED = ((0x3f800000,) * 4, (0, 0x80000000, 0x7fa00042, 0xffc00042), (0xffffffff,) * 4, (1, 0x7f800000, 0xff800000, 0))


def plot54_plan_case(mode: tuple[int, int], plot: tuple[int, int], present: int, core: int,
                     callback: int, alpha: int, forwarded: tuple[int, ...] = FORWARDED[0]) -> tuple[bytes, int]:
    """Keep unknown facts tagged independently of known zero values; e.g. plot54_plan_case((1, 3), (1, 54), 1, 2, 2, 0)."""
    words = [*mode, *plot, present, 0x7fa00042, 0x80000000, 0xffffffff, alpha, *forwarded, core, callback]
    assert len(words) == 15
    expected = plot54_plan_oracle(words)
    return struct.pack('<15I', *words) + expected, struct.unpack_from('<I', expected)[0]


def plot54_operation_cases() -> list[tuple[bytes, int]]:
    """Cover all lazy dependency stages and float classes while2BC remains independently preserved; e.g. plot54_operation_cases()."""
    cases = [plot54_plan_case(mode, plot, present, core, callback, alpha)
        for mode, plot, present, core, callback, alpha in product(MODES, PLOTS, (0, 1), range(3), range(3), (0, 0x3f800001, 0xffc00042))]
    cases.extend(plot54_plan_case((1, mode), (1, 54), 1, 2, 2, alpha, forwarded)
        for mode, alpha, forwarded in product((3, 4), aux_alpha_patterns(), FORWARDED))
    return cases


def verify_plot54_plan_packets() -> None:
    """Freeze exact local flag/copy/projection results without a geometry or invocation claim; e.g. verify_plot54_plan_packets()."""
    cases = plot54_operation_cases()
    packet = struct.pack('<2I', 1, len(cases)) + b''.join(row for row, _ in cases)
    packets = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'plot54-plans.bin': packet})
    dialog_baseline('plot54-plan-reference.json', dict(cases=len(cases),
        statuses=dict(sorted(Counter(str(status) for _, status in cases).items())), alpha_patterns=len(aux_alpha_patterns()),
        forwarded_2bc_patterns=len(FORWARDED), packets=packets, target_executed=False, additional_stored_byte_credit=0,
        assumptions='Masked exceptions, DAZ/FTZ off; local normal-flow colors only, without native virtual state/callback effects'))
