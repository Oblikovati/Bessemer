"""Check that every remaining comparison has a current, explicit caller or rejection review."""
from collections import defaultdict
from pathlib import Path
import hashlib
import json
from audit_provenance import file_hashes

REVIEW_KINDS = frozenset({"explicit-negative-layout","different-caller-context","alternate-reference-helper",
    "enclosing-root-guard","enclosing-reader-guard","delegated-reference-helper","generic-entry-guard"})
PredicateKey = tuple[str,str,str]


def predicate_index(observations: list[dict[str,object]]) -> dict[PredicateKey,list[dict[str,object]]]:
    indexed = defaultdict(list)
    for row in observations:
        module = row["source"].split(".py:")[0]
        indexed[(module,row["function"],row["expression"])].append(row)
    return dict(indexed)


def current_audit_sources(snapshot: dict[str,object]) -> None:
    for domain in ("reference","runtime","adapters"):
        expected = snapshot[domain]
        actual = file_hashes(Path(path) for path in expected)
        if expected!=actual:raise ValueError(f"stale {domain} snapshot; expected current source hashes before reviewing outcomes")


def review_evidence(entries: list[list[str]], root: Path) -> dict[str,str]:
    if not entries:raise ValueError("empty review evidence; expected source or test fragments")
    hashes = {}
    for filename,fragment in entries:
        path = root/filename
        contents = path.read_bytes()
        if fragment not in contents.decode("utf-8-sig"):
            raise ValueError(f"{filename} lacks {fragment!r}; expected the reviewed source/test fragment")
        hashes[filename] = hashlib.sha256(contents).hexdigest()
    return hashes


def covered_comparisons(keys: list[list[str]], indexed: dict[PredicateKey,list[dict[str,object]]]) -> list[str]:
    sources = []
    for key in keys:
        rows = indexed.get(tuple(key),[])
        if not rows or not all(row["both_outcomes"] for row in rows):
            raise ValueError(f"coverage transfer {key}; expected both outcomes for every matching comparison")
        sources.extend(row["source"] for row in rows)
    return sources


def reviewed_comparison(module: str, entry: list[object], indexed: dict[PredicateKey,list[dict[str,object]]]) -> dict[str,object]:
    function,expression,outcomes = entry
    key = module,function,expression
    rows = indexed.get(key,[])
    if len(rows)!=1:raise ValueError(f"review key {key} matched {len(rows)}; expected one comparison")
    row = rows[0]
    if row["both_outcomes"] or sorted(row["observed"])!=sorted(outcomes):
        raise ValueError(f"review outcomes {key}: {sorted(row['observed'])}; expected one-sided {outcomes}")
    return {"source":row["source"],"function":function,"expression":expression,"observed_outcomes":outcomes}


def review_group(rule: dict[str,object], audit: dict[str,object], indexed: dict[PredicateKey,list[dict[str,object]]], root: Path) -> dict[str,object]:
    if rule["classification"] not in REVIEW_KINDS or not rule["reason"]:
        raise ValueError(f"review {rule['id']}; expected a known classification and reason")
    registered = {row["reader"] for row in audit["registered_classes"]}
    invalid = registered.intersection(rule.get("unregistered_readers",[]))
    if invalid:raise ValueError(f"registered readers {sorted(invalid)}; expected the reviewed embedded helpers to remain unregistered")
    return {"id":rule["id"],"classification":rule["classification"],"reason":rule["reason"],
        "predicates":[reviewed_comparison(rule["module"],entry,indexed) for entry in rule["predicates"]],
        "evidence_sha256":review_evidence(rule["evidence"],root),
        "covered_by":covered_comparisons(rule.get("covered_by",[]),indexed)}


def verify_reviews(audit: dict[str,object], catalog: dict[str,object], root: Path) -> dict[str,object]:
    indexed = predicate_index(audit["observations"])
    groups = [review_group(rule,audit,indexed,root) for rule in catalog["reviews"]]
    identities = [group["id"] for group in groups]
    reviewed = [row["source"] for group in groups for row in group["predicates"]]
    expected = {row["source"] for row in audit["observations"] if not row["both_outcomes"]}
    if len(identities)!=len(set(identities)) or len(reviewed)!=len(set(reviewed)):
        raise ValueError("duplicate review identities; expected one review per group and comparison")
    if set(reviewed)!=expected:
        raise ValueError(f"unreviewed {sorted(expected-set(reviewed))}, stale {sorted(set(reviewed)-expected)}; expected every remaining comparison")
    return {"scope":catalog["scope"],"reviewed_comparisons":len(reviewed),"review_groups":groups,
        "registered_classes_with_owned_witnesses":len(audit["registered_classes"]),
        "geometry_and_native_application_acceptance":False}


def main() -> None:
    evidence = Path(__file__).parent
    audit_path,catalog_path = evidence/"version-predicate-audit.json",evidence/"predicate-reviews.json"
    audit = json.loads(audit_path.read_text(encoding="utf-8"))
    current_audit_sources(audit["source_snapshot"])
    report = verify_reviews(audit,json.loads(catalog_path.read_text(encoding="utf-8")),Path("."))
    report["input_sha256"] = file_hashes([audit_path,catalog_path,Path(__file__)])
    (evidence/"predicate-review-verification.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({key:value for key,value in report.items() if key not in {"review_groups","input_sha256"}}))


if __name__=="__main__":
    main()
