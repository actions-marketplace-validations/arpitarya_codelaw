---
id: py/logging-extra-attr-clash
title: "A logging extra does not reuse a LogRecord attribute"
languages: [python]
fix: manual
fix_reason: "the key's new name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/G101]
since: 0.5.0
---

A key in a logging call's `extra={...}` (or `extra=dict(...)`) is not one of `LogRecord`'s own attributes — `name`, `msg`, `args`, `module`, `lineno`, `message`, `asctime` and the rest.

**Why:** `logging` raises `KeyError` when an extra key would overwrite a record attribute, so the log call itself fails.

```codelaw
match:
  kind: keyword_argument
  inside:
    kind: argument_list
    inside: {kind: call, has: {field: function, regex: '\.(debug|info|warning|warn|error|critical|exception|log)$'}}
  all:
    - has: {field: name, regex: '^extra$'}
    - has:
        field: value
        any:
          - {kind: dictionary, has: {kind: pair, has: {field: key, kind: string, regex: '^["''](?:args|asctime|created|exc_info|exc_text|filename|funcName|levelname|levelno|lineno|module|msecs|message|msg|name|pathname|process|processName|relativeCreated|stack_info|thread|threadName|taskName)["'']$'}}}
          - {kind: call, all: [{has: {field: function, regex: '^dict$'}}, {has: {field: arguments, has: {kind: keyword_argument, has: {field: name, regex: '^(?:args|asctime|created|exc_info|exc_text|filename|funcName|levelname|levelno|lineno|module|msecs|message|msg|name|pathname|process|processName|relativeCreated|stack_info|thread|threadName|taskName)$'}}}}]}
message: "An `extra` key clashes with a `LogRecord` attribute"
```

## How to fix

1. Rename the key (`name` → `user_name`) so it no longer collides with a `LogRecord` attribute.

## Examples

### Python

#### Don't

```python
log.info("login", extra={"name": user})
```

#### Do

```python
log.info("login", extra={"user_name": user})
```
