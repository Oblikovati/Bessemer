"""Independent slot identities for the model archive's shared MFC map."""


def reference_slot(index: int, slot: object) -> dict[str, object]:
    """Validate a reference map entry; e.g. reference_slot(1, ('class', 'moPart_c', 1))."""
    if slot is None:
        return {"index": index, "kind": "null"}
    if isinstance(slot, dict) and isinstance(slot.get("_class"), str) and slot["_class"]:
        return {"index": index, "kind": "object", "class_name": slot["_class"]}
    if isinstance(slot, tuple) and len(slot) == 3 and slot[0] == "class":
        name, schema = slot[1:]
        if isinstance(name, str) and name and type(schema) is int and 0 <= schema <= 65535:
            return {"index": index, "kind": "class", "class_name": name, "schema": schema}
    raise ValueError(f"MFC slot {index}: {str(slot)[:120]}; expected null, named object or ('class', name, u16 schema)")


def map_slot_inventory(slots: list[object]) -> list[dict[str, object]]:
    """Keep repeated imported object identities; e.g. map_slot_inventory([None, object_record])."""
    if not slots or slots[0] is not None:
        raise ValueError(f"MFC map length {len(slots)}; expected null slot at index zero")
    identities: dict[int, int] = {}
    entries: list[dict[str, object]] = []
    for index, slot in enumerate(slots):
        entry = reference_slot(index, slot)
        if isinstance(slot, dict):
            identity = id(slot)
            entry["identity_slot"] = identities.setdefault(identity, index)
        entries.append(entry)
    return entries
