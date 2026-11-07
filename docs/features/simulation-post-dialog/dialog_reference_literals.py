"""Reuse reviewed reference-record literals with independent dialog labels and counts (#166)."""
from graph_literals import graph_words
from owned_thermal import thermal_reference_blocks, persisted_literal
from dialog_literal_values import DialogBlocks, DialogLiteralProfile, dialog_count

REFERENCE_LABELS = {'id_header': 'id_header', 'blob_count': 'blob_count', 'blob': 'blob',
    'archive0': 'archive[0]', 'archive1': 'archive[1]', 'header': 'words38_48',
    'name': 'persist_text', 'tail': 'words58_5c', 'late_tail': 'word60_raw64_68',
    'legacy_header': 'legacy_header', 'legacy_name': 'legacy_persist_text'}
PERSIST_TEXT_PAYLOADS: tuple[bytes | None, ...] = (None, b'\0\0', b'\0\xd8\0\0', b'X\0\0')


def dialog_reference_record(label: str, version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Preserve established historical/modern archive framing without parser-derived repairs; e.g. dialog_reference_record('reference.record[0]', 840002, profile)."""
    blocks: DialogBlocks = []
    for name, fragment in thermal_reference_blocks(version):
        if name in ('name', 'legacy_name'):
            fragment = persisted_literal(None if profile.empty_texts else PERSIST_TEXT_PAYLOADS[profile.rotation % 4])
        blocks.append((label + '.' + REFERENCE_LABELS[name], fragment))
    return blocks


def dialog_reference_container(label: str, version: int, profile: DialogLiteralProfile) -> DialogBlocks:
    """Keep duplicate records and mandatory trailing words in wire order; e.g. dialog_reference_container('dialog.reference_b30', 890620, profile)."""
    count = (2 if profile.duplicate_records else 1) if profile.populated else 0
    blocks = [dialog_count(label + '.count', count)]
    for index in range(count): blocks.extend(dialog_reference_record(f'{label}.record[{index}]', version, profile))
    return blocks + [(label + '.two_trailing_words', graph_words(2, profile.rotation))]
