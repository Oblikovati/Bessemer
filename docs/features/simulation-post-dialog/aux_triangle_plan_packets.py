"""Freeze reviewed helper request plans alongside Source's caller-gate oracle (#166)."""
import struct
from dialog_evidence import FEATURE, dialog_baseline
from dialog_operation_packets import persist_dialog_packets
from aux_triangle_plan_oracle import aux_plan_cases


def verify_aux_triangle_plan_packets() -> None:
    """Retain every runtime context, group snapshot and complete expected stage/call stream; e.g. verify_aux_triangle_plan_packets()."""
    rows = list(aux_plan_cases())
    assert len(rows) == 6628, len(rows)
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog',
        {'aux-triangle-plans.bin': struct.pack('<II', 1, len(rows)) + b''.join(rows)})
    dialog_baseline('aux-triangle-plan-reference.json', dict(cases=len(rows),
        oracle='Original Source caller gate plus reviewed helper unsigned guards and native pointer/stride memory traversal',
        kernel_decision='Missing required snapshots and exceeded explicit budgets fail before publication; unused context does not require group data',
        scope='Only ordered local slot58 requests, color provenance and admitted alpha cap; no native rendering or geometry validation',
        additional_stored_byte_credit=0, packets=summaries))
