"""Caller reviews require exact outcomes, current evidence and complete review coverage."""
from copy import deepcopy
from dataclasses import dataclass
import hashlib
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"docs/features/display-lists"))
from audit_provenance import ADAPTERS,audit_source_snapshot,file_hashes,require_unchanged_snapshot
from predicate_probe import MODULES
from verify_predicate_reviews import current_audit_sources,verify_reviews


@dataclass
class FakeReviewEvidence:
    text: str = "fn guarded_reader() {}"

    def write(self, root: Path) -> None:
        (root/"proof.rs").write_text(self.text,encoding="utf-8")

    def catalog(self) -> dict[str,object]:
        return {"scope":"owned review test","reviews":[{"id":"one","module":"dl_ui",
            "classification":"enclosing-reader-guard","reason":"A required enclosing guard excludes the other outcome.",
            "predicates":[["read_one","v < 7",["false"]]],"evidence":[["proof.rs",self.text]],
            "covered_by":[["dl_ui","read_other","v > 3"]],"unregistered_readers":["read_one"]}]}


def review_audit() -> dict[str,object]:
    rows=[{"source":"dl_ui.py:1:0","function":"read_one","expression":"v < 7","both_outcomes":False,"observed":{"false":{}}},
        {"source":"dl_ui.py:2:0","function":"read_other","expression":"v > 3","both_outcomes":True,"observed":{"false":{},"true":{}}}]
    return {"observations":rows,"registered_classes":[{"reader":"registered_other"}]}


class ReviewValidationTests(unittest.TestCase):
    def test_complete_review_binds_current_evidence_and_both_outcome_transfer(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);fake=FakeReviewEvidence();fake.write(root)
            report=verify_reviews(review_audit(),fake.catalog(),root)
            self.assertEqual(report["reviewed_comparisons"],1)
            group=report["review_groups"][0]
            self.assertEqual(group["covered_by"],["dl_ui.py:2:0"])
            self.assertEqual(group["evidence_sha256"]["proof.rs"],hashlib.sha256(fake.text.encode()).hexdigest())

    def test_new_missing_or_duplicate_comparisons_cannot_be_silently_waived(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);fake=FakeReviewEvidence();fake.write(root)
            audit=review_audit();audit["observations"].append(dict(audit["observations"][0],source="dl_ui.py:3:0",function="new_reader"))
            with self.assertRaisesRegex(ValueError,"unreviewed .*expected every remaining"):
                verify_reviews(audit,fake.catalog(),root)
            catalog=fake.catalog();catalog["reviews"]*=2
            with self.assertRaisesRegex(ValueError,"duplicate review identities"):
                verify_reviews(review_audit(),catalog,root)
            audit=review_audit();audit["observations"]*=2
            with self.assertRaisesRegex(ValueError,"expected one comparison"):
                verify_reviews(audit,fake.catalog(),root)

    def test_changed_outcomes_delegation_and_registered_helpers_require_new_review(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);fake=FakeReviewEvidence();fake.write(root)
            for mutation,expected in [("outcome","expected one-sided"),("transfer","expected both outcomes"),("registered","expected the reviewed embedded")]:
                audit=review_audit()
                if mutation=="outcome":audit["observations"][0]["observed"]={"true":{}}
                if mutation=="transfer":audit["observations"][1]["both_outcomes"]=False
                if mutation=="registered":audit["registered_classes"].append({"reader":"read_one"})
                with self.assertRaisesRegex(ValueError,expected):verify_reviews(audit,fake.catalog(),root)

    def test_missing_evidence_unknown_classification_and_empty_reasons_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary);fake=FakeReviewEvidence();fake.write(root)
            for field,value,expected in [("evidence",[],"expected source or test fragments"),("classification","waive","expected a known"),("reason","","expected a known")]:
                catalog=fake.catalog();catalog["reviews"][0][field]=value
                with self.assertRaisesRegex(ValueError,expected):verify_reviews(review_audit(),catalog,root)
            (root/"proof.rs").write_text("changed")
            with self.assertRaisesRegex(ValueError,"expected the reviewed source/test fragment"):
                verify_reviews(review_audit(),fake.catalog(),root)


class AuditProvenanceTests(unittest.TestCase):
    def test_snapshots_include_runtime_reference_adapters_and_exact_executable(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)
            for folder in ["reference","src","examples","adapters"]:(root/folder).mkdir()
            for name in MODULES|{"mfcobjects","swarchive"}:(root/"reference"/(name+".py")).write_text("reader")
            for name in ADAPTERS:(root/"adapters"/(name+".py")).write_text("adapter")
            for name in ["Cargo.toml","Cargo.lock","rust-toolchain.toml","src/lib.rs","examples/check.rs","check.exe"]:(root/name).write_bytes(b"owned")
            snapshot=audit_source_snapshot(root/"reference",root/"check.exe",root,root/"adapters")
            self.assertEqual(len(snapshot["reference"]),8)
            self.assertEqual(len(snapshot["runtime"]),5)
            self.assertEqual(snapshot["executable_sha256"],hashlib.sha256(b"owned").hexdigest())
            current_audit_sources(snapshot)
            (root/"src/lib.rs").write_text("changed")
            with self.assertRaisesRegex(ValueError,"stale runtime snapshot"):current_audit_sources(snapshot)

    def test_mid_run_source_or_executable_changes_are_rejected(self) -> None:
        original={"reference":{"reader.py":"abc"},"executable_sha256":"same"}
        require_unchanged_snapshot(original,deepcopy(original))
        for key in original:
            changed=deepcopy(original);changed[key]="changed"
            with self.assertRaisesRegex(ValueError,"expected unchanged sources and executable"):
                require_unchanged_snapshot(original,changed)


if __name__=="__main__":
    unittest.main()
