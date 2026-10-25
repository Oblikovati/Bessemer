"""Freeze semantic fields from the independent reader, separate from Rust reconstruction."""
from pathlib import Path
import hashlib
import json
import struct
import sys

REFERENCE = Path("../inventor-interface/solidworks").resolve()
sys.path.insert(0, str(REFERENCE))
import displaylists
import swcontainer


def flatten(path: str, value: object, rows: dict[str, str]) -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            if key != "_class":
                flatten(f"{path}.{key}", child, rows)
    elif isinstance(value, (list, tuple)):
        for index, child in enumerate(value):
            flatten(f"{path}[{index}]", child, rows)
    elif isinstance(value, str):
        rows[path] = "s:" + value.encode("utf-8").hex()
    elif isinstance(value, float):
        rows[path] = "f:" + struct.pack(">d", value).hex()
    elif isinstance(value, int):
        rows[path] = f"i:{value}"
    else:
        raise ValueError(f"{path}: {value!r}; expected scalar, record or sequence")


def fonts(root: dict[str, object]) -> dict[str, str]:
    rows: dict[str, str] = {}
    for index, (key, font) in enumerate(root["line_font_mgr"]["line_fonts"]):
        prefix = f"root.line_font_mgr.line_fonts[{index}]"
        flatten(f"{prefix}.key", key, rows)
        for name, value in font.items():
            if name == "weight":
                flatten(f"{prefix}.value", value, rows)
            elif name != "_class":
                flatten(f"{prefix}.value.{name}", value, rows)
    return rows


def tables(root: dict[str, object]) -> dict[str, dict[str, str]]:
    lights: dict[str, str] = {}
    flatten("root.old_lights.items", root["old_lights"], lights)
    materials: dict[str, str] = {}
    flatten("root.visual_properties", root["visual_properties"], materials)
    for index, (key, value) in enumerate(root["tessellation"]["model_viz_props"]):
        prefix = f"root.tessellation.model_viz_props[{index}]"
        flatten(f"{prefix}.key", key, materials)
        flatten(f"{prefix}.value", value, materials)
    return {"fonts": fonts(root), "lights": lights, "materials": materials}


def main() -> None:
    path = Path("tests/fixtures/blank-part.SLDPRT")
    document = swcontainer.Document(path)
    raw = document.read("Contents/DisplayLists")
    archive = displaylists.archive_for(document, str(path), raw)
    root = displaylists.display_lists(archive, archive.doc_kind)
    assert archive.p == len(raw)
    manifest = {"source_sha256": hashlib.sha256(raw).hexdigest(), "tables": {}}
    for name, rows in tables(root).items():
        payload = "".join(f"{key}={value}\n" for key, value in sorted(rows.items()))
        destination = Path(f"tests/fixtures/display-lists/blank-part-{name}.txt")
        destination.write_text(payload, encoding="utf-8", newline="\n")
        manifest["tables"][name] = {"fields": len(rows), "sha256": hashlib.sha256(payload.encode()).hexdigest()}
    Path(__file__).with_name("blank-scalar-oracle.json").write_text(json.dumps(manifest, indent=2)+"\n", encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
