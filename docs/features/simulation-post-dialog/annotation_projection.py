"""Complete C-887 stored controls retain older absence and independent Min/Max identities (#166)."""
from collections import Counter
from typing import cast
import struct
from native_carriers import digest
from dialog_evidence import FEATURE, dialog_baseline
from dialog_owned import dialog_owned_cases
from dialog_operation_packets import persist_dialog_packets
from local_render_evidence import local_render_report
from style_preferences_projection import raw_style_observations, raw_style_native_contexts, raw_style_identity
from annotation_evidence import annotation_report


def annotation_observation(family: int, version: int, raw: bytes) -> dict[str, dict[str, object]] | None:
    """Reuse the canonical complete-EOF raw-word projection without initializer defaults; e.g. annotation_observation(2, 700005, raw)."""
    words = raw_style_observations(family, version, raw)
    assert ('0x234' in words) == ('0x238' in words), (family, version)
    if '0x234' not in words: return None
    return {name: dict(raw_hex=words[member]['raw_hex'], offset=words[member]['offset'],
        enabled_view=words[member]['uint32'] != 0) for name, member in (('minimum', '0x234'), ('maximum', '0x238'))}


def verify_annotation_native() -> None:
    """Replay every original raw pair and physical identity, not only observed truth classes; e.g. verify_annotation_native()."""
    report = annotation_report()
    rows = cast(list[dict[str, object]], report['rows'])
    original = cast(list[dict[str, object]], local_render_report()['rows'])
    identities = {raw_style_identity(row): 0 if row['kind'] == 'cwPostDlgParam' else 2 for row in original}
    assert Counter(raw_style_identity(row) for row in rows) == Counter(identities.keys())
    available = {(family, digest(raw), version): annotation_observation(family, version, raw)
        for family, version, raw in raw_style_native_contexts()}
    for row in rows:
        assert available[(identities[raw_style_identity(row)], row['sha256'], row['selected_version'])] == {
            name: row[name] for name in ('minimum', 'maximum')}, row
    counts = Counter(f"min_{row['minimum']['enabled_view']}_max_{row['maximum']['enabled_view']}" for row in rows)
    assert dict(counts) == report['native_control_pairs']
    dialog_baseline('annotation-native-reference.json', dict(explicit_input_sets=276,
        native_control_pairs=dict(counts), additional_stored_byte_credit=0))


def annotation_context(family: int, version: int, raw: bytes) -> bytes:
    """Keep complete source bytes and explicitly frame missing historical controls; e.g. annotation_context(2, 700005, raw)."""
    observed = annotation_observation(family, version, raw)
    packet = struct.pack('<IIII', family, version, len(raw), int(observed is not None)) + raw
    if observed is None: return packet
    controls = b''.join(bytes.fromhex(cast(str, observed[name]['raw_hex'])) for name in ('minimum', 'maximum'))
    minimum, maximum = (int(observed[name]['enabled_view']) for name in ('minimum', 'maximum'))
    return packet + controls + struct.pack('<III', minimum, maximum, int(bool(minimum or maximum)))


def verify_annotation_projection() -> None:
    """Freeze every complete owned/native body, including all twenty older missing pairs; e.g. verify_annotation_projection()."""
    owned = [(family, version, raw) for family, version, _, raw in dialog_owned_cases() if family in (0, 2)]
    native = raw_style_native_contexts()
    assert len(owned) == 425
    packets = [struct.pack('<II', 1, len(cases)) + b''.join(annotation_context(*case) for case in cases) for cases in (owned, native)]
    present = [sum(annotation_observation(*case) is not None for case in cases) for cases in (owned, native)]
    assert present == [405, 276], present
    summaries = persist_dialog_packets(FEATURE.parents[2] / 'tests/fixtures/simulation-post-dialog', {'annotation-projection.bin': packets[0]})
    summaries.update(persist_dialog_packets(FEATURE.parents[2] / 'target', {'simulation-annotation-projection-native.bin': packets[1]}))
    dialog_baseline('annotation-projection-reference.json', dict(owned_contexts=425, owned_present=405, owned_missing=20,
        native_contexts=276, native_present=276, additional_stored_byte_credit=0, packets=summaries))
