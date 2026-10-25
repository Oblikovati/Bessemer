"""Version-specific view-prefix reconstruction from the owned blank part."""
from pathlib import Path
from collections.abc import Callable
from generate_alias_fixture import displaylists,swcontainer
from generate_version_range import capture_plain,apply_patches,Patch
from generate_spatial_vectors import collection
from early_shaded_wire import early_items
from generate_shaded_vectors import ShadedArchive
import dl_doc
import dl_light

Reader = Callable[[ShadedArchive],object]


def light_slot(original: Callable[[ShadedArchive],object], slots: list[tuple[int,int]]) -> Callable[[ShadedArchive],object]:
    def capture(archive: ShadedArchive) -> object:
        slots.append((archive.p-4,len(archive.map)))
        return original(archive)
    return capture


def environment_slot(archive: object, version: int, patches: list[Patch]) -> None:
    if version>=0x1ff1:
        return
    original = archive.object
    def capture(expect: object = None) -> object:
        result = original(expect)
        if isinstance(result,dict) and result.get("_class")=="uiUserModelEnv_c":
            patches.append((archive.p-4,archive.p,b""))
        return result
    archive.object = capture


def replacements(originals: list[Reader], version: int, patches: list[Patch], slots: list[tuple[int,int]]) -> list[Reader]:
    return [capture_plain(originals[0],b"",patches),light_slot(originals[1],slots),
            capture_plain(originals[2],early_items(version),patches),
            capture_plain(originals[3],b"",patches) if version<=0x1be7 else originals[3],
            capture_plain(originals[4],collection(version,2),patches),
            capture_plain(originals[5],b"",patches) if version<=0x1c39 else originals[5]]


def prefix(document: swcontainer.Document, path: Path, raw: bytes, version: int) -> tuple[bytes,int]:
    patches: list[Patch] = []
    slots: list[tuple[int,int]] = []
    bindings = [(dl_doc,"read_OrigFontScale"),(dl_light,"read_old_lights"),(dl_doc,"read_display_item_list"),(dl_doc,"read_NamedViewTransform"),(dl_doc,"read_3DViewTransform"),(dl_doc,"read_AnnotationViewTransform")]
    originals = [getattr(module,name) for module,name in bindings]
    try:
        for (module,name),replacement in zip(bindings,replacements(originals,version,patches,slots),strict=True):
            setattr(module,name,replacement)
        archive = displaylists.archive_for(document,str(path),raw)
        environment_slot(archive,version,patches)
        displaylists.display_lists(archive,archive.doc_kind)
        assert archive.p==len(raw) and len(slots)==1
    finally:
        for (module,name),original in zip(bindings,originals,strict=True):
            setattr(module,name,original)
    end,index = slots[0]
    return apply_patches(raw[:end],patches),index
