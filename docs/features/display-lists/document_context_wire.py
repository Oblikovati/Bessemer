"""Independent filler-free version-4 containers for document context tests."""
from io import BytesIO
import struct
import zipfile


def swap_name(raw: bytearray, offset: int, length: int) -> None:
    for index in range(offset,offset+length):
        value = raw[index]
        raw[index] = ((value<<4)|(value>>4))&255


def rewrite_record(raw: bytearray, local: int, central: int) -> int:
    raw[local:local+4] = b"loc!"
    struct.pack_into("<HHH",raw,local+4,20,6,8)
    name_length = struct.unpack_from("<H",raw,local+26)[0]
    swap_name(raw,local+30,name_length)
    raw[central:central+4] = b"dir!"
    struct.pack_into("<HHHH",raw,central+4,0,20,6,8)
    name_length,extra,comment = struct.unpack_from("<HHH",raw,central+28)
    swap_name(raw,central+46,name_length)
    return central+46+name_length+extra+comment


def document(streams: list[tuple[str,bytes]]) -> bytes:
    memory = BytesIO()
    with zipfile.ZipFile(memory,"w",compression=zipfile.ZIP_DEFLATED) as archive:
        for name,payload in streams:
            info = zipfile.ZipInfo(name,(2026,1,1,0,0,0))
            archive.writestr(info,payload,compress_type=zipfile.ZIP_DEFLATED)
        records = archive.infolist()
    raw = bytearray(memory.getvalue())
    end = len(raw)-22
    central = struct.unpack_from("<I",raw,end+16)[0]
    for record in records:
        central = rewrite_record(raw,record.header_offset,central)
    raw[end:end+4] = b"end!"
    return struct.pack(">II",0x12345678,4)+raw
