"""Independent OUT bodies with exact reviewed syntax and byte-edit goldens (#114)."""
import struct

SI = b' International System of Units, SI, is used for all input/output data.'


def transfer(body: bytes) -> bytes:
    """Reproduce successful-copy framing independently from the reader."""
    assert body
    return struct.pack('<I', (len(body) - 1) // 0xA00000 + 1) + body


def owned_cases() -> list[tuple[str, bytes]]:
    """Retain unknown lines, duplicate symbols and exact decimal patterns."""
    bodies = [
        ('duplicate-symbols', b' Form stiffness ... (IFORMK) = 0\r\n EQ. 0, off\r\n Unknown text\r\n Inertia relief (IFORMK)=1\r\n EQ. +1, on\r\n'),
        ('decimal-patterns', b' Small (A)=+0.100D-03\r\n Negative zero(B)=-0.000e+2\r\n Fraction(C)=.5\r\n Trailing dot(D)=1.\r\n'),
        ('nul-si-and-empty-lines', b'\0B\0A\0N\0N\0ER\r\n\r\n' + SI + b'\r\n Controls (K)=2\r\n'),
        ('unterminated', b' Unknown path C:\\study\r\n Value (V)= -12.5E+2'),
        ('unknown-and-dangling-legend', b' EQ. 1, no preceding occurrence\r\n Wrong (lower)=2\r\n Broken (A)=2x\r\n Unknown\r\n'),
        ('empty-final-line', b'\r\n'),
        ('nul-only', b'\0'),
        ('signed-legends', b' A.. label (A_1) = -1\r\n EQ. -1, negative\r\n EQ. 0, zero\r\n EQ. +2, positive\r\n'),
        ('transfer-upper-boundary', b'x' * (0xA00000 + 1)),
        ('edit-crosses-transfer-boundary', b'x' * (0xA00000 - len(b'\r\nV(X)=1')) + b'\r\nV(X)=1'),
    ]
    return [(name, transfer(body)) for name, body in bodies]
