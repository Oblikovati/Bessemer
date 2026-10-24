"""Derive an archive cycle to test MFC identity; this is not native model acceptance."""
from pathlib import Path
import hashlib
import json
import struct
from generate_alias_fixture import collect, displaylists, swcontainer
from generate_blank_scalars import flatten


def derive(document: swcontainer.Document, path: Path, raw: bytes) -> tuple[bytes, int, int]:
    records = collect(document, path, raw)
    start, end, identity, _ = next(record for record in records if record[3] == "uoTempSubAssemblySHDData_c")
    owner, owner_end, _, _ = max(record for record in records if start < record[0] < end and record[3] == "")
    assert raw[owner:owner_end] == b"\0\0"
    derived = raw[:owner]+struct.pack("<H", identity)+raw[owner_end:]
    archive = displaylists.archive_for(document, str(path), derived)
    root = displaylists.display_lists(archive, archive.doc_kind)
    assert archive.p == len(derived)
    component = root["tessellation"][0]
    assert component["owner"] is component
    return derived, identity, owner


def scalar_oracle(document: swcontainer.Document, path: Path, raw: bytes, destination: Path) -> int:
    archive = displaylists.archive_for(document, str(path), raw)
    root = displaylists.display_lists(archive, archive.doc_kind)
    component = dict(root["tessellation"][0]["viewer_comp_data"])
    # Geometry has separate bit-exact transform and reference-plane oracles.
    for key in ["comp_xform", "comp_bbox", "comp_viz_prop", "ref_geom_data_collector"]:
        del component[key]
    rows: dict[str, str] = {}
    dependencies = component.pop("equ_dep_comp_names")
    for index, (key, values) in enumerate(dependencies):
        prefix = f"root.tessellation.components[0].viewer_comp_data.equ_dep_comp_names[{index}]"
        flatten(f"{prefix}.key", key, rows)
        flatten(f"{prefix}.values", values, rows)
    flatten("root.tessellation.components[0].viewer_comp_data", component, rows)
    flatten("root.envelope_system_options", root["envelope_system_options"], rows)
    payload = "".join(f"{key}={value}\n" for key, value in sorted(rows.items()))
    destination.write_text(payload, encoding="utf-8", newline="\n")
    return len(rows)


def main() -> None:
    path = Path("tests/fixtures/blank-assembly.SLDASM")
    document = swcontainer.Document(str(path))
    raw = document.read("Contents/DisplayLists")
    derived, identity, offset = derive(document, path, raw)
    Path("tests/fixtures/display-lists/owner-cycle-assembly.bin").write_bytes(derived)
    fields = scalar_oracle(document, path, raw, Path("tests/fixtures/display-lists/blank-assembly-component.txt"))
    evidence = {"reason": "independent self-owner graph regression; no model/application acceptance claim", "source_sha256": hashlib.sha256(raw).hexdigest(), "derived_sha256": hashlib.sha256(derived).hexdigest(), "bytes": len(derived), "object_index": identity, "owner_offset": offset, "scalar_oracle_fields": fields}
    Path(__file__).with_name("owner-cycle-fixture.json").write_text(json.dumps(evidence, indent=2)+"\n", encoding="utf-8", newline="\n")
    print(json.dumps(evidence))


if __name__ == "__main__":
    main()
