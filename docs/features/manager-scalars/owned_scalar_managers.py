"""Place scalar values in independently located complete CMgr maps (#54)."""
from pathlib import Path
import struct
from freeze_blank_part import source_module
from primitive_inventory import observe_primitives,verify_primitive_coverage
from owned_circular_fixtures import write_observation
from scalar_bytes import scalar_map
from freeze_owned_features import ReferenceDocument
from manager_tag_observation import observe_manager,relocated_tail


class SavedScalarManagerDocument:
    """Override only CMgr while retaining all other original document streams."""
    def __init__(self, original: ReferenceDocument, manager: bytes) -> None:
        self.original=original;self.manager=manager

    def names(self) -> list[str]:
        return self.original.names()

    def read(self, name: str) -> bytes:
        return self.manager if name=='Contents/CMgr' else self.original.read(name)


def owned_manager(reference: Path, destination: Path, kind: int, style: int, edited: bool) -> dict[str,object]:
    """Require exact CMgr EOF and map identities through the actual complete public reader."""
    root=Path(__file__).resolve().parents[3]
    container,manager,archive,objects=(source_module(name,reference) for name in ('swcontainer','configurationmanager','swarchive','mfcobjects'))
    original=container.Document((root/'tests/fixtures/blank-part.SLDPRT').read_bytes(),table={})
    baseline,events=observe_manager(original,manager,objects.ObjectArchive.object.__code__);raw=original.read('Contents/CMgr')
    position=baseline['root']['state_650_position']+4 if kind==0 else baseline['root']['manager_maps_start']
    assert raw[position:position+4]==bytes(4)
    first_slot=next(size for offset,size in events if offset>position)
    entries,slots=scalar_map(kind,19000,style,edited,first_slot-1)
    added=slots-first_slot;relocated=relocated_tail(raw,events,position,first_slot,added)
    changed=relocated[:position]+entries+relocated[position+4:];document=SavedScalarManagerDocument(original,changed)
    with observe_primitives(archive.Archive,changed) as reads:parsed=manager.read_supported(document)
    assert parsed['end']==len(changed) and len(parsed['archive'].map)==len(baseline['archive'].map)+added
    values=parsed['root']['map_640' if kind==0 else 'map_5f8']['entries'];field='pair' if kind==0 else 'material'
    assert values[0][field] is values[2][field] and values[0][field] is not values[1][field]
    verify_primitive_coverage(changed,reads)
    label=f'owned-{kind}-{style}'+('-edited' if edited else '')
    return write_observation(destination,label,changed,[r for r in reads if r.end>r.start])|dict(slots=len(parsed['archive'].map),map_position=position,inserted_slots=added)
