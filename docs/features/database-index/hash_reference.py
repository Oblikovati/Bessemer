"""Independent default-hash and bucket-request oracles from retained native instructions (#230)."""


def default_hash(key: bytes) -> int:
    """Model the selected unsigned32 prefix hash; e.g. default_hash(b'chm@symisc') == 0x03f523f5."""
    result = 5381
    for byte in key[:2048]:
        result = (33 * result + byte) & 0xffffffff
    return result


def closed_form_hash(key: bytes) -> int:
    """Use an independent modular polynomial oracle; e.g. closed_form_hash(b'abc')."""
    prefix = key[:2048]
    modulus = 1 << 32
    initial = 5381 * pow(33, len(prefix), modulus)
    weighted = sum(byte * pow(33, len(prefix) - index - 1, modulus)
                   for index, byte in enumerate(prefix))
    return (initial + weighted) % modulus


def requested_bucket(key_hash: int, split: int, maximum: int) -> int:
    """Preserve native unsigned64 wrapping request arithmetic; e.g. requested_bucket(7,0,4) == 3."""
    word = (1 << 64) - 1
    twice_mask = ((maximum * 2 & word) - 1) & word
    candidate = key_hash & twice_mask
    if ((split + maximum) & word) <= candidate:
        return key_hash & ((maximum - 1) & word)
    return candidate
