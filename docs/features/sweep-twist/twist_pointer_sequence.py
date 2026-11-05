"""Operation-local MFC identities for complete sweep-twist fixtures (#211)."""
from dataclasses import dataclass, field
from typing import Literal
import struct
from fixture_object_map import WireMap, object_tag

TWIST_CLASS = 'moTwistDataForBiDirectionalSweep_c'
TwistStyle = Literal['fresh', 'shared', 'cached-class']


@dataclass
class TwistPointerSequence:
    style: TwistStyle
    payloads: tuple[bytes, ...]
    class_slot: int = 0
    object_slots: list[int] = field(default_factory=list)
    position: int = 0

    def __call__(self, mapping: WireMap) -> bytes:
        """Reserve identities in the enclosing archive; e.g. sequence(mapping)."""
        self.validate()
        payload = self.payloads[self.position]
        self.position += 1
        if self.class_slot and self.style == 'shared':
            return struct.pack('<H', self.object_slots[0])
        if self.class_slot and self.style == 'cached-class':
            mapping.slot += 1
            self.object_slots.append(mapping.slot)
            return struct.pack('<H', 0x8000 | self.class_slot) + payload
        self.class_slot = mapping.slot + 1
        mapping.slot += 2
        self.object_slots.append(mapping.slot)
        return object_tag(TWIST_CLASS, 1) + payload

    def validate(self) -> None:
        """Reject malformed literal plans before changing state; e.g. sequence.validate()."""
        if self.style not in ('fresh', 'shared', 'cached-class'):
            raise ValueError(f'twist style {self.style!r}; expected fresh, shared, or cached-class')
        if self.position >= len(self.payloads):
            raise ValueError(f'twist position {self.position}; expected below {len(self.payloads)}')
        payload = self.payloads[self.position]
        if len(payload) != 17:
            raise ValueError(f'twist payload length {len(payload)}; expected 17 bytes')
