"""Compose complete condition-200 decal maps inside actual CMgr streams (#54)."""
from pathlib import Path
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from owned_scalar_managers import SavedScalarManagerDocument
from manager_tag_observation import observe_manager,relocated_tail
from decal_bytes import decal_map


def owned_decal(reference: Path, destination: Path, kind: int, style: int, edited: bool) -> dict[str,object]:
    """Use the actual supported-manager endpoint and relocate only observed MFC tags."""
    root=Path(__file__).resolve().parents[3]
    container,manager,archive,objects=(source_module(name,reference) for name in ('swcontainer','configurationmanager','swarchive','mfcobjects'))
    original=container.Document((root/'tests/fixtures/blank-part.SLDPRT').read_bytes(),table={});raw=original.read('Contents/CMgr')
    baseline,events=observe_manager(original,manager,objects.ObjectArchive.object.__code__)
    position=baseline['root']['manager_owned_start'];assert raw[position:position+8]==bytes(8)
    # With no later object calls, the completed map is also the insertion map.
    first_slot=next((size for offset,size in events if offset>position),len(baseline['archive'].map))
    payload,slots=decal_map(kind,19000,style,edited,first_slot-1);added=slots-first_slot
    relocated=relocated_tail(raw,events,position,first_slot,added);changed=relocated[:position]+payload+relocated[position+8:]
    with observe_primitives(archive.Archive,changed) as reads:parsed=manager.read_supported(SavedScalarManagerDocument(original,changed))
    assert parsed['end']==len(changed) and len(parsed['archive'].map)==len(baseline['archive'].map)+added
    verify_primitive_coverage(changed,reads);label=f'owned-{kind}-{style}'+('-edited' if edited else '')
    return write_observation(destination,label,changed,[r for r in reads if r.end>r.start])|dict(slots=len(parsed['archive'].map),map_position=position,inserted_slots=added)
