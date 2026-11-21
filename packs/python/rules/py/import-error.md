---
id: py/import-error
title: "An import of the project's own module must name a module that exists"
languages: [python]
scope: project
fix: manual
fix_reason: "which module the import should name, or whether the file is missing from the checkout, only the author can say"
fix_exception: value
stability: preview
ported_from: [pylint/import-error]
since: 0.5.0
---

An `import a.b` or `from a.b import x` is reported when `a` is a module or a regular package (with an `__init__.py`) of the project and `a.b` is not there, and a relative import (`from .b import x`) when the module it names is not there. A first part the project does not hold — the standard library, an installed package, a folder without `__init__.py` that other path entries may add to — is not judged: codelaw does not read `site-packages`, so Pylint's `E0401` for a package that is simply not installed is not built. Also not judged: an import inside `try … except ImportError` (or a base of it), under `if TYPE_CHECKING:` or `if sys.version_info …:`, in a file that changes `sys.path`, and from a package that changes its own `__path__`. A relative import that climbs past the top-level package is `py/relative-beyond-top-level`. A module generated at build time (`_version.py`) is reported until it is in the checkout; put the import in a guard or exempt it. The project's roots are the repo root, `src/`, each `pyproject.toml` folder and its `src/`, a `setup.py` folder, and the importing file's own folder.

**Why:** the import raises `ModuleNotFoundError` the first time it runs, often only on a path a test never takes.

```codelaw
graph:
  python:
    imports: unresolved
message: "Unable to import `$SOURCE`"
```

## How to fix

1. Correct the module name, or add the file the import names.
2. If the module is optional, guard the import with `try … except ImportError`.

## Examples

### Python

#### Don't

```py files
# pkg/__init__.py
# pkg/real.py
VALUE = 1
# pkg/app.py
from pkg.rael import VALUE

print(VALUE)
```

#### Do

```py files
# pkg/__init__.py
# pkg/real.py
VALUE = 1
# pkg/app.py
from pkg.real import VALUE

print(VALUE)
```
