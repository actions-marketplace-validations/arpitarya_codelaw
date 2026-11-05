---
id: py/subprocess-popen-preexec-fn
title: "No preexec_fn"
languages: [python]
fix: manual
fix_reason: "what replaces the function (`start_new_session`, `process_group`, `user`) depends on what it does"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLW1509]
since: 0.5.0
---

`subprocess` calls do not pass `preexec_fn`.

**Why:** It is not safe with threads and can deadlock the child; the newer keyword arguments cover most uses.

```codelaw
match:
  kind: keyword_argument
  has: {field: name, regex: '^preexec_fn$'}
  not: {has: {field: value, kind: none}}
  inside: {kind: argument_list, inside: {kind: call, has: {field: function, regex: '^subprocess\.(Popen|run|call|check_call|check_output)$'}}}
message: "`preexec_fn` is not thread-safe; use `start_new_session`, `process_group` or `user`"
```

## How to fix

1. Replace it with the keyword that does what the function did (`start_new_session=True`, `process_group=0`, `user=`, `umask=`).

## Examples

### Python

#### Don't

```python
subprocess.Popen(cmd, preexec_fn=os.setsid)
```

#### Do

```python
subprocess.Popen(cmd, start_new_session=True)
```
