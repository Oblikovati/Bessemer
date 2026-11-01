"""Shared exact captured-text qualification without global opcode normalization (#169)."""
from collections.abc import Mapping
from typing import cast
import pefile

CapturedNativeText = Mapping[int, tuple[str, str, str, str]]


def qualify_captured_native_line(line: str, raw: bytes | None, captures: CapturedNativeText) -> tuple[str, dict[str, str] | None]:
    """Admit only an exact recorded instruction and byte sequence; e.g. qualify_captured_native_line('180000000 ret', None, {})."""
    address = int(line.split(' ', 1)[0], 16)
    captured = captures.get(address)
    if captured is None: return line, None
    original, canonical, expected_hex, kind = captured
    assert line == original and raw == bytes.fromhex(expected_hex), (line, raw, captured)
    return canonical, dict(address=hex(address), original=original, canonical=canonical, raw_hex=expected_hex, kind=kind)


def qualify_captured_native_method(image: pefile.PE, method: dict[str, object], captures: CapturedNativeText) -> tuple[dict[str, object], list[dict[str, str]]]:
    """Copy qualified expectations while preserving the original immutable report; e.g. qualify_captured_native_method(image, method, {})."""
    lines: list[str] = []
    qualifications: list[dict[str, str]] = []
    for line in cast(list[str], method['instructions']):
        address = int(line.split(' ', 1)[0], 16)
        captured = captures.get(address)
        raw = image.get_data(address - image.OPTIONAL_HEADER.ImageBase, len(captured[2]) // 2) if captured is not None else None
        canonical, qualification = qualify_captured_native_line(line, raw, captures)
        lines.append(canonical)
        if qualification is not None: qualifications.append(qualification)
    return method | {'instructions': lines}, qualifications
