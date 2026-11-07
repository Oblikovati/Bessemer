"""Independent complete reference-geometry field traces, retaining first object declarations."""
import dl_tess
import struct
from itertools import zip_longest
from generate_blank_scalars import flatten

COLLECTORS = {"ref_geom_feature_data_collector", "ref_geom_data_collector"}
KINDS = {name: kind for kind, (name, _) in dl_tess.REF_GEOM_TYPES.items()}


def collector(path: str, items: list[dict[str, object]], rows: dict[str, str]) -> None:
    rows[f"{path}.count_high"] = f"i:{len(items) >> 32}"
    rows[f"{path}.count_low"] = f"i:{len(items) & 0xffffffff}"
    for index, item in enumerate(items):
        prefix = f"{path}.entries[{index}]"
        rows[f"{prefix}.type"] = f"i:{KINDS[item['_class']]}"
        flatten(prefix, item, rows)


def walk(path: str, value: object, rows: dict[str, str], seen: set[int]) -> None:
    if isinstance(value, dict):
        if id(value) in seen:
            return
        seen.add(id(value))
        for key, child in value.items():
            if key in COLLECTORS:
                collector(f"{path}.{key}", child, rows)
            elif not key.startswith("_"):
                suffix = ".components" if key == "tessellation" and isinstance(child, list) else ""
                walk(f"{path}.{key}{suffix}", child, rows, seen)
    elif isinstance(value, (list, tuple)):
        for index, child in enumerate(value):
            walk(f"{path}[{index}]", child, rows, seen)


def trace(root: dict[str, object]) -> bytes:
    rows: dict[str, str] = {}
    walk("root", root, rows, set())
    return "".join(f"{key}={value}\n" for key, value in sorted(rows.items())).encode()


def frames(raw: bytes) -> list[bytes]:
    count, = struct.unpack_from("<I", raw)
    position = 4
    result: list[bytes] = []
    for _ in range(count):
        payload, position = frame_at(raw, position)
        result.append(payload)
    assert position == len(raw)
    return result


def frame_at(raw: bytes, position: int) -> tuple[bytes, int]:
    length, = struct.unpack_from("<I", raw, position)
    start = position+4
    end = start+length
    assert end <= len(raw), f"frame ending at {end}; expected at most {len(raw)} bytes"
    return raw[start:end], end


def compare_trace(actual: bytes, expected: bytes, label: str) -> int:
    if actual == expected:
        return len(actual.splitlines())
    for index, (current, wanted) in enumerate(zip_longest(actual.splitlines(), expected.splitlines())):
        if current != wanted:
            raise AssertionError(f"{label}: reference field {index} was {current!r}; expected {wanted!r}")
    raise AssertionError(f"{label}: reference trace framing {actual!r}; expected {expected!r}")
