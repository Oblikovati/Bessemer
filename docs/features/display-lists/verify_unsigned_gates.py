"""Bind the high-version fixtures to the reference's current version comparisons."""
from pathlib import Path
import ast
import hashlib
import json

NAMES = ["dl_doc","dl_tess","dl_light","dl_ui","facettess","drawingvblists"]


def mentions_version(node: ast.AST) -> bool:
    for child in ast.walk(node):
        if isinstance(child,ast.Name) and child.id in {"v","V","version"}:return True
        if isinstance(child,ast.Attribute) and child.attr=="version":return True
        if isinstance(child,ast.Call) and isinstance(child.func,ast.Name) and child.func.id=="_v":return True
    return False


def comparisons(path: Path) -> dict[str,object]:
    raw = path.read_bytes()
    tree = ast.parse(raw.decode())
    rows = []
    for node in ast.walk(tree):
        if not isinstance(node,ast.Compare) or not mentions_version(node):continue
        literals = [child.value for child in ast.walk(node) if isinstance(child,ast.Constant) and type(child.value) is int]
        if not literals:continue
        rows.append({"line":node.lineno,"comparison":ast.unparse(node),"literals":literals})
    maximum = max(value for row in rows for value in row["literals"])
    assert maximum<=19000,f"{path}: version literal {maximum}; review the newer layout before reusing 19000 bytes"
    literals = sorted({value for row in rows for value in row["literals"]})
    return {"source_sha256":hashlib.sha256(raw).hexdigest(),"largest_literal":maximum,"comparison_count":len(rows),"literals":literals}


if __name__=="__main__":
    result = {name:comparisons(Path("../inventor-interface/solidworks")/(name+".py")) for name in NAMES}
    report = {"scope":"reference comparisons mentioning v, V, version or _v; complete-root oracles verify representative high-version layouts separately","modules":result}
    Path(__file__).with_name("unsigned-gates.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({name:row["comparison_count"] for name,row in result.items()}))
