"""Narrow decoder presentation equivalence cannot erase meaningful prefixes or operands (#166)."""
from style_preferences_disassembly import canonical_style_opcode, canonical_style_operands


def require_style_decoder_rejection(mnemonic: str, operands: str, raw: bytes) -> None:
    """Require unsupported prefix/alias bytes to fail explicitly; e.g. require_style_decoder_rejection('rex', 'push rbx', bytes.fromhex('4853'))."""
    try:
        canonical_style_opcode(mnemonic, operands, raw)
    except AssertionError:
        return
    raise AssertionError((mnemonic, operands, raw.hex(), 'expected unsupported prefix/alias rejection'))


def verify_vector_style_decoder_spellings() -> None:
    """Pin observed NOP aliases while preserving meaningful operand spelling and prefix constraints; e.g. verify_vector_style_decoder_spellings()."""
    assert canonical_style_operands('nop', 'dword ptr [rax+rax*1+0]') == 'dwordptr[rax+rax]'
    assert canonical_style_operands('lea', 'rcx,[rax+rax*1+0]') == 'rcx,[rax+rax*1+0]'
    assert canonical_style_opcode('rex', 'push rbx', bytes.fromhex('4053')) == ('push', 'rbx')
    assert canonical_style_opcode('xchg', 'ax,ax', bytes.fromhex('6690')) == ('nop', '')
    assert canonical_style_opcode('data16', 'data16 nop word ptr [rax]', bytes.fromhex('66660f1f00')) == ('nop', 'word ptr [rax]')
    require_style_decoder_rejection('rex', 'push rbx', bytes.fromhex('4853'))
    require_style_decoder_rejection('data16', 'mov ax,1', bytes.fromhex('66b80100'))
    require_style_decoder_rejection('xchg', 'ax,ax', bytes.fromhex('4890'))
