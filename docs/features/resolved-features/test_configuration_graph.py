"""Independent postload policy oracle for shared, deep and invalid graphs."""
import sys
import unittest
from freeze_manager_context import WORKSPACE, source_module


def graph_context(children: list[list[int]], roots: list[int], selected: int) -> dict[str, object]:
    """Construct explicit identity relationships; e.g. graph_context([[1], []], [0], 1)."""
    nodes = [dict(_class='moPartConfiguration_c', parent_698=None, raw_77c=0,
                  feature={'has_unknown_1e0': 0}, children_6a0={'items': []}) for _ in children]
    for node, indices in zip(nodes, children):
        node['children_6a0']['items'] = [nodes[index] for index in indices]
    return {'configuration_358': nodes[selected], 'configuration_360': nodes[selected],
            'list_368': {'items': [nodes[index] for index in roots]}}


class ConfigurationGraphReferenceTests(unittest.TestCase):
    def test_reference_accepts_dag_multiple_roots_selected_descendant_and_deep_graph(self) -> None:
        reference = WORKSPACE.parent / 'inventor-interface/solidworks'
        sys.path.insert(0, str(reference))
        validate = source_module('configurationmanager', reference).validate_postload_graph
        self.assertEqual(validate(graph_context([[1, 2], [3], [3], []], [0], 3)), 4)
        self.assertEqual(validate(graph_context([[1], []], [0, 1], 1)), 2)
        chain = [[node+1] if node < 1500 else [] for node in range(1501)]
        self.assertEqual(validate(graph_context(chain, [0], 1500)), 1501)

    def test_reference_rejects_duplicate_empty_cycles_and_unreachable_selection(self) -> None:
        reference = WORKSPACE.parent / 'inventor-interface/solidworks'
        sys.path.insert(0, str(reference))
        validate = source_module('configurationmanager', reference).validate_postload_graph
        error = source_module('swarchive', reference).ArchiveError
        for graph in [graph_context([[]], [0, 0], 0), graph_context([[]], [], 0),
                      graph_context([[], []], [0], 1), graph_context([[0]], [0], 0),
                      graph_context([[1], [0]], [0], 0)]:
            with self.assertRaises(error):
                validate(graph)


if __name__ == '__main__':
    unittest.main()
