"""Project only independently observed fold byte-array raw spans (#54)."""
from dataclasses import replace
from primitive_inventory import PrimitiveRead, verify_primitive_coverage


def byte_spans(reads: list[PrimitiveRead]) -> list[PrimitiveRead]:
    """Select the fold serializer's actual byte-array read; e.g. byte_spans(reads)."""
    return [read for read in reads if read.end > read.start and read.kind == 'raw'
            and read.source_file == 'foldtransforms.py' and read.source_function == '<lambda>']


def project_fold_bytes(raw: bytes, reads: list[PrimitiveRead], expected: list[bytes]) -> list[PrimitiveRead]:
    """Keep all source reads unchanged and split only proven byte payloads; e.g. project_fold_bytes(raw, reads, [b'\x00\xff'])."""
    verify_primitive_coverage(raw, reads)
    reads = [read for read in reads if read.end > read.start]
    selected = byte_spans(reads)
    if [bytes.fromhex(read.raw_hex) for read in selected] != expected:
        raise ValueError(f'fold byte spans {selected}; expected independently decoded byte arrays {expected}')
    positions = {(read.start, read.end) for read in selected}
    projected: list[PrimitiveRead] = []
    for read in reads:
        if (read.start, read.end) not in positions:
            projected.append(read)
            continue
        projected.extend(replace(read, start=index, end=index+1, kind='B', raw_hex=raw[index:index+1].hex())
                         for index in range(read.start, read.end))
    verify_primitive_coverage(raw, projected)
    return projected
