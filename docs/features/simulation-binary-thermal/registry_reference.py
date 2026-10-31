"""Audit declared classes from the actual copied callback AST and actual registry dictionaries (#134)."""
import ast
import importlib
import inspect
from types import FunctionType
from typing import cast
from owned_references import owned_reference_cases
from owned_chains import owned_chain_cases


def unpacked_classes(expression: ast.expr, namespace: dict[str, object]) -> set[str]:
    """Resolve only actual named registries or the declared KINDS comprehension; e.g. unpacked_classes(node, namespace)."""
    if isinstance(expression, ast.DictComp):
        expression = expression.generators[0].iter
    assert isinstance(expression, ast.Name), f'{ast.dump(expression)}; expected named registry'
    registry = namespace[expression.id]
    assert isinstance(registry, (dict, tuple)), f'{expression.id}; expected actual dictionary or tuple'
    names = set(registry)
    assert all(isinstance(name, str) for name in names)
    return cast(set[str], names)


def dictionary_classes(dictionary: ast.Dict, namespace: dict[str, object]) -> set[str]:
    """Read declarations without altering the actual reader; e.g. dictionary_classes(node, namespace)."""
    names: set[str] = set()
    for key, value in zip(dictionary.keys, dictionary.values):
        if isinstance(key, ast.Constant) and isinstance(key.value, str):
            names.add(key.value)
        else:
            assert key is None, f'{ast.dump(dictionary)}; expected string keys or registry expansion'
            names.update(unpacked_classes(value, namespace))
    return names


def callback_classes(module_name: str, function_name: str) -> set[str]:
    """Inspect the unmodified pinned function and its real globals; e.g. callback_classes('cosworks_embedded_reference_archive', 'read_archive')."""
    module = importlib.import_module(module_name)
    callback = cast(FunctionType, getattr(module, function_name))
    namespace = cast(dict[str, object], callback.__globals__)
    names: set[str] = set()
    for node in ast.walk(ast.parse(inspect.getsource(callback))):
        if isinstance(node, ast.Dict):
            names.update(dictionary_classes(node, namespace))
    return names


def declared_registry() -> list[str]:
    """Require owned root cases for every actual additive class declaration; e.g. declared_registry()."""
    declared = callback_classes('cosworks_embedded_reference_archive', 'read_archive')
    declared |= callback_classes('cosworks_force_optional_audit', 'archive_reader')
    declared |= callback_classes('cosworks_binary_thermal_transform_audit', 'archive_reader')
    owned = {name for _, name, _ in owned_reference_cases() + owned_chain_cases()}
    owned |= {'moCompXFormWrapper_c', 'moCompHolder_c'}
    assert declared == owned, f'missing={declared-owned}, extra={owned-declared}; expected the complete declared registry'
    assert len(declared) == 61
    return sorted(declared)
