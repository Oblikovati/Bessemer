"""Check the independent reference behind the table and common-folder regressions."""
import struct
import sys
import unittest
from freeze_manager_context import WORKSPACE, source_module


def native_payload() -> bytes:
    """Keep the independently specified pair sequence; e.g. native_payload()."""
    return (b'\x03raw'+struct.pack('<iii', -7, 123, 3)
            +b'\x01A\x03one\xff\xfe\xff\x01A\0\x03two\0\0'+struct.pack('<I', 0xffffffff))


class ConfigurationTableReferenceTests(unittest.TestCase):
    def test_complete_reference_objects_match_the_specified_shared_body_sizes_and_suffixes(self) -> None:
        reference = WORKSPACE.parent / 'inventor-interface/solidworks'
        sys.path.insert(0, str(reference))
        source_module('modelfeatures', reference)
        objects = source_module('mfcobjects', reference)
        cases = [('moConfigTableFolder_c', bytes(139)+struct.pack('<ii', -7, -2147483648)),
                 ('moNativeConfigurationTableFeature_c', bytes(143)+native_payload()),
                 ('moDesignTableFeature_c', bytes(143))]
        for name, body in cases:
            encoded = name.encode('ascii')
            raw = struct.pack('<3H', 0xffff, 1, len(encoded))+encoded+body
            cursor = objects.ObjectArchive(raw, 19000)
            cursor.feature_context = 'ordinary_external_xml'
            parsed = cursor.object()
            self.assertEqual(cursor.p, len(raw))
            self.assertEqual(parsed['_class'], name)
            if 'payload' in parsed:
                self.assertEqual(parsed['payload']['pairs'], [{'key': 'A', 'value': 'one'},
                    {'key': 'A', 'value': 'two'}, {'key': '', 'value': ''}])
                self.assertEqual(parsed['payload']['raw_388'], 4294967295)


if __name__ == '__main__':
    unittest.main()
