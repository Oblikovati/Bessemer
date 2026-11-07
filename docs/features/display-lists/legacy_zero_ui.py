"""Observed early pane, bucket and temporary edge omissions in complete roots."""
import struct
from legacy_edge_roots import span_reader
from legacy_root_item_wire import item_span_reader
from generate_component_vectors import integers
import dl_ui
import dl_doc


def zero_pane_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        saved,transforms = dl_ui.read_mgXform_c,[]
        dl_ui.read_mgXform_c = span_reader(saved,transforms)
        try:result = original(archive)
        finally:dl_ui.read_mgXform_c = saved
        assert len(transforms)==2*len(result["panes"])
        for index,pane in enumerate(result["panes"]):
            position = transforms[2*index+1][1]+2
            assert archive.b[position:position+8]==struct.pack("<d",pane["perspective"])
            patches.append((position,position+8,b""))
        return result
    return read


def zero_edge_reader(original: object, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object) -> object:
        first = archive.p
        result = original(archive)
        assert archive.b[first+16:first+20]==struct.pack("<i",result["unknown_c"])
        patches.append((first+16,first+20,b""))
        return result
    return read


def minimal_edge_array(version: int, edited: bool) -> bytes:
    record = integers(-1,2,-3,23 if edited else -8)
    if version>561:record += integers(5)
    return integers(128)+(record+integers(0))*128


def zero_bucket_reader(original: object, version: int, scenario: int, patches: list[tuple[int,int,bytes]]) -> object:
    def read(archive: object, flag: int, keys: int) -> object:
        saved = dl_doc.read_display_item_list,dl_ui.read_gcEdgeVisData_c
        items,edges = [],[]
        dl_doc.read_display_item_list = item_span_reader(saved[0],items)
        if scenario in (188,189):dl_ui.read_gcEdgeVisData_c = span_reader(saved[1],edges)
        try:result = original(archive,flag,keys)
        finally:dl_doc.read_display_item_list,dl_ui.read_gcEdgeVisData_c = saved
        assert len(items)==1 and items[0][2] is result["dl_bucket"]
        if version<=595:patches.append((items[0][0],items[0][1],b""))
        if scenario in (188,189):
            last = items[0][0]
            first = edges[0][0]-4 if edges else last-4
            assert max(struct.unpack_from("<i",archive.b,first)[0],0)==len(result["edge_vis_data"])
            patches.append((first,last,minimal_edge_array(version,scenario==189)))
        return result
    return read
