---
id: py/hardcoded-string-charset
title: "A string module constant, not its characters spelled out"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FURB156]
since: 0.5.0
---

A string literal whose value is exactly one of the `string` module's constants — `ascii_letters`, `ascii_lowercase`, `ascii_uppercase`, `digits`, `hexdigits`, `octdigits`, `punctuation`, `printable`, `whitespace` — is written as that constant, `string.digits` for `"0123456789"`, and `import string` is added where the file has none. The characters must be in the constant's own order; a docstring, a bytes literal, an f-string and a literal joined to another by implicit concatenation are not read. `punctuation`, `printable` and `whitespace` are read in the spellings `repr()` gives them (`\x0b\x0c`, or `\v\f`), in either quote.

**Why:** A name says which characters are meant and cannot drop one; a run of forty characters can, and no one checks it.

```codelaw
match:
  kind: string
  all:
    - not: {inside: {kind: concatenated_string}}
    - not: {inside: {kind: expression_statement, nthChild: 1}}
  any:
    - all: [{regex: '^[uU]?(''abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ''|"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")$'}, {pattern: $ASCII_LETTERS}]
    - all: [{regex: '^[uU]?(''abcdefghijklmnopqrstuvwxyz''|"abcdefghijklmnopqrstuvwxyz")$'}, {pattern: $ASCII_LOWERCASE}]
    - all: [{regex: '^[uU]?(''ABCDEFGHIJKLMNOPQRSTUVWXYZ''|"ABCDEFGHIJKLMNOPQRSTUVWXYZ")$'}, {pattern: $ASCII_UPPERCASE}]
    - all: [{regex: '^[uU]?(''0123456789''|"0123456789")$'}, {pattern: $DIGITS}]
    - all: [{regex: '^[uU]?(''0123456789abcdefABCDEF''|"0123456789abcdefABCDEF")$'}, {pattern: $HEXDIGITS}]
    - all: [{regex: '^[uU]?(''01234567''|"01234567")$'}, {pattern: $OCTDIGITS}]
    - all: [{regex: '^[uU]?(''!"#\$%&\\''\(\)\*\+,-\./:;<=>\?@\[\\\\\]\^_`\{\|\}~''|"!\\"#\$%&''\(\)\*\+,-\./:;<=>\?@\[\\\\\]\^_`\{\|\}~")$'}, {pattern: $PUNCTUATION}]
    - all: [{regex: '^[uU]?(''0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!"#\$%&\\''\(\)\*\+,-\./:;<=>\?@\[\\\\\]\^_`\{\|\}~ \\t\\n\\r\\x0b\\x0c''|"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\\"#\$%&''\(\)\*\+,-\./:;<=>\?@\[\\\\\]\^_`\{\|\}~ \\t\\n\\r\\x0b\\x0c"|''0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!"#\$%&\\''\(\)\*\+,-\./:;<=>\?@\[\\\\\]\^_`\{\|\}~ \\t\\n\\r\\v\\f''|"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\\"#\$%&''\(\)\*\+,-\./:;<=>\?@\[\\\\\]\^_`\{\|\}~ \\t\\n\\r\\v\\f")$'}, {pattern: $PRINTABLE}]
    - all: [{regex: '^[uU]?('' \\t\\n\\r\\x0b\\x0c''|" \\t\\n\\r\\x0b\\x0c"|'' \\t\\n\\r\\v\\f''|" \\t\\n\\r\\v\\f")$'}, {pattern: $WHITESPACE}]
message: "Use the `string` module's constant"
fix:
  - replace: { target: $ASCII_LETTERS, with: "string.ascii_letters" }
  - replace: { target: $ASCII_LOWERCASE, with: "string.ascii_lowercase" }
  - replace: { target: $ASCII_UPPERCASE, with: "string.ascii_uppercase" }
  - replace: { target: $DIGITS, with: "string.digits" }
  - replace: { target: $HEXDIGITS, with: "string.hexdigits" }
  - replace: { target: $OCTDIGITS, with: "string.octdigits" }
  - replace: { target: $PUNCTUATION, with: "string.punctuation" }
  - replace: { target: $PRINTABLE, with: "string.printable" }
  - replace: { target: $WHITESPACE, with: "string.whitespace" }
  - ensure-import: { module: string }
```

## How to fix

1. Replace the literal with the `string` constant it spells, importing `string`.

## Examples

### Python

#### Don't

```python
codes = [c for c in text if c in "0123456789"]
```

#### Do

```python
import string

codes = [c for c in text if c in string.digits]
```
