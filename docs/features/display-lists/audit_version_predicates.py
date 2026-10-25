"""Replay owned full roots through unchanged comparison values in an instrumented reference."""
from pathlib import Path
import hashlib
import json
import platform
import sys
from owned_root_inventory import OwnedRoot,inventory_roots
from predicate_probe import PredicateCollector,ProbedSourceFinder,predicate_report,MODULES
from registered_class_audit import DisplayClassWitness,observe_classes,registered_readers,dispatched_classes,reader_class_report
from audit_provenance import audit_source_snapshot,require_unchanged_snapshot


def install_probe(reference: Path, collector: PredicateCollector) -> ProbedSourceFinder:
    if any(name in sys.modules for name in MODULES):
        raise RuntimeError("reference modules already imported; expected a fresh audit process")
    finder = ProbedSourceFinder(reference,collector)
    sys.path.insert(0,str(reference))
    sys.meta_path.insert(0,finder)
    return finder


def parse_owned_root(root: OwnedRoot, collector: PredicateCollector) -> dict[str,int]:
    import displaylists
    from mfcobjects import ObjectArchive
    archive = ObjectArchive(root.payload,root.version)
    archive.doc_kind,archive.file_version = root.kind,root.file_version
    archive.face_tess_comp_ids = set(root.external_ids)
    collector.active = root.label,root.version,root.kind,root.file_version
    try:displaylists.display_lists(archive,root.kind)
    finally:collector.active = None
    if archive.p!=len(root.payload):
        raise ValueError(f"{root.label} ended at {archive.p}; expected {len(root.payload)}")
    return archive.classes


def rust_replay(executable: str, roots: list[OwnedRoot]) -> str:
    from verify_configuration_processes import frames,two_processes
    cases = [(root.context_row(),root.payload) for root in roots]
    source,expected = frames(cases)
    return two_processes(executable,source,expected)


def bind_graphics_plane() -> None:
    import dl_tess
    from generate_graphics_references import plane_reader
    dl_tess.EXTERN["mgPlane"] = plane_reader


def write_report(reference: Path, roots: list[OwnedRoot], collector: PredicateCollector, trace_hash: str, witnesses: dict[str,DisplayClassWitness], snapshot: dict[str,object]) -> None:
    predicates = predicate_report(collector)
    report = {"scope":"owned full-root execution of version predicates; one-sided and unobserved paths require review",
        "platform":platform.platform(),"roots":len(roots),"groups":len({root.label.split('[')[0] for root in roots}),
        "two_rust_processes_byte_identical":True,"rust_trace_sha256":trace_hash,
        "reference_source_sha256":{name:hashlib.sha256((reference/(name+".py")).read_bytes()).hexdigest() for name in sorted(MODULES)},
        "predicates":len(predicates),"both_outcomes":sum(row["both_outcomes"] for row in predicates),
        "observations":predicates,
        "source_snapshot":snapshot,"registered_classes":reader_class_report(registered_readers(reference),
            dispatched_classes(Path("src/display_lists/archive_objects.rs")),witnesses)}
    destination = Path(__file__).with_name("version-predicate-audit.json")
    destination.write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8",newline="\n")
    print(json.dumps({key:value for key,value in report.items() if key not in {"observations","registered_classes","source_snapshot"}}))


def main(executable: str) -> None:
    reference = Path("../inventor-interface/solidworks").resolve()
    collector = PredicateCollector()
    install_probe(reference,collector)
    snapshot = audit_source_snapshot(reference,Path(executable))
    roots = inventory_roots(Path(__file__).parent,Path("tests/fixtures/display-lists"))
    trace_hash = rust_replay(executable,roots)
    bind_graphics_plane()
    witnesses: dict[str,DisplayClassWitness] = {}
    for index,root in enumerate(roots):
        observe_classes(witnesses,root,parse_owned_root(root,collector))
        if index%1000==0:print(json.dumps({"parsed_roots":index+1,"total":len(roots)}),flush=True)
    require_unchanged_snapshot(snapshot,audit_source_snapshot(reference,Path(executable)))
    write_report(reference,roots,collector,trace_hash,witnesses,snapshot)


if __name__=="__main__":
    main(sys.argv[1])
