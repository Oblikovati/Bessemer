"""Run reference fixture checks from an immutable source copy while upstream develops (#54)."""
from pathlib import Path
from collections.abc import Sequence
from importlib.machinery import ModuleSpec
from types import ModuleType
import hashlib
import importlib
import importlib.abc
import importlib.util
import json
import sys
import tempfile
import unittest

WORKSPACE=Path(__file__).resolve().parents[3]
SOURCE=WORKSPACE.parent/'inventor-interface/solidworks'


class FrozenSourceImports(importlib.abc.MetaPathFinder):
    """Keep byte-builder imports on the pin even when older helpers prepend live paths."""
    def __init__(self, pin: Path) -> None:
        """Bind this process's immutable reference root.
        Example: `FrozenSourceImports(pin)`.
        """
        self.pin=pin

    def find_spec(self, fullname: str, path: Sequence[str] | None=None, target: ModuleType | None=None) -> ModuleSpec | None:
        """Load the actual pinned module without replacing reader functions; e.g. find_spec('endcaps')."""
        candidate=self.pin/(fullname+'.py')
        if '.' not in fullname and candidate.is_file():
            return importlib.util.spec_from_file_location(fullname,candidate)
        return None


def copy_sources(pin: Path) -> dict[str,str]:
    """Require each copied file to match the initial source snapshot; e.g. copy_sources(pin)."""
    hashes={p.relative_to(SOURCE).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(SOURCE.rglob('*.py'))}
    for name,digest in hashes.items():
        raw=(SOURCE/name).read_bytes()
        assert hashlib.sha256(raw).hexdigest()==digest,f'{name}; expected stable source hash {digest}'
        destination=pin/name;destination.parent.mkdir(parents=True,exist_ok=True);destination.write_bytes(raw)
    return hashes


def record_snapshot(pin: Path, hashes: dict[str,str], result: unittest.TestResult, destination: Path, readers: tuple[str,...]) -> None:
    """Retain origin/hash evidence before cleanup; e.g. record_snapshot(pin,hashes,result,folder,readers)."""
    pinned={p.relative_to(pin).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(pin.rglob('*.py'))}
    assert pinned==hashes,'pinned files changed; expected the immutable source snapshot'
    origins={name:str(Path(module.__file__).resolve()) for name,module in sys.modules.copy().items()
             if getattr(module,'__file__',None) and Path(module.__file__).resolve().is_relative_to(pin)}
    live=[name for name,module in sys.modules.copy().items() if getattr(module,'__file__',None) and Path(module.__file__).resolve().is_relative_to(SOURCE)]
    assert not live,f'imported live modules {live}; expected every source import under {pin}'
    assert {'swcontainer','resolvedfeatures','mfcobjects',*readers}<=origins.keys(),f'missing pinned readers; expected {readers}'
    report=dict(source_hashes=hashes,source_python_files=len(hashes),source_origins=origins,
                tests_run=result.testsRun,successful=result.wasSuccessful(),
                snapshot_identity=hashlib.sha256(json.dumps(hashes,sort_keys=True).encode()).hexdigest())
    (destination/'fixture-reference-snapshot.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8',newline='\n')


def run(feature_directory: Path, freeze_module: str, readers: tuple[str,...]) -> bool:
    """Check actual independent readers in a fresh process; e.g. run(folder,'freeze_end_caps',('endcaps',))."""
    with tempfile.TemporaryDirectory(prefix='bessemer-reference-') as temporary:
        pin=Path(temporary);hashes=copy_sources(pin)
        sys.meta_path.insert(0,FrozenSourceImports(pin))
        sys.path.insert(0,str(feature_directory))
        module=importlib.import_module(freeze_module)
        module.REFERENCE=pin
        suite=unittest.defaultTestLoader.discover(str(feature_directory))
        result=unittest.TextTestRunner(verbosity=1).run(suite)
        record_snapshot(pin,hashes,result,feature_directory,readers)
    return result.wasSuccessful()

