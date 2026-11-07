"""Registered reader coverage fails closed on missing decoders and absent owned witnesses."""
from dataclasses import dataclass
from pathlib import Path
import ast
import sys
import tempfile
import unittest

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"docs/features/display-lists"))
from owned_root_inventory import OwnedRoot
from predicate_probe import MODULES
from registered_class_audit import (DisplayClassWitness,RegisteredDisplayReader,dispatched_classes,
    observe_classes,reader_class_report,registered_readers,registration_name,statement_readers)


@dataclass
class FakeRegisteredSources:
    source: str = '@register("Base")\n@register("Alias")\ndef read_base(a, schema):\n    return {}\nregister("Derived")(read_base)\n'

    def write(self, root: Path) -> None:
        for module in MODULES:
            (root/(module+".py")).write_text(self.source if module=="dl_ui" else "",encoding="utf-8")


def witness_root(label: str = "owned[0]", version: int = 2771) -> OwnedRoot:
    return OwnedRoot(label,version,"part",19000,(),b"owned root")


class RegisteredClassTests(unittest.TestCase):
    def test_decorators_aliases_and_runtime_match_arms_have_exact_names(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)
            FakeRegisteredSources().write(root)
            readers=registered_readers(root)
            self.assertEqual(set(readers),{"Base","Alias","Derived"})
            self.assertTrue(all(reader.function=="read_base" for reader in readers.values()))
            self.assertEqual(readers["Derived"].line,5)
            runtime=root/"objects.rs"
            runtime.write_text('fn dispatch(a: A) { match a { "Base" | "Alias" => f(), "Derived" => g() } }')
            self.assertEqual(dispatched_classes(runtime),set(readers))

    def test_nonliteral_alias_duplicate_and_missing_dispatch_shapes_fail_explicitly(self) -> None:
        with self.assertRaisesRegex(ValueError,"expected register with one literal"):
            registration_name(ast.parse("register(variable)").body[0].value)
        with self.assertRaisesRegex(ValueError,"expected register.name..reader_function"):
            statement_readers(ast.parse('register("Base")(lambda: None)').body[0],"dl_ui")
        with tempfile.TemporaryDirectory() as temporary:
            root=Path(temporary)
            FakeRegisteredSources('@register("Base")\ndef first(): pass\n@register("Base")\ndef second(): pass\n').write(root)
            with self.assertRaisesRegex(ValueError,"duplicate class 'Base'"):
                registered_readers(root)
            runtime=root/"objects.rs";runtime.write_text("fn unrelated() {}")
            with self.assertRaisesRegex(ValueError,"expected fn dispatch"):
                dispatched_classes(runtime)

    def test_class_report_retains_exact_counts_contexts_and_first_owned_witness(self) -> None:
        readers={name:RegisteredDisplayReader("dl_ui","reader",7) for name in ["Beta","Alpha"]}
        witnesses={}
        observe_classes(witnesses,witness_root(),{"Beta":3,"Alpha":1})
        observe_classes(witnesses,witness_root("owned[1]",3196),{"Beta":2})
        report=reader_class_report(readers,set(readers),witnesses)
        self.assertEqual([row["class"] for row in report],["Alpha","Beta"])
        beta=report[1]
        self.assertEqual((beta["loaded_objects"],beta["roots"]),(5,2))
        self.assertEqual(beta["versions"],[2771,3196])
        self.assertEqual(beta["first_root"],"owned[0]")
        self.assertEqual(beta["kinds"],["part"])

    def test_runtime_drift_and_unobserved_or_unknown_classes_cannot_pass(self) -> None:
        readers={"Base":RegisteredDisplayReader("dl_ui","reader",1)}
        witnesses={};observe_classes(witnesses,witness_root(),{"Base":1})
        for dispatched in [set(),{"Base","Future"}]:
            with self.assertRaisesRegex(ValueError,"expected exact registered class coverage"):
                reader_class_report(readers,dispatched,witnesses)
        for observed in [{},{"Base":DisplayClassWitness()},dict(witnesses,Unknown=witnesses["Base"])]:
            with self.assertRaisesRegex(ValueError,"expected a witness for each registered class"):
                reader_class_report(readers,{"Base"},observed)
        for count in [0,-1]:
            with self.assertRaisesRegex(ValueError,"expected a positive loaded-object count"):
                DisplayClassWitness().observe(witness_root(),count)


if __name__=="__main__":
    unittest.main()
