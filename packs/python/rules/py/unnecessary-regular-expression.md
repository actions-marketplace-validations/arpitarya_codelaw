---
id: py/unnecessary-regular-expression
title: "str.replace, not re.sub with a plain string"
languages: [python]
fix: suggest
fix_reason: "`str.replace` needs a str, not bytes, and takes no `count`/`flags` keywords"
stability: preview
ported_from: [ruff/RUF055]
since: 0.5.0
---

`re.sub` whose pattern and replacement are plain strings with no regex characters is written `s.replace(old, new)`. (Ruff's rule also rewrites `re.match`, `re.search`, `re.fullmatch` and `re.split`; those are left out.)

**Why:** A regex costs a compile and reads as if a pattern were involved.

```codelaw
match:
  pattern: re.sub($P, $R, $S)
constraints:
  P: {kind: string, regex: '^["''][^.*+?^$()\[\]{}|\\]*["'']$'}
  R: {kind: string, regex: '^["''][^\\]*["'']$'}
  S: {not: {kind: keyword_argument}}
message: "No regex here; use `$S.replace($P, $R)`"
fix: "$S.replace($P, $R)"
```

## Examples

### Python

#### Don't

```python
clean = re.sub("-", "_", name)
```

#### Do

```python
clean = name.replace("-", "_")
```
