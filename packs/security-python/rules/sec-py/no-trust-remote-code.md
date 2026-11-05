---
id: sec-py/no-trust-remote-code
title: 'No `trust_remote_code=True`'
languages: [python]
fix: suggest
fix_reason: "removing the keyword fails closed, but a model that needs its custom code stops loading, so the reviewer confirms it does not, or vendors and pins the code"
stability: preview
security: { cwe: [94], owasp: ["A03:2025"], severity: high }
since: 0.2.0
---

`from_pretrained`, `load_dataset` and `pipeline` are not called with `trust_remote_code=True`.

**Why:** The repository's Python files are downloaded and run on your machine, so whoever can push to it runs code in your process (CWE-94).

```codelaw
match:
  all:
    - { kind: call }
    - has:
        field: arguments
        has:
          kind: keyword_argument
          pattern: $K
          all:
            - { has: { field: name, regex: '^trust_remote_code$' } }
            - { has: { field: value, kind: 'true' } }
message: 'This runs code from the model repository; review it and vendor it, or use a model without custom code'
fix:
  - remove: $K
```

## How to fix

1. Remove `trust_remote_code=True`; use a model whose architecture is built into the library.
2. If you need the code, read it, copy it into your repository at a pinned commit, and load that instead.
3. At least pin `revision=` to a commit you reviewed.

## Examples

### Python

#### Don't

```py
model = AutoModel.from_pretrained(name, trust_remote_code=True)
```

#### Fixed

```py
model = AutoModel.from_pretrained(name)
```

#### Do

```py
model = AutoModel.from_pretrained(name, revision=REVISION)
```
