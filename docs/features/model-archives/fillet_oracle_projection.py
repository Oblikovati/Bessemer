"""Retain cached-body suppression while typed kernel oracles cover frame contents."""


def project(value: object, seen: set[int] | None = None) -> None:
    seen = set() if seen is None else seen
    if not isinstance(value, (dict, list, tuple)) or id(value) in seen:
        return
    seen.add(id(value))
    if not isinstance(value, dict):
        for child in value:
            project(child, seen)
        return
    if 'bodies' in value and 'face_edge_map' in value:
        value['bodies'] = [{'suppressed': body['suppressed']} for body in value['bodies']]
    for child in value.values():
        project(child, seen)
