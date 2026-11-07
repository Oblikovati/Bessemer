"""Literal object-map assembly without importing reference readers (#192)."""
from collections.abc import Callable
import struct


def object_tag(name: str, schema: int = 7) -> bytes:
    """Emit independently specified class metadata; e.g. object_tag('suObArray', 1)."""
    encoded = name.encode('ascii')
    return struct.pack('<HHH', 65535, schema, len(encoded)) + encoded


class WireMap:
    def __init__(self, schema: int = 7) -> None:
        """Keep existing fixture defaults while allowing an explicit schema; e.g. WireMap(1)."""
        self.slot = 0
        self.schema = schema

    def new(self, name: str, body: Callable[[], bytes]) -> bytes:
        """Reserve class/object slots before nested bodies; e.g. mapping.new('suObArray', lambda: bytes(2))."""
        self.slot += 2
        return object_tag(name, self.schema) + body()

    def handle(self, name: str = 'sgPointHandle') -> bytes:
        """Retain the existing escaped-index handle literal; e.g. mapping.handle()."""
        return self.new(name, lambda: struct.pack('<Hiii', 0x777f, -1, -19, 2**31-1))

    def array(self, name: str = 'suObArray', extended: bool = False) -> bytes:
        """Retain short/extended count fixtures and null pointers; e.g. mapping.array(extended=True)."""
        return self.new(name, lambda: (struct.pack('<HI', 65535, 2) if extended else struct.pack('<H', 2)) + bytes(4))


def shared_object(mapping: WireMap, identities: dict[str, int], key: str, name: str,
                  contents: Callable[[], bytes]) -> bytes:
    """Reserve identity before nested objects and retain aliases; e.g. shared_object(mapping, ids, 'profile', 'moCompProfile_c', body)."""
    if key in identities:
        return struct.pack('<H', identities[key])
    def reserved() -> bytes:
        identities[key] = mapping.slot
        return contents()
    return mapping.new(name, reserved)
