"""Execute only the unchanged published fields function, excluding top-level native inspection."""
import ast
import sys
from pathlib import Path
from types import ModuleType

NAME='research.simulation_aux_body_ranges_evidence'


def load_fields(pin: Path) -> ModuleType:
    """Compile the actual copied function AST and original stdlib import with its real filename."""
    path=pin/'research/simulation_aux_body_ranges_evidence.py'
    tree=ast.parse(path.read_text(),filename=str(path))
    functions=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='fields']
    assert len(functions)==1
    imports=[n for n in tree.body if isinstance(n,ast.Import) and all(a.name in ('hashlib','json','struct','sys') for a in n.names)]
    module=ModuleType(NAME)
    module.__file__=str(path)
    sys.modules[NAME]=module
    unit=ast.Module(body=[*imports,functions[0]],type_ignores=[])
    exec(compile(unit,str(path),'exec'),module.__dict__)
    return module


def fields_source(pin: Path) -> bytes:
    """Retain the exact helper source segment so bounded execution scope is independently auditable."""
    path=pin/'research/simulation_aux_body_ranges_evidence.py'
    text=path.read_text()
    function=next(n for n in ast.parse(text).body if isinstance(n,ast.FunctionDef) and n.name=='fields')
    source=ast.get_source_segment(text,function)
    assert source is not None
    return source.encode()
