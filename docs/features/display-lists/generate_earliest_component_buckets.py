"""Independent complete bucket roots before and after the BOS flag began being stored."""
import json
import struct
from generate_historical_buckets import generate as bucket_root
from generate_earlier_range import write_group
from generate_earliest_scene_roots import nan_payload64
from generate_blank_scalars import flatten
import bucket_corpus

VERSIONS = [4800,0x12dd,0x12de,4850]


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    original = bucket_corpus.walk
    bucket_corpus.walk = raw_points_reader(original)
    try:framed,row = bucket_root(version,kind,scenario%32)
    finally:bucket_corpus.walk = original
    row["scenario"] = 160+scenario%32
    return framed[:5]+bytes([row["scenario"]])+framed[6:],row


def raw_points_reader(original: object) -> object:
    def read(path: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
        if not isinstance(value,bytes) or not path.endswith(".points_f32"):
            return original(path,value,rows,seen)
        for index,(bits,) in enumerate(struct.iter_unpack("<I",value)):
            field = f"{path}[{index}]"
            if bits&0x7f800000==0x7f800000 and bits&0x7fffff:
                rows[field] = f"f:{nan_payload64(bits):016x}"
            else:flatten(field,struct.unpack("<f",struct.pack("<I",bits))[0],rows)
    return read


if __name__=="__main__":
    totals = []
    for dimension,scenario in [("3d",0),("2d",16)]:
        cases = [generate(v,"part",scenario) for v in VERSIONS]
        cases += [generate(4800,"part",scenario+1),generate(4800,"part",2)]
        totals.append(write_group("earliest-component-buckets-"+dimension,cases))
    print(json.dumps({"fixtures":sum(n for n,_ in totals),"fields":sum(n for _,n in totals)}))
