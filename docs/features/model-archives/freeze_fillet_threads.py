"""Complete scoped fillet/thread oracles; preloaded instance/cache targets are explicit."""
from pathlib import Path
import hashlib,json
from freeze_views import WORKSPACE,REFERENCE,view_oracle
from freeze_blank_part import reference_snapshot
import fillet_fixture_bytes as fillet
import thread_fixture_bytes as thread
import bodyfeatures,fillets,cosmeticthreads,displaydimensions
from mfcobjects import ObjectArchive
from fillet_oracle_projection import project

def freeze(mode,version,raw):
    loaded=thread.LOADED if mode&0x80 else []
    a=ObjectArchive(raw,version,inherited_map=[None]+[{'_class':name} for name in loaded])
    a.feature_context='ordinary_external_xml';a.model_root_class='moPart_c'
    bodyfeatures.install_scoped(a,overwrite=True);displaydimensions.install_scoped(a)
    a.readers.update(moFilletSIdKeeper_c=fillets.surface_id_keeper,moFaceFilletSIdKeeper_c=fillets.surface_id_keeper,
        edgeRadiusObject_c=fillets.edge_radius,faceRadiusObject_c=fillets.face_radius,loopRadiusObject_c=fillets.loop_radius,
        moCThreadData_c=cosmeticthreads.data)
    value=a.object();project(value)
    if a.p!=len(raw):raise ValueError(f'fillet/thread end {a.p}; expected {len(raw)}')
    oracle=view_oracle('root.object',value)
    return fillet.P('<BIII',mode,version,len(raw),len(oracle))+raw+oracle,len(oracle.splitlines())

def generate():
    before=reference_snapshot(REFERENCE);records=[]
    versions=[0x27cc,0x27cf,0x27d0,0x27de,0x27df,0x2b6a,0x2b6b,0x2b8e,0x2b8f,0x2f6a,0x2f6b,0x3366,0x3367,
              0x3379,0x337a,0x3748,0x3749,0x39d0,0x39d1,17000,17001,19000]
    for version in versions:
        records.append(freeze(0,version,fillet.fillet(version,False)))
        records += [freeze(0,version,fillet.fillet(version,True,kernel)) for kernel in [0,1]]
        for rich in [False,True]:
            records.append(freeze(0,version,fillet.variable(version,rich)))
            for name in ['moFilletSIdKeeper_c','moFaceFilletSIdKeeper_c','edgeRadiusObject_c','faceRadiusObject_c','loopRadiusObject_c','moEdgeDim_c','moVertDim_c']:
                records.append(freeze(0,version,fillet.support(version,name,rich)))
            records.append(freeze(0x80 if rich else 0,version,thread.feature(version,rich)))
        for count in [0,1,2]:
            for point in [0,1,2,255]:
                for active in [False,True]:
                    records.append(freeze(0x80 if count else 0,version,thread.tagged_data(version,count,point,active,bool(count))))
    edits={f'fillet-thread-{name}.bin':fillet.fillet(19000,True,1,word) for name,word in [('original',-2**31),('edited',12345)]}
    for raw in edits.values():freeze(0,19000,raw)
    if before!=reference_snapshot(REFERENCE):raise ValueError('reference changed; expected stable source hashes')
    packet=b''.join(raw for raw,_ in records)
    for name,raw in {'fillet-thread-cases.bin':packet,**edits}.items():(WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    report={'cases':len(records),'fields':sum(n for _,n in records),'sha256':hashlib.sha256(packet).hexdigest(),'source_hashes':before,
            'loaded_scope':thread.LOADED,'exclusions':['kernel nodes have separate typed oracle; cached-body suppression remains included']}
    Path(__file__).with_name('fillet-thread-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({k:v for k,v in report.items() if k not in ['source_hashes','loaded_scope','exclusions']})

if __name__=='__main__':generate()
