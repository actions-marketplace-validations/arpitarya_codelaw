---
id: py/blocking-path-method-in-async-function
title: "An async function does not call `os.path` or `pathlib` file-system methods"
languages: [python]
fix: manual
fix_reason: "the call moves to a worker thread or to an async path library, and which one the project uses is the author's"
fix_exception: restructure
stability: preview
ported_from: [ruff/ASYNC240]
since: 0.5.0
---

Inside an `async def`, a call reaches the file system through `os.path` (`os.path.exists`, `os.path.getsize`, `os.path.realpath`, …) or through a method of a `pathlib` path built in the same expression (`Path(p).read_text()`, `Path(p).exists()`). Pure path arithmetic — `join`, `basename`, `dirname`, `split`, `splitext`, `isabs`, `normcase`, `expandvars`, `commonpath`, a path's `name`, `stem`, `suffix`, `parent`, `parts`, `with_*`, `relative_to`, `joinpath`, `as_posix`, `match` and the like — is left alone, and so is `open`, which `py/blocking-open-call-in-async-function` reports. Ruff also follows a name assigned a `Path(...)` or annotated `Path`; this rule reads the constructor written in the call, and leaves a name alone.

**Why:** A file-system call blocks the event loop for every other task while the disk answers.

```codelaw
utils:
  pure: {regex: '^(altsep|anchor|as_posix|as_uri|basename|commonpath|commonprefix|curdir|defpath|devnull|dirname|drive|expandvars|extsep|genericpath|is_absolute|is_relative_to|is_reserved|isabs|join|joinpath|match|name|normcase|open|os|pardir|parent|parents|parts|pathsep|relative_to|root|samestat|sep|split|splitdrive|splitext|splitroot|stem|suffix|suffixes|supports_unicode_filenames|sys|with_name|with_segments|with_stem|with_suffix)$'}
match:
  kind: call
  inside: {kind: function_definition, regex: '^async\s', stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}}
  has:
    field: function
    any:
      - kind: attribute
        regex: '^os\.path\.\w+$'
        has: {field: attribute, not: {matches: pure}}
      - kind: attribute
        all:
          - has: {field: attribute, not: {matches: pure}}
          - has:
              field: object
              kind: call
              has: {field: function, regex: '^(pathlib\.)?(Path|PosixPath|PurePath|PurePosixPath|PureWindowsPath|WindowsPath)$'}
message: "This call blocks the event loop on the file system"
```

## How to fix

1. Run the call in a worker thread (`await asyncio.to_thread(os.path.exists, p)`), or use an async path library such as `anyio.Path` or `trio.Path`.

## Examples

### Python

#### Don't

```python
import os.path


async def size_of(p):
    return os.path.getsize(p)
```

#### Do

```python
import asyncio
import os.path


async def size_of(p):
    return await asyncio.to_thread(os.path.getsize, p)
```
