import unittest
from pathlib import Path
import sys
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'docs/features/model-archives'))
from profile_oracle_projection import reference_words,project

class ProfileProjectionTests(unittest.TestCase):
    def test_little_endian_groups_preserve_all_bits_and_empty_wide_rows(self):
        self.assertEqual(reference_words(8,2,bytes.fromhex('00000000ffffffff7856341200000080')),
                         [[0,0xffffffff],[0x12345678,0x80000000]])
        self.assertEqual(reference_words(0xfffffffc,0,b''),[])

    def test_invalid_shapes_do_not_drop_or_invent_payload_bytes(self):
        for size,count,payload in [(0,0,b''),(3,1,b'123'),(4,-1,b''),(4,1,b'123'),(4,1,b'12345')]:
            with self.assertRaises(ValueError):reference_words(size,count,payload)

    def test_projection_preserves_aliases_cycles_and_raw_metadata(self):
        array={'element_size':4,'capacity':0,'growth':0xffffffff,'count':1,'unknown_data':b'\xff'*4}
        graph={'first':array,'second':array};graph['self']=graph
        project(graph)
        self.assertIs(graph['first'],graph['second'])
        self.assertIs(graph['self'],graph)
        self.assertEqual(array,{'element_size':4,'capacity':0,'growth':0xffffffff,'count':1,'words':[[0xffffffff]]})

    def test_profile_kernel_projection_keeps_primary_and_extra_suppression(self):
        value={'word_10':-1,'body':{'suppressed':0,'serialized_body':True},'pairs':[],
               'reference_data':None,'extra_bodies':[(-7,{'suppressed':195}),(-9,{'suppressed':0,'serialized_body':True})]}
        project(value)
        self.assertEqual(value['body'],{'suppressed':0})
        self.assertEqual(value['extra_bodies'],[(-7,{'suppressed':195}),(-9,{'suppressed':0})])
