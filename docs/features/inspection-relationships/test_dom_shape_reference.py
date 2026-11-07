"""Pin DOM event grouping needed by the shared XML relationship projection (#197)."""
from xml.dom import Node, minidom
import unittest


class RelationshipDomShapeTests(unittest.TestCase):
    def test_references_coalesce_without_erasing_cdata_boundaries(self) -> None:
        """Keep one decoded text node around references and a separate CDATA node."""
        root = minidom.parseString(b'<r>a&amp;&#98;<![CDATA[c]]>d&#x65;</r>').documentElement
        actual = [(node.nodeType, node.nodeValue) for node in root.childNodes]
        self.assertEqual(actual, [(Node.TEXT_NODE, 'a&b'), (Node.CDATA_SECTION_NODE, 'c'),
                                  (Node.TEXT_NODE, 'de')])

    def test_nested_text_order_cannot_be_recovered_by_appending_direct_text(self) -> None:
        """Require interleaving of element references and surrounding text."""
        root = minidom.parseString(b'<r>a<x>b<y>c</y>d</x>e<!--ignored--><?p ignored?>f</r>').documentElement
        actual = [(node.nodeType, node.nodeName, node.nodeValue) for node in root.childNodes]
        self.assertEqual(actual, [(Node.TEXT_NODE, '#text', 'a'), (Node.ELEMENT_NODE, 'x', None),
                                  (Node.TEXT_NODE, '#text', 'e'), (Node.COMMENT_NODE, '#comment', 'ignored'),
                                  (Node.PROCESSING_INSTRUCTION_NODE, 'p', 'ignored'),
                                  (Node.TEXT_NODE, '#text', 'f')])
        self.assertEqual(root.childNodes[1].firstChild.nodeValue, 'b')
        self.assertEqual(root.childNodes[1].lastChild.nodeValue, 'd')

    def test_literal_line_endings_normalize_but_numeric_references_survive(self) -> None:
        """Distinguish XML normalization from later Qt text trimming."""
        root = minidom.parseString(b'<r>a\r\nb\rc&#13;d&#10;e<![CDATA[f\r\ng\rh]]></r>').documentElement
        actual = [(node.nodeType, node.nodeValue) for node in root.childNodes]
        self.assertEqual(actual, [(Node.TEXT_NODE, 'a\nb\nc\rd\ne'),
                                  (Node.CDATA_SECTION_NODE, 'f\ng\nh')])


if __name__ == '__main__':
    unittest.main()
