"""Complete roots with all five old-light classes at their early state-word gates."""
import generate_earliest_visuals as roots
from earliest_light_wire import replace_light,KINDS
from generate_earlier_range import write_group

ORIGINAL_SOURCE = roots.rv_source
ORIGINAL_ENTRIES = roots.entry_count


def generate(version: int, kind: str, scenario: int) -> tuple[bytes,dict[str,object]]:
    light = scenario%32//2
    roots.rv_source = lambda document,path,raw:replace_light(document,path,ORIGINAL_SOURCE(document,path,raw),light)
    roots.entry_count = lambda v,k,s:ORIGINAL_ENTRIES(v,k,s)+4
    try:framed,row = roots.generate(version,kind,0)
    finally:roots.rv_source,roots.entry_count = ORIGINAL_SOURCE,ORIGINAL_ENTRIES
    row["scenario"],row["light_kind"] = 224+2*light,KINDS[light]
    return framed[:5]+bytes([row["scenario"]])+framed[6:],row


if __name__ == "__main__":
    cases = [generate(v,k,s) for v in [3196,0xc90,0xc91,0xc9c,0xc9d,0xc9e,0xc9f,0xca5,3238,0xcc0,0xcc1,0xd01,0xd02] for k in ["part","assembly"] for s in [0,2,4,6,8]]
    print(write_group("earliest-light-state-kinds",cases,("displaylists.py",)))
