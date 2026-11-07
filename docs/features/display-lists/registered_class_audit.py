"""Require a concrete runtime decoder and owned full-root witness for every registered class."""
import ast
from dataclasses import dataclass,field
from pathlib import Path
import re
from owned_root_inventory import OwnedRoot
from predicate_probe import MODULES


@dataclass(frozen=True)
class RegisteredDisplayReader:
    module: str
    function: str
    line: int


def registration_name(node: ast.AST) -> str | None:
    if not isinstance(node,ast.Call) or not isinstance(node.func,ast.Name) or node.func.id!="register":return None
    if len(node.args)!=1 or not isinstance(node.args[0],ast.Constant) or not isinstance(node.args[0].value,str):
        raise ValueError(f"registration {ast.dump(node)}; expected register with one literal class name")
    return node.args[0].value


def statement_readers(node: ast.stmt, module: str) -> list[tuple[str,RegisteredDisplayReader]]:
    if isinstance(node,ast.FunctionDef):
        return [(name,RegisteredDisplayReader(module,node.name,node.lineno))
            for decorator in node.decorator_list if (name:=registration_name(decorator)) is not None]
    if not isinstance(node,ast.Expr) or not isinstance(node.value,ast.Call):return []
    call = node.value
    name = registration_name(call.func)
    if name is None:return []
    if len(call.args)!=1 or not isinstance(call.args[0],ast.Name):
        raise ValueError(f"alias {ast.dump(call)}; expected register(name)(reader_function)")
    return [(name,RegisteredDisplayReader(module,call.args[0].id,node.lineno))]


def registered_readers(reference: Path) -> dict[str,RegisteredDisplayReader]:
    readers = {}
    for module in sorted(MODULES):
        tree = ast.parse((reference/(module+".py")).read_text(encoding="utf-8-sig"))
        for statement in tree.body:
            for name,reader in statement_readers(statement,module):
                if name in readers:raise ValueError(f"duplicate class {name!r}; expected one reference reader")
                readers[name] = reader
    return readers


def dispatched_classes(source: Path) -> set[str]:
    text = source.read_text(encoding="utf-8")
    _,separator,dispatch = text.partition("fn dispatch(")
    if not separator:raise ValueError(f"{source}; expected fn dispatch with literal match arms")
    return set(re.findall(r'"([A-Za-z][A-Za-z0-9_]*)"\s*(?:\||=>)',dispatch))


@dataclass
class DisplayClassWitness:
    objects: int = 0
    roots: int = 0
    versions: set[int] = field(default_factory=set)
    kinds: set[str] = field(default_factory=set)
    first_root: str = ""

    def observe(self, root: OwnedRoot, count: int) -> None:
        if count<1:raise ValueError(f"class object count {count}; expected a positive loaded-object count")
        self.objects += count
        self.roots += 1
        self.versions.add(root.version)
        self.kinds.add(root.kind)
        if not self.first_root:self.first_root = root.label


def observe_classes(witnesses: dict[str,DisplayClassWitness], root: OwnedRoot, classes: dict[str,int]) -> None:
    for name,count in classes.items():
        witnesses.setdefault(name,DisplayClassWitness()).observe(root,count)


def reader_class_report(readers: dict[str,RegisteredDisplayReader], dispatched: set[str], witnesses: dict[str,DisplayClassWitness]) -> list[dict[str,object]]:
    missing,extra = set(readers)-dispatched,dispatched-set(readers)
    if missing or extra:
        raise ValueError(f"runtime classes missing {sorted(missing)}, extra {sorted(extra)}; expected exact registered class coverage")
    unobserved = {name for name in readers if name not in witnesses
        or witnesses[name].objects<1 or witnesses[name].roots<1}
    unknown = set(witnesses)-set(readers)
    if unobserved or unknown:
        raise ValueError(f"owned classes unobserved {sorted(unobserved)}, unknown {sorted(unknown)}; expected a witness for each registered class")
    return [class_report_row(name,readers[name],witnesses[name]) for name in sorted(readers)]


def class_report_row(name: str, reader: RegisteredDisplayReader, witness: DisplayClassWitness) -> dict[str,object]:
    return {"class":name,"module":reader.module,"reader":reader.function,"line":reader.line,
        "runtime_dispatch_present":True,"loaded_objects":witness.objects,"roots":witness.roots,
        "versions":sorted(witness.versions),"kinds":sorted(witness.kinds),"first_root":witness.first_root}
