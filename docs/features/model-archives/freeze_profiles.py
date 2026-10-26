"""Independent complete profiles, contours, chains and framed-geometry boundaries."""
from pathlib import Path
import hashlib,json
from freeze_views import REFERENCE,WORKSPACE,view_oracle
from freeze_blank_part import reference_snapshot
from freeze_sketch_tail import remove_derived
from freeze_sketch_tail import install as install_sketch
from profile_oracle_projection import project
import profile_fixture_bytes as wire
import bodyfeatures,contours
from mfcobjects import ObjectArchive

def freeze(mode,version,raw):
    a=ObjectArchive(raw,version)
    a.feature_context='ordinary_external_xml';a.model_root_class='moPart_c'
    bodyfeatures.install_scoped(a,overwrite=True);install_sketch(a)
    a.readers['moSketchChain_c']=bodyfeatures.sketch_chain
    if mode==0:value=a.object()
    elif mode==1:value=bodyfeatures.profile_geometry(a)
    else:value=bodyfeatures.profile_reference_array(a)
    if a.p!=len(raw):raise ValueError(f'profile mode {mode}, end {a.p}; expected {len(raw)}')
    remove_derived(value);project(value)
    oracle=view_oracle('root.object' if mode==0 else 'root',value)
    return wire.P('<BIII',mode,version,len(raw),len(oracle))+raw+oracle,len(oracle.splitlines())

def generate():
    before=reference_snapshot(REFERENCE);records=[]
    for version in [0x27cc,0x2b91,0x2b92,19000]:
        for name in ['moProfileFeature_c','mo3DProfileFeature_c']:
            for rich in [False,True]:
                records += [freeze(0,version,wire.tagged(version,name,rich,kernel)) for kernel in [0,1,2]]
                records.append(freeze(0,version,wire.tagged(version,name,rich,reused=True)))
        for rich in [False,True]:
            for mask in range(4):
                w=wire.WireMap();records.append(freeze(0,version,w.new('moSketchChain_c',lambda:wire.chain(w,rich,mask))))
                w=wire.WireMap();records.append(freeze(0,version,w.new('moContourChooser_c',lambda:wire.chooser(w,rich,0,mask))))
            w=wire.WireMap();records.append(freeze(0,version,w.new('moOneContour_c',lambda:wire.contour(w,rich,0))))
        records.append(freeze(0,version,wire.WireMap().new('moSketchRegion_c',lambda:bytes(2))))
        for rich in [False,True]:
            for kernel in [0,1,2]:records.append(freeze(1,version,wire.geometry(rich,kernel)))
    for size in [0,4,8,12,32]:
        for count in [0,1,3]:records.append(freeze(2,19000,wire.reference_array(size,count)))
    records.append(freeze(2,19000,wire.P('<4I',0xfffffffc,0,0xffffffff,0)))
    edits={f'profile-{name}.bin':wire.tagged(19000,'moProfileFeature_c',True,2,False,word) for name,word in [('original',-1),('edited',12345)]}
    for raw in edits.values():freeze(0,19000,raw)
    if before!=reference_snapshot(REFERENCE):raise ValueError('reference changed; expected stable source hashes')
    packet=b''.join(raw for raw,_ in records)
    files={'profile-cases.bin':packet,**edits}
    for name,raw in files.items():(WORKSPACE/'tests/fixtures/model-archives'/name).write_bytes(raw)
    report={'cases':len(records),'fields':sum(n for _,n in records),'sha256':hashlib.sha256(packet).hexdigest(),'source_hashes':before,
            'projection':'aligned reference bytes become independently decoded u32 groups; kernel nodes use the separate typed oracle; suppression retained'}
    Path(__file__).with_name('profile-cases.json').write_bytes((json.dumps(report)+'\n').encode())
    print({k:v for k,v in report.items() if k not in ['source_hashes','projection']})

if __name__=='__main__':generate()
