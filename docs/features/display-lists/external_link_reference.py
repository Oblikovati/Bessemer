"""Independent native-reader events and block spans for exact external facet link expectations."""
import struct
from generate_alias_fixture import swcontainer,displaylists
from mfcobjects import ObjectArchive
import facettess

ResolvedFace = tuple[str,str,str,int,int,dict[str,object]]


def face_reader(original: object, records: dict[int,tuple[str,int]]) -> object:
    def read(archive: object, schema: int) -> object:
        placeholder = archive.map[-1]
        result = original(archive,schema)
        if result.get("facet_container_from")=="FaceTessellations":
            cursor = archive.link_reference_cursor
            records[id(placeholder)] = (cursor[0],cursor[1])
            cursor[1] += 1
        return result
    return read


def part_reader(original: object) -> object:
    def read(archive: object, schema: int) -> object:
        previous = getattr(archive,"link_reference_cursor",None)
        archive.link_reference_cursor = [getattr(archive,"part_comp_info",None),0]
        try:return original(archive,schema)
        finally:archive.link_reference_cursor = previous
    return read


def read_display(document: object, path: str) -> tuple[dict[str,object],dict[int,tuple[str,int]]]:
    names = ["uoTempPartTessData_c","uoTempFaceTessData_c"]
    originals = [ObjectArchive.readers[name] for name in names]
    requests: dict[int,tuple[str,int]] = {}
    readers = [part_reader(originals[0]),face_reader(originals[1],requests)]
    try:
        for name,reader in zip(names,readers,strict=True):ObjectArchive.readers[name] = reader
        archive = displaylists.archive_for(document,path,document.read("Contents/DisplayLists"))
        root = displaylists.display_lists(archive,archive.doc_kind)
        assert archive.p==len(archive.b)
    finally:
        for name,reader in zip(names,originals,strict=True):ObjectArchive.readers[name] = reader
    return root,requests


def request_paths(path: str, value: object, requests: dict[int,tuple[str,int]], seen: set[int], rows: list[tuple[str,str,int]]) -> None:
    if isinstance(value,dict):
        if id(value) in seen:return
        seen.add(id(value))
        if id(value) in requests:
            identity,index = requests[id(value)]
            rows.append((path+".facet_container",identity,index))
        for key,child in value.items():
            if not key.startswith("_") and key!="facet_container":
                suffix = ".components" if key=="tessellation" and isinstance(child,list) else ""
                request_paths(path+"."+key+suffix,child,requests,seen,rows)
    elif isinstance(value,(list,tuple)):
        for index,child in enumerate(value):request_paths(f"{path}[{index}]",child,requests,seen,rows)


def capture_facet(original: object, spans: dict[int,bytes]) -> object:
    def read(archive: object) -> object:
        start = archive.p
        result = original(archive)
        spans[id(result)] = archive.b[start:archive.p]
        return result
    return read


def read_blocks(document: object) -> tuple[dict[str,list[dict[str,object]]],dict[int,bytes]]:
    original = facettess.facet_container
    spans: dict[int,bytes] = {}
    blocks: dict[str,list[dict[str,object]]] = {}
    facettess.facet_container = capture_facet(original,spans)
    try:
        for name in sorted(document.names()):
            if name.startswith("FaceTessellations/") and name!="FaceTessellations/Directory":
                blocks[name.split("/",1)[1]] = facettess.block(document.read(name),19000)
    finally:facettess.facet_container = original
    return blocks,spans


def blob(raw: bytes) -> bytes:
    return struct.pack("<I",len(raw))+raw


def target(entry: dict[str,object]) -> tuple[str,int,int]:
    return entry["face_tess_block_name"],entry["index_in_block"],entry["hash_key"]


def named_links(entries: list[dict[str,object]], blocks: dict[str,list[dict[str,object]]]) -> tuple[int,dict[tuple[str,int],dict[str,object]]]:
    indexed = {(name,component["index_in_block"]):component for name,components in blocks.items() for component in components}
    assert len(indexed)==sum(len(components) for components in blocks.values())
    for entry in entries:
        name,index,key = target(entry)
        if not name:continue
        if name not in blocks:return 1,indexed
        if (name,index) not in indexed or indexed[name,index]["hash_key"]!=key:return 2,indexed
    return 0,indexed


def resolve_requests(rows: list[tuple[str,str,int]], entries: list[dict[str,object]], indexed: dict[tuple[str,int],dict[str,object]]) -> tuple[int,list[ResolvedFace],str]:
    targets: dict[str,set[tuple[str,int,int]]] = {}
    for entry in entries:targets.setdefault(entry["comp_id"],set()).add(target(entry))
    resolved = []
    for path,identity,index in rows:
        options = targets.get(identity,set())
        if not options:return 1,[],identity
        if len(options)!=1:return 3,[],identity
        name,component_index,_ = next(iter(options))
        if not name:return 1,[],identity
        component = indexed[name,component_index]
        if index>=len(component["faces"]):return 2,[],identity
        resolved.append((path,identity,name,component_index,index,component["faces"][index]))
    return 0,resolved,""


def complete_report(entries: list[dict[str,object]], blocks: dict[str,list[dict[str,object]]], rows: list[ResolvedFace], spans: dict[int,bytes]) -> tuple[bytes,tuple[int,...]]:
    named = [target(entry)[:2] for entry in entries if entry["face_tess_block_name"]]
    unique = {(name,component,index) for _,_,name,component,index,_ in rows}
    available_components = sum(map(len,blocks.values()))
    available_faces = sum(len(component["faces"]) for components in blocks.values() for component in components)
    counts = (len(blocks),len(named),len(entries)-len(named),available_components,available_components-len(set(named)),len(rows),len(unique),available_faces,available_faces-len(unique))
    raw = b"\0"+struct.pack("<9QI",*counts,len(rows))
    for path,identity,name,component,index,facet in rows:
        raw += blob(path.encode())+blob(identity.encode())+blob(name.encode())+struct.pack("<iI",component,index)+blob(spans[id(facet)])
    return raw,counts


def expected(path: str) -> dict[str,object]:
    document = swcontainer.Document(path)
    entries = facettess.directory(document.read("FaceTessellations/Directory"),19000)["entries"]
    root,requests = read_display(document,path)
    rows: list[tuple[str,str,int]] = []
    request_paths("root",root,requests,set(),rows)
    assert len(rows)==len(requests)
    blocks,spans = read_blocks(document)
    status,indexed = named_links(entries,blocks)
    resolved,identity = [],""
    if status==0:status,resolved,identity = resolve_requests(rows,entries,indexed)
    raw,counts = complete_report(entries,blocks,resolved,spans) if status==0 else (b"",())
    return {"status":status,"raw":raw,"coverage":counts,"identity":identity,"requested_faces":len(rows),"directory_entries":len(entries),"blocks":len(blocks),"unlocated_entries":sum(not entry["face_tess_block_name"] for entry in entries)}
