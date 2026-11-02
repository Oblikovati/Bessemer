"""Supplementary raw identifier checks without historical serialization or unit inference (#175)."""
from typing import cast
from legacy_material_evidence import legacy_material_reports


def fresh_identifier_record(record: dict[str, object], matches: dict[str, object]) -> dict[str, object]:
    """Preserve all16 identifier bytes and the raw64 value around a bounded ASCII view; e.g. fresh_identifier_record(record, matches)."""
    identifier = bytes.fromhex(cast(str, record['identifier_raw_hex']))
    name = cast(str, record['ascii_prefix_view'])
    assert len(identifier) == 16 and name in matches, f'{identifier.hex()}/{name}; expected a retained16-byte candidate identifier'
    stop = identifier.find(b'\0')
    assert stop >= 0 and identifier[:stop] == name.encode('ascii'), f'{identifier.hex()}; expected the original bounded prefix {name}'
    return dict(offset=record['offset'], identifier_raw_hex=identifier.hex(), ascii_prefix_view=name,
        installed_literal_match=matches[name], nonzero_bytes_after_first_nul=any(identifier[stop + 1:]),
        value_raw_hex=record['value_raw_hex'])


def fresh_identifier_row(row: dict[str, object]) -> dict[str, object]:
    """Reproduce only the original C895 corroboration fields from freshly parsed candidates; e.g. fresh_identifier_row(row)."""
    report = legacy_material_reports()[2]
    candidate = cast(dict[str, object], row['candidate'])
    records = cast(list[dict[str, object]], candidate['records'])
    matches = cast(dict[str, object], report['literal_matches'])
    return {key: row[key] for key in ('path', 'stream', 'sha256', 'retained_versions')} | dict(
        candidate_eof=True, complete=False, records=[fresh_identifier_record(record, matches) for record in records])


def verify_fresh_legacy_identifiers(rows: list[dict[str, object]]) -> None:
    """Keep every original identifier/suffix/pointer corroboration outcome exact; e.g. verify_fresh_legacy_identifiers(rows)."""
    report = legacy_material_reports()[2]
    actual = [fresh_identifier_row(row) for row in rows]
    assert actual == report['rows'], 'Fresh identifier records differ; expected every original C895 outcome'
    records = [record for row in actual for record in cast(list[dict[str, object]], row['records'])]
    assert len(records) == 196, f'{len(records)}; expected196 retained fixed records'
    suffixes = sum(cast(bool, record['nonzero_bytes_after_first_nul']) for record in records)
    assert suffixes == 182, f'{suffixes}; expected182 retained nonzero suffixes'
