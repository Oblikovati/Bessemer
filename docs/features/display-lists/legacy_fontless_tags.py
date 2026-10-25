"""Relocate only observed MFC tags when an owned root omits its font-manager subtree."""
import struct


def relocated_index(index: int, first: int, last: int) -> int:
    assert not first<=index<last,(index,first,last)
    return index-(last-first) if index>=last else index


def relocated_tag(raw: bytes, position: int, first: int, last: int, extended_form: bool) -> tuple[int,int,bytes] | None:
    word, = struct.unpack_from("<H",raw,position)
    if word in (0,0xffff):return None
    if word==0x7fff:
        extended, = struct.unpack_from("<I",raw,position+2)
        index = relocated_index(extended&0x7fffffff,first,last)
        return position+2,position+6,struct.pack("<I",(extended&0x80000000)|index)
    index = relocated_index(word&0x7fff,first,last)
    if extended_form:
        flag = 0x80000000 if word&0x8000 else 0
        return position,position+2,struct.pack("<HI",0x7fff,flag|index)
    return position,position+2,struct.pack("<H",(word&0x8000)|index)


def remove_font_manager(raw: bytes, root: dict[str,object], objects: list[tuple[int,int,int,int,object]], patches: list[tuple[int,int,bytes]], extended_form: bool) -> None:
    manager = root["line_font_mgr"]
    removed = min((row for row in objects if row[4] is manager),key=lambda row:row[0])
    start,end,first,last,_ = removed
    assert (first,last)==(3,46),(first,last)
    patches.append((start,end,b""))
    rewrite_font_tags(raw,objects,(start,end,first,last),patches,extended_form)


def rewrite_font_tags(raw: bytes, objects: list[tuple[int,int,int,int,object]], removed: tuple[int,int,int,int], patches: list[tuple[int,int,bytes]], extended_form: bool) -> None:
    start,end,first,last = removed
    for position,_,_,_,_ in objects:
        if start<=position<end:continue
        replacement = relocated_tag(raw,position,first,last,extended_form)
        if replacement is not None:patches.append(replacement)
