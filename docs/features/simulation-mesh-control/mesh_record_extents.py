"""Complete independent proper-prefix and trailing-byte reference acceptance (#161)."""
from mesh_record_packets import actual_mesh_records
from mesh_control_evidence import mesh_baseline
from owned_mesh_records import owned_mesh_records


def verify_mesh_record_extents() -> None:
    """Check every shorter physical extent without sampling or context repair; e.g. verify_mesh_record_extents()."""
    cases = owned_mesh_records()
    checked = 0
    for case in cases:
        raw = case['raw']
        for end in range(len(raw)):
            assert not actual_mesh_records(raw[:end], case['version'])['success'], (case['version'], case['profile'], end)
            checked += 1
        assert not actual_mesh_records(raw + b'\0', case['version'])['success'], (case['version'], case['profile'])
    mesh_baseline('records-extent-reference.json', dict(contexts=len(cases), proper_prefixes=checked,
                  accepted_prefixes=0, trailing_byte_rejections=len(cases)))
