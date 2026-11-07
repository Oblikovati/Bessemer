"""C893 reviewed primary/reflected submission and fixed triple-write branch decisions (#166)."""
from typing import cast
from dialog_evidence import dialog_baseline
from mode3_rgb_evidence import mode3_rgb_report


def mode3_primary_filter(pass_code: int, plot: int | None, selector: int | None) -> bool | None:
    """Keep the primary plot54 filter out of the reflected pass; e.g. mode3_primary_filter(1, None, None)."""
    assert pass_code in (0, 1), (pass_code, 'expected primary0 or reflected1')
    if pass_code == 1: return False
    if plot is None: return None
    if plot != 54: return False
    if selector is None: return None
    return ((selector - 1) & 0xffffffff) <= 1


def mode3_fixed_array_branch(capacity: int | None, growth_result: int | None) -> str:
    """Model count1/index5 admission without inventing allocator success; e.g. mode3_fixed_array_branch(6, None)."""
    if capacity is None: return 'missing_capacity'
    if capacity > 5: return 'existing_destination'
    if growth_result is None: return 'missing_growth_result'
    return 'accepted_growth' if growth_result == 0 else 'rejected_growth'


def verify_mode3_pass_anchors() -> None:
    """Retain both submission calls and their distinct surrounding guards; e.g. verify_mode3_pass_anchors()."""
    methods = cast(list[dict[str, object]], mode3_rgb_report()['functions'])
    caller = set(cast(list[str], methods[0]['instructions']))
    assert {'18015c50d cmp dword ptr [rsi + 0x170], 0x36', '18015c514 jne 0x18015c523',
        '18015c516 lea eax, [r15 - 1]', '18015c51a cmp eax, 1', '18015c51d jbe 0x18015c62b',
        '18015c6b4 cmp qword ptr [rsp + 0x68], 0', '18015c6ba jne 0x18015c6c5',
        '18015c6bc cmp r13d, 0x405', '18015c6c3 jne 0x18015c732',
        '18015c6e7 cmp r13d, 0x404', '18015c6ee jne 0x18015c709',
        '18015c7a6 lea r9, [rbp + 0x790]', '18015c7ad mov edx, 1', '18015c7b2 mov r8d, 5',
        '18015c7bf call 0x18018ae40'} <= caller
    calls = [line for line in cast(list[str], methods[0]['instructions']) if line.endswith('call 0x18018ae40')]
    assert calls == ['18015c5a8 call 0x18018ae40', '18015c7bf call 0x18018ae40']
    helper = set(cast(list[str], methods[1]['instructions']))
    assert {'18018a8d6 cmp eax, dword ptr [rcx + 0x10]', '18018a8d9 jl 0x18018a8f1',
        '18018a8e0 call 0x18018ad10', '18018a8e5 test eax, eax', '18018a8e7 je 0x18018a8f1',
        '18018a8e9 mov dword ptr [rbx], 4'} <= helper


def mode3_reviewed_filter_rows() -> list[tuple[int, int | None, int | None, bool | None]]:
    """Keep independently stated boundary expectations for both passes; e.g. mode3_reviewed_filter_rows()."""
    return [(0, None, None, None), (0, 53, None, False), (0, 55, None, False), (0, 54, None, None),
        (0, 54, -2147483648, False), (0, 54, -1, False), (0, 54, 0, False), (0, 54, 1, True),
        (0, 54, 2, True), (0, 54, 3, False), (0, 54, 2147483647, False),
        (1, None, None, False), (1, 54, None, False), (1, 54, 1, False), (1, 54, 2, False)]


def mode3_reviewed_array_rows() -> list[tuple[int | None, int | None, str]]:
    """Retain signed capacity and unknown/nonzero growth distinctions; e.g. mode3_reviewed_array_rows()."""
    return [(None, None, 'missing_capacity'), (None, 0, 'missing_capacity'),
        (-2147483648, None, 'missing_growth_result'), (-1, 0, 'accepted_growth'),
        (0, None, 'missing_growth_result'), (0, 0, 'accepted_growth'), (0, 1, 'rejected_growth'),
        (5, None, 'missing_growth_result'), (5, 0, 'accepted_growth'), (5, -1, 'rejected_growth'),
        (5, 2147483647, 'rejected_growth'), (6, None, 'existing_destination'),
        (6, -1, 'existing_destination'), (2147483647, None, 'existing_destination')]


def verify_mode3_rgb_kernel_decisions() -> None:
    """Bind complete methods to reviewed local pass/array boundary expectations; e.g. verify_mode3_rgb_kernel_decisions()."""
    verify_mode3_pass_anchors()
    filters, arrays = mode3_reviewed_filter_rows(), mode3_reviewed_array_rows()
    for pass_code, plot, selector, expected in filters:
        assert mode3_primary_filter(pass_code, plot, selector) == expected, (pass_code, plot, selector, expected)
    for capacity, growth, expected in arrays:
        assert mode3_fixed_array_branch(capacity, growth) == expected, (capacity, growth, expected)
    dialog_baseline('mode3-rgb-kernel-reference.json', dict(behavior_version='C893-local-color-and-triple-write-v1',
        pass_filter_rows=filters, fixed_array_rows=arrays, submission_addresses=['0x18015c5a8', '0x18015c7bf'],
        count=1, index=5, stride_bytes=12, target_offset_bytes=60, target_executed=False,
        allocator_execution_modeled=False, actual_submission_observed=False, additional_stored_byte_credit=0))
