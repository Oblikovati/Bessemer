"""Freeze known count prefixes and conditional absent-map initialization without executing context."""
from __future__ import annotations
import gzip
import hashlib
import json
import struct
import sys
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
SOURCE = ROOT.parent / "inventor-interface/solidworks"
sys.path.insert(0, str(SOURCE))
import flowcontextcounts
import flowcountbinding
import flowresults


def blob(raw: bytes) -> bytes:
    return struct.pack("<I", len(raw)) + raw


def signed_words(values: list[int]) -> bytes:
    return struct.pack("<I", len(values)) + struct.pack("<" + "i" * len(values), *values)


def native_cases() -> list[dict]:
    rows = json.loads((SOURCE / "research/flow-block-data-corpus.json").read_text())
    documents = []
    for row in rows:
        raw = Path(row["path"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
        documents.append(flowresults.read(raw))
    counts = flowresults.latest(documents[1], 406)["payload"]
    arrays = flowresults.latest(documents[0], 433)["payload"]
    assert hashlib.sha256(counts).hexdigest() == rows[1]["payload_sha256"]
    bound = flowcountbinding.read_record_433(dict(identifier=433, size=len(arrays), payload=arrays),
        counts_decoded=zlib.decompress(counts), initialize_absent_map=True)
    assert bound["count"] == 5952 and bound["count_binding"]["solid_pointer_byte_offset"] == 26944
    return [dict(name="native-cpt-cfld", compressed=True, stored=counts,
                 decoded=zlib.decompress(counts), arrays=[arrays])]


def owned_arrays(fluid: int, solid: int) -> list[bytes]:
    count = fluid + solid
    if count > 8:
        return []
    patterns = (-2147483648, -1, 0, 2147483647)
    arrays = []
    for selector in (0, 7):
        values = [patterns[i % 4] for i in range(count * (2 if selector == 0 else 1))]
        raw = struct.pack("<ii", selector, count) + struct.pack("<" + "i" * len(values), *values)
        arrays.append(zlib.compress(raw, 9))
    return arrays


def owned_case(version: int, counts: tuple[int, int], tail: bytes, compressed: bool) -> dict:
    fluid, solid = counts
    decoded = struct.pack("<3i", version, fluid, solid) + tail
    return dict(name=f"v{version}-{fluid}-{solid}-tail{len(tail)}", compressed=compressed,
                stored=zlib.compress(decoded, 9) if compressed else decoded,
                decoded=decoded, arrays=owned_arrays(fluid, solid))


def owned_cases() -> list[dict]:
    counts = ((0, 0), (0, 1), (1, 0), (3, 5), (2147483647, 0), (0, 2147483647),
              (2147483647, 1), (2147483647, 2147483647))
    return [owned_case(version, pair, tail, compressed)
            for version in (1, 7, 8, 17, 18, 2147483647) for pair in counts
            for tail in (b"", bytes.fromhex("00ff8001020304deadbeef")) for compressed in (False, True)]


def binding_packet(binding: dict) -> bytes:
    raw = struct.pack("<IIQIIQQ", binding["expected_433_count"], binding["pointer_slot_bytes"],
        binding["pointer_array_bytes"], binding["fluid_start_index"], binding["solid_start_index"],
        binding["fluid_pointer_byte_offset"], binding["solid_pointer_byte_offset"])
    flags = ("initialization_assumed", "native_count_assignment_proven", "native_context_executed",
             "pointer_population_reproduced", "existing_map_context_supported")
    return raw + bytes(binding[key] for key in flags)


def array_packet(case: dict, stored: bytes) -> bytes:
    record = dict(identifier=433, size=len(stored), payload=stored)
    bound = flowcountbinding.read_record_433(record, counts_decoded=case["decoded"], initialize_absent_map=True)
    raw = blob(stored) + blob(zlib.decompress(stored)) + struct.pack("<ii", bound["selector"], bound["count"])
    raw += bytes([bound["first"] is not None]) + signed_words(bound["first"] or [])
    return raw + signed_words(bound["second"]) + bytes([bound["native_context_checked"]])


def case_packet(case: dict) -> bytes:
    record = dict(identifier=406, payload=case["stored"], size=len(case["stored"]))
    parsed = flowcontextcounts.read_record(record, compression="zlib" if case["compressed"] else "none")
    raw = blob(case["name"].encode()) + bytes([case["compressed"]]) + blob(case["stored"]) + blob(case["decoded"])
    raw += struct.pack("<4iQB", parsed["version"], parsed["g_fluid"], parsed["g_solid"],
        parsed["g_total_native_i32"], parsed["g_total_mathematical"], parsed["native_body_branch"] == "restored_18_plus")
    raw += blob(parsed["opaque_tail"])
    admissible = parsed["g_total_mathematical"] <= 2147483647
    raw += bytes([admissible])
    if admissible:
        binding = flowcountbinding.derive_decoded(case["decoded"], initialize_absent_map=True)
        raw += binding_packet(binding)
    else:
        assert not case["arrays"]
    return raw + struct.pack("<I", len(case["arrays"])) + b"".join(array_packet(case, arrays) for arrays in case["arrays"])


def freeze(output_root: Path = ROOT) -> None:
    cases = native_cases() + owned_cases()
    packet = b"FCB1" + struct.pack("<I", len(cases)) + b"".join(map(case_packet, cases))
    destination = output_root / "tests/fixtures/flow-count-binding/binding.bin.gz"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(gzip.compress(packet, mtime=0))
    proof = dict(cases=len(cases), bound_arrays=sum(len(case["arrays"]) for case in cases),
                 bytes=len(packet), sha256=hashlib.sha256(packet).hexdigest(),
                 prefix_reference_sha256=hashlib.sha256((SOURCE / "flowcontextcounts.py").read_bytes()).hexdigest(),
                 binding_reference_sha256=hashlib.sha256((SOURCE / "flowcountbinding.py").read_bytes()).hexdigest(),
                 scope="Exact twelve-byte prefix and retained opaque body; conditional absent-map count/layout; no native context execution")
    report = output_root / "docs/features/flow-count-binding/fixture-verification.json"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.write_text(json.dumps(proof, indent=2) + "\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    freeze()
