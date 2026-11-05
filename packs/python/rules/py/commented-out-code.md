---
id: py/commented-out-code
title: "No commented-out code"
languages: [python]
fix: suggest
fix_reason: "the comment is removed; review that it was not documentation"
stability: preview
ported_from: [ruff/ERA001]
since: 0.5.0
---

A comment that is a line of Python — an import, a definition, an assignment, a call, a `return` or a block header — is removed.

**Why:** Version control keeps old code; commented-out code goes stale and misleads.

```codelaw
match:
  kind: comment
  regex: '^#\s*(import\s+[\w.]+(\s+as\s+\w+)?(\s*,\s*[\w.]+)*$|from\s+[\w.]+\s+import\s+[\w., ()]+$|def\s+\w+\(.*:$|class\s+\w+.*:$|return(\s+[\w.()\[\]''"]+)?$|print\(.*\)$|(if|elif|for|while|with)\s.*:$|else:$|try:$|except\b.*:$|[a-zA-Z_][\w.]*\s*=\s*\S+$|[a-zA-Z_][\w.]*\(.*\)$)'
  not: {regex: '^#\s*(noqa|type:|pragma|fmt:|pylint:|TODO|FIXME|XXX|NOTE|-\*-|(en)?coding[:=])|^#!|https?:'}
message: "Remove the commented-out code"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
total = 0
# total = compute()
print(total)
```

#### Do

```python
total = 0
print(total)
```
