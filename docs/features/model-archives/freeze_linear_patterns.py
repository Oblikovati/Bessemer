"""Complete linear-pattern expectations from separately assembled wires and the source reader."""
import hashlib
import json
from functools import partial
from freeze_views import WORKSPACE, REFERENCE, view_oracle
from freeze_blank_part import reference_snapshot
from mfcobjects import ObjectArchive
import bodyfeatures
import featurepatterns
import sketchprefix
import pattern_fixture_bytes as fixture


def freeze(version: int, raw: bytes, mode: int=0) -> tuple[bytes,int]:
    archive=ObjectArchive(raw,version)
    archive.feature_context='ordinary_external_xml';archive.model_root_class='moPart_c'
    bodyfeatures.install_scoped(archive,overwrite=True)
    archive.readers['moTempAxisRef_w']=sketchprefix.temporary_axis_reference
    readers={0:archive.object,1:partial(featurepatterns.shape_data,archive),
             2:partial(featurepatterns.increment,archive),3:partial(featurepatterns.original_dimension,archive),
             4:partial(featurepatterns.variation,archive),5:partial(featurepatterns.collections,archive)}
    value=readers[mode]()
    if archive.p!=len(raw):raise ValueError(f'linear end {archive.p}; expected {len(raw)}')
    oracle=view_oracle('root.object',value)
    return fixture.P('<BIII',mode,version,len(raw),len(oracle))+raw+oracle,len(oracle.splitlines())


def generate() -> None:
    before=reference_snapshot(REFERENCE);records=[]
    versions=[0x27cc,0x2f86,0x2f87,0x2f93,0x2f94,0x3745,0x3746,0x3eff,0x3f00,0x42ce,0x42cf,0x42db,0x42dc,19000]
    for version in versions:
        records.append(freeze(version,fixture.linear(version,True,4,True,temporary=True)))
        for rich in [False,True]:
            for kind in [0,1,2,3,4,5,-1]:
                for point in [False,True]:records.append(freeze(version,fixture.linear(version,rich,kind,point)))
        for kind in [0,1,2,3,4,5,-1]:
            for point in [False,True]:records.append(freeze(version,fixture.shape(kind,point),1))
        for rich in [False,True]:
            for mode,builder in [(2,fixture.increment),(3,fixture.original),(4,fixture.variation),(5,fixture.collections)]:
                records.append(freeze(version,builder(rich),mode))
    edits={f'linear-pattern-{name}.bin':fixture.linear(19000,True,4,True,word)
           for name,word in [('original',-2**31),('edited',12345)]}
    for raw in edits.values():freeze(19000,raw)
    if before!=reference_snapshot(REFERENCE):raise ValueError('source changed; expected stable inventory')
    packet=b''.join(raw for raw,_ in records)
    for name,raw in {'linear-pattern-cases.bin':packet,**edits}.items():
        (WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    result={'cases':len(records),'fields':sum(count for _,count in records),'sha256':hashlib.sha256(packet).hexdigest(),
            'source_hashes':before,'exclusions':[]}
    (WORKSPACE/'docs/features/model-archives/linear-pattern-cases.json').write_bytes((json.dumps(result)+'\n').encode())
    print({key:result[key] for key in ['cases','fields','sha256']})


if __name__=='__main__':generate()
