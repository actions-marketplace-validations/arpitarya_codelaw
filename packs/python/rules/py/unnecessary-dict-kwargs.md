---
id: py/unnecessary-dict-kwargs
title: "No **{...} of literal keys in a call"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PIE804]
since: 0.5.0
---

A call does not unpack a dict literal whose keys are all identifier strings: `f(**{"a": 1})` is written `f(a=1)`. A dict with a key that is not an identifier, or a Python keyword, is left alone.

**Why:** Keyword arguments say the same thing without building a dict.

```codelaw
match:
  kind: dictionary_splat
  pattern: $S
  has:
    kind: dictionary
    pattern: $D
    regex: '^\{\s*(["''][A-Za-z_]\w*["'']\s*:\s*[^:{}"''\n]+?\s*,\s*)*["''][A-Za-z_]\w*["'']\s*:\s*[^:{}"''\n]+?\s*,?\s*\}$'
    not: {has: {kind: pair, has: {field: key, regex: '^["''](False|None|True|and|as|assert|async|await|break|class|continue|def|del|elif|else|except|finally|for|from|global|if|import|in|is|lambda|nonlocal|not|or|pass|raise|return|try|while|with|yield)["'']$'}}}
  inside: {kind: argument_list}
transform:
  I: { substring: { source: $D, startChar: 1, endChar: -1 } }
  K: { replace: { source: $I, replace: '["'']([A-Za-z_]\w*)["'']\s*:\s*', by: '$1=' } }
  T: { replace: { source: $K, replace: ',\s*$', by: '' } }
message: "Pass the keys as keyword arguments"
fix: "$T"
```

## Examples

### Python

#### Don't

```python
connect(**{"host": h, "port": 5432})
```

#### Do

```python
connect(host=h, port=5432)
```
