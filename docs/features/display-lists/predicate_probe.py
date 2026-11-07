"""Observe source-version comparisons without changing their values or import files."""
import ast
import importlib.abc
import importlib.machinery
import importlib.util
from pathlib import Path
from types import CodeType,ModuleType
from collections.abc import Sequence
from dataclasses import dataclass,field

MODULES = frozenset({"displaylists","dl_doc","dl_ui","dl_light","dl_tess","facettess"})


def version_expression(node: ast.AST) -> bool:
    if isinstance(node,ast.Name):return node.id in {"v","V","sv"}
    if isinstance(node,ast.Attribute):return node.attr=="version"
    if isinstance(node,ast.Call) and isinstance(node.func,ast.Name):
        return node.func.id in {"_v","_sv","_file_version"}
    if isinstance(node,ast.BinOp):
        return version_expression(node.left) or version_expression(node.right)
    return False


@dataclass
class PredicateObservation:
    expression: str
    function: str
    outcomes: dict[bool,dict[str,object]] = field(default_factory=dict)


@dataclass
class PredicateCollector:
    active: tuple[str,int,str,int] | None = None
    predicates: dict[str,PredicateObservation] = field(default_factory=dict)

    def record(self, identity: str, outcome: bool) -> bool:
        if self.active is None:return outcome
        label,version,kind,file_version = self.active
        result = self.predicates[identity].outcomes.setdefault(outcome,{
            "evaluations":0,"versions":set(),"file_versions":set(),"kinds":set(),"first_root":label,
        })
        result["evaluations"] += 1
        result["versions"].add(version)
        result["file_versions"].add(file_version)
        result["kinds"].add(kind)
        return outcome


class VersionComparisonProbe(ast.NodeTransformer):
    def __init__(self, module: str, collector: PredicateCollector) -> None:
        self.module,self.collector = module,collector
        self.functions: list[str] = []

    def visit_FunctionDef(self, node: ast.FunctionDef) -> ast.AST:
        self.functions.append(node.name)
        result = self.generic_visit(node)
        self.functions.pop()
        return result

    def visit_Compare(self, node: ast.Compare) -> ast.AST:
        if not any(version_expression(value) for value in [node.left,*node.comparators]):
            return self.generic_visit(node)
        identity = f"{self.module}.py:{node.lineno}:{node.col_offset}"
        self.collector.predicates[identity] = PredicateObservation(ast.unparse(node),".".join(self.functions))
        call = ast.Call(func=ast.Name(id="_bessemer_record_predicate",ctx=ast.Load()),
            args=[ast.Constant(identity),node],keywords=[])
        return ast.copy_location(call,node)


def instrument_source(source: str, path: str, module: str, collector: PredicateCollector) -> CodeType:
    tree = VersionComparisonProbe(module,collector).visit(ast.parse(source,filename=path))
    ast.fix_missing_locations(tree)
    return compile(tree,path,"exec")


class ProbedSourceLoader(importlib.machinery.SourceFileLoader):
    def __init__(self, module: str, path: str, collector: PredicateCollector) -> None:
        super().__init__(module,path)
        self.collector = collector

    def get_code(self, fullname: str) -> CodeType:
        source = self.get_data(self.path).decode("utf-8-sig")
        return instrument_source(source,self.path,fullname,self.collector)

    def exec_module(self, module: ModuleType) -> None:
        module.__dict__["_bessemer_record_predicate"] = self.collector.record
        super().exec_module(module)


class ProbedSourceFinder(importlib.abc.MetaPathFinder):
    def __init__(self, root: Path, collector: PredicateCollector) -> None:
        self.root,self.collector = root,collector

    def find_spec(self, fullname: str, path: Sequence[str] | None = None,
                  target: ModuleType | None = None) -> importlib.machinery.ModuleSpec | None:
        if fullname not in MODULES:return None
        source = self.root/(fullname+".py")
        loader = ProbedSourceLoader(fullname,str(source),self.collector)
        return importlib.util.spec_from_file_location(fullname,source,loader=loader)


def predicate_report(collector: PredicateCollector) -> list[dict[str,object]]:
    records = []
    for identity,predicate in sorted(collector.predicates.items()):
        outcomes = {str(outcome).lower():{
            key:sorted(value) if isinstance(value,set) else value for key,value in observed.items()
        } for outcome,observed in sorted(predicate.outcomes.items())}
        records.append({"source":identity,"function":predicate.function,"expression":predicate.expression,
            "both_outcomes":len(outcomes)==2,"observed":outcomes})
    return records
