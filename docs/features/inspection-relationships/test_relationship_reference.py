"""Independent selected relationship helper contracts before a Rust representation change (#197)."""
from pathlib import Path
from xml.dom import Node, minidom
from xml.dom.minidom import Element
import unittest
import inspectionrelationships as relationships

ROOT = Path(__file__).resolve().parents[3]
FIXTURE = ROOT / 'tests/fixtures/inspection-relationships/ordered.xml'


def package(xml: bytes) -> Element:
    """Provide an explicit retained-node DOM, without claiming native parser policy; e.g. package(b'<PackageOptions/>')."""
    return minidom.parseString(xml).getElementsByTagName('PackageOptions')[0]


class RelationshipReferenceTests(unittest.TestCase):
    def test_retained_nodes_repeated_fields_and_descendant_text_preserve_order(self) -> None:
        """Require non-element empty attempts and exact repeated-field behavior."""
        original = FIXTURE.read_bytes()
        attempts = relationships.load_attempts(package(original))
        self.assertEqual(len(attempts), 9)
        self.assertEqual([entry['node'].nodeType for entry in attempts[:3]], [Node.TEXT_NODE, Node.COMMENT_NODE, Node.PROCESSING_INSTRUCTION_NODE])
        self.assertEqual(attempts[3]['node'].nodeName, 'Other')
        self.assertEqual(attempts[3]['key'], 'final')
        self.assertEqual(attempts[3]['values'], ['abc', 'x', ''])
        expected = [dict(key='', values=[]), dict(key='final', values=['abc', 'x', '']),
                    dict(key='A', values=['first']), dict(key='ns-key', values=['ns-value'])]
        self.assertEqual(relationships.insert_first(attempts), expected)
        self.assertEqual(relationships.writer_entries(expected), expected[1:])
        self.assertEqual(FIXTURE.read_bytes(), original)

    def test_element_only_input_changes_first_empty_key_result_explicitly(self) -> None:
        """Keep caller DOM retention assumptions visible rather than selecting a hidden default."""
        attempts = relationships.load_attempts(package(FIXTURE.read_bytes()))
        element_attempts = [entry for entry in attempts if entry['node'].nodeType == Node.ELEMENT_NODE]
        self.assertEqual(len(element_attempts), 6)
        expected = [dict(key='final', values=['abc', 'x', '']), dict(key='A', values=['first']),
                    dict(key='', values=['later-empty']), dict(key='ns-key', values=['ns-value'])]
        self.assertEqual(relationships.insert_first(element_attempts), expected)
        self.assertEqual(relationships.insert_first(attempts)[0], dict(key='', values=[]))

    def test_exact_dom_node_names_differ_from_namespace_expanded_matching(self) -> None:
        """Accept unprefixed default-namespace names and skip prefixed or case-mismatched names."""
        owner = package(FIXTURE.read_bytes())
        attempts = relationships.load_attempts(owner)
        self.assertEqual(attempts[3]['node'].namespaceURI, 'urn:dictionary')
        self.assertEqual(attempts[-1]['key'], 'ns-key')
        self.assertEqual(attempts[-1]['values'], ['ns-value'])
        self.assertEqual(attempts[-2]['key'], '')
        self.assertEqual(attempts[-2]['values'], [])
        self.assertNotIn('prefixed-container-excluded', [entry['key'] for entry in attempts])
        self.assertNotIn('second-dictionary-excluded', [entry['key'] for entry in attempts])

    def test_exact_utf16_equality_case_and_normalization_without_aliasing_values(self) -> None:
        """Check the pure insertion helper's wider native string domain separately from XML admission."""
        entries = [dict(key=key, values=[str(index)]) for index, key in enumerate(
            ('A', 'A', 'a', 'A\0', '\U0001f600', '\ud83d\ude00', 'é', 'e\u0301'))]
        actual = relationships.insert_first(entries)
        self.assertEqual([entry['key'] for entry in actual], ['A', 'a', 'A\0', '\U0001f600', 'é', 'e\u0301'])
        entries[0]['values'].append('mutation')
        self.assertEqual(actual[0]['values'], ['0'])
        self.assertEqual(relationships.trimmed('\u200bx\ufeff'), '\u200bx\ufeff')
        self.assertEqual(relationships.trimmed('\u0085\u00a0 x \u3000'), 'x')

    def test_accepted_tutorial_keeps_48_keys_and_120_reciprocal_links(self) -> None:
        """Reuse the accepted decoded tutorial without selecting an encrypted decoder dependency."""
        raw = (ROOT / 'tests/fixtures/standalone-inspection/historical.xml').read_bytes()
        entries = relationships.writer_entries(relationships.insert_first(relationships.load_attempts(package(raw))))
        self.assertEqual(len(entries), 48)
        graph = {entry['key']: entry['values'] for entry in entries}
        self.assertEqual(sum(map(len, graph.values())), 120)
        self.assertTrue(all(key in graph[target] for key, values in graph.items() for target in values))
