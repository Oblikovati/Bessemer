"""Project reference-only metadata without changing MFC graph identity (#45)."""


def physical(value: object, memo: dict[int, object] | None = None) -> object:
    """Keep stored fields and aliases; e.g. physical({'raw_08': 3})."""
    if not isinstance(value, (dict, list)):
        return value
    if memo is None:
        memo = {}
    identity = id(value)
    if identity in memo:
        return memo[identity]
    if isinstance(value, dict):
        return record_fields(value, memo)
    projected: list[object] = []
    memo[identity] = projected
    projected.extend(physical(child, memo) for child in value)
    return projected


def record_fields(value: dict[str, object], memo: dict[int, object]) -> object:
    """Remove constructor labels; e.g. record_fields({'member': '08', 'raw_10': 3}, {})."""
    if 'member' in value and 'data' in value:
        projected = physical(value['data'], memo)
        memo[id(value)] = projected
        return projected
    projected_record: dict[str, object] = {}
    memo[id(value)] = projected_record
    metadata = {'member','kind','start','end','unknown_bytes','type'} if 'member' in value else set()
    for key, child in value.items():
        if key not in metadata and key != 'character_format_start':
            projected_record[key] = physical(child, memo)
    return projected_record


def prune_compound_kernel(value: object, seen: set[int]) -> None:
    """Keep suppression fields; typed kernel nodes have their separate independent oracle."""
    if not isinstance(value, (dict, list)) or id(value) in seen:
        return
    seen.add(id(value))
    if isinstance(value, list):
        for child in value:
            prune_compound_kernel(child, seen)
        return
    for key, child in list(value.items()):
        if key in ('embedded_body', 'body_a8', 'kernel_body') and isinstance(child, dict):
            if 'suppressed' not in child:
                raise ValueError(f'kernel field {key!r}: {child!r}; expected a suppression field')
            value[key] = {'suppressed': child['suppressed']}
        else:
            prune_compound_kernel(child, seen)
