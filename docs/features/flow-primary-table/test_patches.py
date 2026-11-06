"""Independent edit oracle, byte locations, complete reparse and reproducibility."""
import gzip
import tempfile
import unittest
from pathlib import Path
import freeze_patches as frozen
import patch_oracle as oracle
from patch_fields import fields
from wire_packet import ROOT


class PrimaryPatchTests(unittest.TestCase):
    def test_all_patch_fields_rebuild_and_reparse_with_only_the_expected_bytes_changed(self) -> None:
        roles=set();widths=set();versions=set()
        for name,raw in oracle.requests():
            versions.add(oracle.cases.schema.read(raw)['version'])
            for _,field,before,after,raw,expected in oracle.patch_cases(name,raw):
                oracle.validate_patch(field,before,after,raw,expected)
                roles.update(part for part in field.path if isinstance(part,str));widths.add(field.width)
        self.assertEqual(versions,set(range(4)));self.assertEqual(widths,{1,4,8})
        self.assertTrue({'kind_byte','element_kind_byte','presence_byte','stored_mode_byte','member_i32',
                         'first_patterns','second_patterns','member_triplet','wide6','wide3','legacy_wide9',
                         'stored_flag','rows','wide_blocks'}<=roles)

    def test_packet_regenerates_exactly_in_a_temporary_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root=Path(directory);frozen.freeze(root)
            relative='tests/fixtures/flow-primary-table/patches.bin.gz'
            self.assertEqual(gzip.decompress((root/relative).read_bytes()),gzip.decompress((ROOT/relative).read_bytes()))
            relative='docs/features/flow-primary-table/patch-verification.json'
            self.assertEqual((root/relative).read_bytes(),(ROOT/relative).read_bytes())

    def test_all_baseline_field_locations_match_stored_values_without_structural_words(self) -> None:
        for _,raw in oracle.cases.cases():
            value=oracle.cases.schema.read(raw);occupied=set()
            for field in fields(value):
                expected=oracle.struct.pack('<'+field.encoding,oracle.at(value,field.path))
                self.assertEqual(raw[field.offset:field.offset+field.width],expected)
                span=set(range(field.offset,field.offset+field.width))
                self.assertFalse(span&occupied);occupied.update(span)
            self.assertFalse(set(range(8))&occupied)

    def test_tuple_edits_isolate_original_tree_and_recompute_only_exact_one_flag_projection(self) -> None:
        value={'rows':[(1,2)],'body':{'stored_flag':1,'native_flag':True}}
        changed=oracle.replaced(value,('rows',0,1),99)
        self.assertEqual(value['rows'],[(1,2)]);self.assertEqual(changed['rows'],[(1,99)])
        field=oracle.PatchField(('body','stored_flag'),0,'B')
        self.assertFalse(oracle.changed_tree(value,field,255)['body']['native_flag'])
        self.assertTrue(value['body']['native_flag'])
        self.assertEqual(oracle.replacement(value,field),255)
        value['body']['stored_flag']=0
        self.assertEqual(oracle.replacement(value,field),1)
        self.assertTrue(oracle.changed_tree(value,field,1)['body']['native_flag'])
