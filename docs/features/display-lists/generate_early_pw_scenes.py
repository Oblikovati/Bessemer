"""Freeze complete roots proving the early PW scene caller path and field gates (#41)."""
from functools import partial
import hashlib
from early_pw_scene_roots import early_pw_root
from legacy_component_reference import component_reference,activity_scalars
from generate_legacy_visuals import visual_scalars
from generate_blank_scalars import flatten
from complete_root_case import frame_case
from generate_earlier_range import write_group
from generate_earliest_components import write_documents
import bucket_corpus

VERSIONS = [2771,3196,3575,3576,3595,3596,3600,3601,3897,3898,3934,3935,4300]


def pw_wall_member(original: object, version: int) -> object:
    def read(path: str, key: str, value: object, rows: dict[str,str], seen: set[int]) -> None:
        if key!="wall_info" or ".view_bucket." not in path:return original(path,key,value,rows,seen)
        for index,(word,wall) in enumerate(value):
            prefix = f"{path}.wall_info[{index}]"
            if version>=3576:flatten(prefix+".key",word,rows)
            bucket_corpus.walk(prefix+".value",wall,rows,seen)
    return read


def generate(version: int, kind: str, scenario: int = 224) -> tuple[bytes,dict[str,object]]:
    source,raw = early_pw_root(version,kind,scenario)
    root = component_reference(raw,version,kind,())
    original = bucket_corpus.member
    bucket_corpus.member = pw_wall_member(original,version)
    try:framed,row = frame_case(version,kind,scenario,raw,(),root,visual_scalars(root)+activity_scalars(root))
    finally:bucket_corpus.member = original
    row["source_sha256"] = hashlib.sha256(source).hexdigest()
    return framed,row


if __name__=="__main__":
    records = [generate(version,kind,scenario) for version in VERSIONS
        for kind in ("part","assembly") for scenario in range(224,229)]
    print(write_group("early-pw-scene-roots",records,("displaylists.py",)))
    write_documents(generate,2771,"early-pw-scene")
    write_documents(partial(generate,scenario=225),2771,"early-pw-scene-edited")
