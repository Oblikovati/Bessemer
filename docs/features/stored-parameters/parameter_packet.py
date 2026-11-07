"""Decode actual Rust exports with explicit bounds, without rebuilding expected values (#202)."""
from verify_actual_reference import RootPacket


class ParameterPacket(RootPacket):
    def frame(self) -> bytes:
        """Bound values by the native corpus archive budget; e.g. packet.frame()."""
        size = self.word()
        assert size <= 64 * 1024 * 1024, f'{size}; expected a frame within the64MB corpus budget'
        return self.take(size)

    def present(self) -> bool:
        """Require an explicit0/1 protocol discriminator; e.g. packet.present()."""
        value = self.word()
        assert value in (0, 1), f'{value}; expected presence0/1'
        return value == 1

    def tag(self) -> tuple[object, ...]:
        """Retain physical framing and original map metadata; e.g. packet.tag()."""
        word = self.word()
        extension = self.word() if self.present() else None
        kind, identity = self.word(), self.word()
        name, schema = self.frame().decode('utf-8'), self.word()
        assert kind in (0, 1, 2), f'{kind}; expected null/new/reference'
        return word, extension, kind, identity, name, schema

    def name(self) -> dict[str, object]:
        """Decode both pointer identity and actual text/wire bytes; e.g. packet.name()."""
        tag, origin = self.tag(), self.word()
        assert origin in (0, 1, 2), f'{origin}; expected null/inherited/local origin'
        text = None if origin == 0 else (self.frame().decode('utf-8'), self.frame())
        return dict(tag=tag, origin=origin, text=text)

    def value(self) -> tuple[object, ...]:
        """Keep numeric images separate from text and framing; e.g. packet.value()."""
        kind = self.word()
        if kind in (0, 1):
            return kind, self.take(8 if kind == 0 else 4)
        assert kind == 2, f'{kind}; expected double/integer/CString value'
        return kind, self.frame().decode('utf-8'), self.frame()

    def parameter(self) -> dict[str, object]:
        """Read one complete result record; e.g. packet.parameter()."""
        definition, kind = self.tag(), self.word()
        assert kind in range(6), f'{kind}; expected one of the six physical parameter classes'
        names = [self.name(), self.name()]
        presence = self.word() if self.present() else None
        primary = self.value()
        alternate = self.value() if self.present() else None
        return dict(definition=definition, kind=kind, names=names,
                    presence=presence, primary=primary, alternate=alternate)

    def parameters(self) -> list[dict[str, object]]:
        """Bound each physical context's result count; e.g. packet.parameters()."""
        count = self.word()
        assert count <= 100_000, f'{count}; expected no more than the native object budget'
        return [self.parameter() for _ in range(count)]
