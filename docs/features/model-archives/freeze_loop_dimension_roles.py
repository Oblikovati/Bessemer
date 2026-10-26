"""Independent complete display-dimension bodies inside the direct sketch loop manager (#46)."""
import hashlib
import json
from freeze_views import WORKSPACE, REFERENCE, view_oracle
from freeze_blank_part import reference_snapshot
from freeze_display_dimensions import install
from mfcobjects import ObjectArchive
from sketch_tail_fixture_bytes import P
import display_dimension_fixture_bytes as dimension
import sketchtail


def generate() -> None:
    before=reference_snapshot(REFERENCE); records=[]; fields=0
    for name in dimension.CLASSES:
        prefix=P('<9id',1,-1,2**31-1,0,-2**31,2**31-1,-1,0,-1,-0.0)
        raw=prefix+dimension.tagged(19000,name,False)
        source=ObjectArchive(raw,19000)
        source.feature_context='ordinary_external_xml';source.model_root_class='moPart_c'
        install(source)
        value=sketchtail.loop_manager(source)
        if source.p!=len(raw):raise ValueError(f'loop end {source.p}; expected {len(raw)}')
        oracle=view_oracle('root',value)
        records.append(P('<BIII',0,19000,len(raw),len(oracle))+raw+oracle)
        fields+=len(oracle.splitlines())
    if before!=reference_snapshot(REFERENCE):raise ValueError('source changed; expected stable inventory')
    packet=b''.join(records)
    (WORKSPACE/'tests/fixtures/model-archives/loop-dimension-role-cases.bin').write_bytes(packet)
    result={'cases':len(records),'fields':fields,'sha256':hashlib.sha256(packet).hexdigest(),'source_hashes':before}
    (WORKSPACE/'docs/features/model-archives/loop-dimension-role-cases.json').write_bytes((json.dumps(result)+'\n').encode())
    print({key:result[key] for key in ['cases','fields','sha256']})


if __name__=='__main__':generate()
