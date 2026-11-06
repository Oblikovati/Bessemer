"""Freeze version-prefixed record-108 bodies without inferring runtime layouts."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
import zlib
from pathlib import Path
from wire_packet import blob, counted, optional, packed

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
import flowfieldvalues
import flowcontextcounts
import flowresults

MAXIMUM = (1 << 31) - 1
BITS = (0, 0x8000000000000000, 0x7ff8000000001234, 0x7ff0000000000001,
        0x7ff0000000000000, 0xfff0000000000000, 1, 0xffffffffffffffff)
LEGACY_NAMES = ("empty", "empty-expected", "zero-objects", "rich", "unchecked", "wildcards",
                "partial", "wide-256", "many-zero", "all-wide-patterns")
REGION_NAMES = ("empty", "rich", "inactive-max", "zero-rows", "inactive-positive", "active-empty",
                "rich-mixed", "many-zero", "many-inactive", "wide-256", "active-booleans", "zero-width-huge-rows")


def patterns(count: int) -> bytes:
    return packed([BITS[index % len(BITS)] for index in range(count)], "Q")


def legacy_context(name: str) -> tuple[list[int], dict]:
    table, widths, expected = [], [], None
    if name == "empty-expected":
        expected = []
    elif name == "zero-objects":
        table, widths, expected = [0], [0, 0, 0], [0]
    elif name == "rich":
        table, widths, expected = [2, 1], [2, 0, 1], [2, 1]
    elif name in ("unchecked", "wildcards", "partial"):
        table, widths = [-(1 << 31), -1, 0, MAXIMUM], [0, 1, 0, 2]
        expected = None if name == "unchecked" else [None] * 4 if name == "wildcards" else [None, None, 0, MAXIMUM]
    elif name == "wide-256":
        table, widths, expected = [256], [255, 0, 1, 0], [256]
    elif name == "many-zero":
        widths = [0] * 4096
    elif name == "all-wide-patterns":
        table, widths, expected = [MAXIMUM], [8], [None]
    return table, dict(kind="legacy", width_table_count=len(table), object_widths=widths, expected_width_table=expected)


def region_context(name: str) -> dict:
    layouts = {
        "empty": ([], [], []), "rich": ([2, 3, 4], [2, 0, 1], [True, True, False]),
        "inactive-max": ([MAXIMUM, MAXIMUM, 0], [MAXIMUM, 0, MAXIMUM], [False, True, True]),
        "zero-rows": ([0, 0, 0], [0, 1, MAXIMUM], [True] * 3),
        "inactive-positive": ([2, 1, 4], [2, 3, 0], [False] * 3),
        "active-empty": ([0, 2, 0], [8, 0, 1], [True] * 3),
        "rich-mixed": ([1, 0, 3, 2], [8, MAXIMUM, 1, 0], [True, False, False, True]),
        "many-zero": ([0] * 4096, [0] * 4096, [index % 2 == 0 for index in range(4096)]),
        "many-inactive": ([MAXIMUM] * 4096, [MAXIMUM] * 4096, [False] * 4096),
        "wide-256": ([2], [128], [True]),
        "active-booleans": ([1] * 8, [1] * 8, [index % 2 == 0 for index in range(8)]),
        "zero-width-huge-rows": ([MAXIMUM], [0], [True]),
    }
    rows, widths, active = layouts[name]
    return dict(kind="regions", row_counts=rows, expected_widths=widths, active=active)


def decoded_body(context: dict, table: list[int] | None = None) -> bytes:
    if context["kind"] == "legacy":
        return packed(table, "i") + b"".join(patterns(width) for width in context["object_widths"])
    raw = struct.pack("<i", len(context["row_counts"]))
    for rows, width, active in zip(context["row_counts"], context["expected_widths"], context["active"]):
        raw += struct.pack("<i", width)
        if active and width:
            raw += patterns(rows * width)
    return raw


def transports(name: str, version: int, decoded: bytes, context: dict, restriction: bool) -> list[dict]:
    return [dict(name=name, version=version, decoded=decoded, context=context, restriction=restriction,
                 compressed=compressed, stored=struct.pack("<i", version) + (zlib.compress(decoded, 9) if compressed else decoded))
            for compressed in (False, True)]


def owned_cases() -> list[dict]:
    cases = []
    for version in (5, 6):
        for restriction in (False, True):
            for name in LEGACY_NAMES:
                table, context = legacy_context(name)
                cases += transports(f"v{version}-{name}-restriction-{int(restriction)}", version, decoded_body(context, table), context, restriction)
    for version, restriction in ((7, False), (7, True), (8, False), (99, False), (MAXIMUM, False)):
        for name in REGION_NAMES:
            context = region_context(name)
            cases += transports(f"v{version}-{name}-restriction-{int(restriction)}", version, decoded_body(context), context, restriction)
    return cases


def native_cases() -> list[dict]:
    row = json.loads((SOURCE / "research/flow-field-values-corpus.json").read_text())
    fld, cpt = Path(row["fld_path"]).read_bytes(), Path(row["cpt_path"]).read_bytes()
    assert hashlib.sha256(fld).hexdigest() == row["fld_source_sha256"]
    assert hashlib.sha256(cpt).hexdigest() == row["cpt_source_sha256"]
    counts = flowcontextcounts.read_decoded(zlib.decompress(flowresults.latest(flowresults.read(cpt), 406)["payload"]))
    record = flowresults.latest(flowresults.read(fld), 108)
    assert hashlib.sha256(record["payload"]).hexdigest() == row["payload_sha256"]
    decoded = zlib.decompress(record["payload"][4:])
    assert hashlib.sha256(decoded).hexdigest() == row["decoded_body_sha256"]
    context = row["explicit_layout_context"]
    assert context["object_widths"] == [14] * counts["g_fluid"] + [0] * counts["g_solid"]
    return [dict(name=f"native-fld-108-{label}", stored=record["payload"], decoded=decoded, version=5,
                 compressed=True, restriction=False, context=dict(context, expected_width_table=expected))
            for label, expected in (("checked", [14]), ("unchecked", None), ("wildcard", [None]))]


def context_packet(context: dict) -> bytes:
    if context["kind"] == "legacy":
        raw = b"\0" + struct.pack("<I", context["width_table_count"]) + counted(context["object_widths"], "I")
        expected = context.get("expected_width_table")
        raw += bytes((expected is not None,))
        if expected is not None:
            raw += struct.pack("<I", len(expected)) + b"".join(optional(width, "I") for width in expected)
        return raw
    return b"\1" + counted(context["row_counts"], "I") + counted(context["expected_widths"], "I") + blob(bytes(context["active"]))


def object_packet(value: dict) -> bytes:
    return packed([value[key] for key in ("ordinal", "width", "start", "end")], "I") + packed(value["raw_bits"], "Q")


def region_packet(value: dict) -> bytes:
    return packed([value[key] for key in ("ordinal", "start", "end", "row_count", "width")], "I") + bytes((value["active"],)) + counted(value["raw_bits"], "Q")


def projection_packet(parsed: dict) -> bytes:
    raw = struct.pack("<i", parsed["version"])
    if parsed["layout"] == "global_object_order":
        raw += b"\0" + counted(parsed["stored_width_table"], "i") + bytes((parsed["expected_width_table_checked"],))
        raw += struct.pack("<I", len(parsed["objects"])) + b"".join(map(object_packet, parsed["objects"]))
    else:
        raw += b"\1" + bytes((parsed["expected_region_count_checked"], parsed["expected_region_widths_checked"]))
        raw += struct.pack("<I", len(parsed["regions"])) + b"".join(map(region_packet, parsed["regions"]))
    raw += packed([parsed[key] for key in ("end", "stored_version_bytes", "stored_body_bytes", "decoded_body_bytes", "decoded_payload_bytes")], "I")
    return raw + bytes(parsed[key] for key in ("native_context_executed", "layout_context_assumed", "native_storage_reproduced", "engineering_columns_resolved"))


def case_packet(case: dict) -> bytes:
    parsed = flowfieldvalues.read_record(dict(identifier=108, size=len(case["stored"]), payload=case["stored"]),
        body_compression="zlib" if case["compressed"] else "none", context=case["context"], compatibility_restriction=case["restriction"])
    raw = blob(case["name"].encode()) + bytes((case["compressed"], case["restriction"])) + context_packet(case["context"])
    return raw + blob(case["stored"]) + blob(case["decoded"]) + projection_packet(parsed)


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + owned_cases()
    packet = b"FFV1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-field-values/values.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowfieldvalues.py").read_bytes()).hexdigest(),
                 native_payloads=1, native_context_views=3, native_version=5, native_objects=5952,
                 native_wide_words=47152, native_decoded_body_bytes=377220,
                 scope="Complete version-prefixed field arrays with explicit layout, compatibility and body transport; no runtime binding, storage reconstruction or engineering columns")
    report = output_root / "docs/features/flow-field-values/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
