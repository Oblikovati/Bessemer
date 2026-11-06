"""Independent width projection cases, including ignored context and conversion edges."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
from pathlib import Path
from freeze_values import ROOT, SOURCE, MAXIMUM, flowfieldvalues
from wire_packet import counted, optional
from swarchive import ArchiveError


def signed(value: int) -> dict:
    return dict(kind=0, value=value)


def floating(bits: int) -> dict:
    return dict(kind=1, value=bits)


def numeric_value(value: dict) -> int | float:
    return value["value"] if value["kind"] == 0 else struct.unpack("<d", struct.pack("<Q", value["value"]))[0]


def request(code: int, cached: int | None = None, index: int | None = None, numeric: list[dict] | None = None) -> dict:
    return dict(code=code, cached=cached, index=index, numeric=numeric)


def scalar_cases() -> list[dict]:
    cases = [request(code, 7, 0, [floating(0x402dcccccccccccd)]) for code in range(-128, 128)]
    cases += [request(code, 7, 0, [signed(14)]) for code in (-(1 << 31), -129, 128, MAXIMUM)]
    for code in (-128, -2, 0, 2, 127):
        cases += [request(code), request(code, -1, -(1 << 31), [floating(0x7ff0000000000001)])]
    cases += [request(-1, cached) for cached in (None, -(1 << 63), -1, 0, 1, MAXIMUM, MAXIMUM + 1, (1 << 63) - 1)]
    cases += [request(1, index=index, numeric=numeric) for index, numeric in
              ((None, None), (0, None), (-1, None), (None, []), (0, []), (-1, []),
               (-1, [floating(0x7ff0000000000001)]), (1, [floating(0x7ff0000000000000)]))]
    return cases


def numeric_cases() -> list[dict]:
    bits = (0, 0x8000000000000000, 1, 0x8000000000000001, 0x3fefffffffffffff,
            0xbfefffffffffffff, 0xbfc999999999999a, 0xbff0000000000000, 0xbff028f5c28f5c29,
            0x3fffffffffffffff, 0x402dcccccccccccd, 0x41dfffffffc00000,
            0x41dfffffffffffff, 0x41e0000000000000, 0xc1e0000000000000,
            0x7ff0000000000000, 0xfff0000000000000, 0x7ff8000000001234,
            0x7ff0000000000001, 0xfff8000000001234)
    cases = [request(1, index=0, numeric=[floating(value)]) for value in bits]
    cases += [request(1, index=0, numeric=[signed(value)]) for value in
              (-(1 << 63), -1, 0, 1, MAXIMUM, MAXIMUM + 1, (1 << 63) - 1)]
    cases += [request(1, index=1, numeric=[floating(0x7ff0000000000001), signed(14)]),
              request(1, index=0, numeric=[signed(14), floating(0x7ff0000000000001)])]
    return cases


def outcome(case: dict) -> int | None:
    numeric = None if case["numeric"] is None else [numeric_value(value) for value in case["numeric"]]
    try:
        return flowfieldvalues.region_width(case["code"], cached_width=case["cached"],
                                            region_index=case["index"], numeric_widths=numeric)
    except ArchiveError:
        return None


def case_packet(case: dict) -> bytes:
    raw = struct.pack("<i", case["code"]) + optional(case["cached"], "q") + optional(case["index"], "i")
    numeric = case["numeric"]
    raw += bytes((numeric is not None,))
    if numeric is not None:
        raw += struct.pack("<I", len(numeric))
        raw += b"".join(bytes((value["kind"],)) + struct.pack("<q" if value["kind"] == 0 else "<Q", value["value"]) for value in numeric)
    return raw + optional(outcome(case), "I")


def freeze(output_root: Path = ROOT) -> None:
    cases = scalar_cases() + numeric_cases()
    packet = b"FWP1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-field-values/widths.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 reference_sha256=hashlib.sha256((SOURCE / "flowfieldvalues.py").read_bytes()).hexdigest(),
                 accepted=sum(outcome(case) is not None for case in cases), rejected=sum(outcome(case) is None for case in cases),
                 scope="Isolated native width calculation with explicit cached/selected numeric context; no binding to global record-102 slot 118 or native floating-point exception state")
    report = output_root / "docs/features/flow-field-values/width-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
