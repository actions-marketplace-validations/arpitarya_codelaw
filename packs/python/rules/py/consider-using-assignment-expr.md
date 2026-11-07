---
id: py/consider-using-assignment-expr
title: "An assignment tested straight after is an assignment expression"
languages: [python]
fix: suggest
fix_reason: "the line can grow past the line length, and the walrus needs Python 3.8; the author accepts the rewrite"
stability: preview
ported_from: [pylint/consider-using-assignment-expr]
since: 0.5.0
---

`match = pattern.search(line)` followed directly by `if match:` (or `if not match:`, or `if match is None:` with no `elif`) can be one statement: `if match := pattern.search(line):`. Only a one-line assignment of one name, with nothing in between, is read. Pylint also checks that the result fits the line length; this rule leaves that to the formatter. This is one of Pylint's optional extensions; it is in no preset.

**Why:** The name is bound where it is tested, and the statement that only feeds the test goes.

```codelaw
match:
  kind: if_statement
  all:
    - follows:
        stopBy: neighbor
        kind: expression_statement
        pattern: $PREV
        not: {regex: '\n'}
        has:
          kind: assignment
          not: {has: {field: type, kind: type}}
          all:
            - has: {field: left, kind: identifier, pattern: $N}
            - has:
                field: right
                pattern: $V
                not: {any: [{kind: expression_list}, {kind: assignment}, {kind: yield}, {kind: lambda}, {kind: conditional_expression}]}
    - any:
        - has: {field: condition, kind: identifier, all: [{pattern: $N}, {pattern: $C}]}
        - has: {field: condition, kind: not_operator, has: {kind: identifier, all: [{pattern: $N}, {pattern: $C}]}}
        - all:
            - has:
                field: condition
                kind: comparison_operator
                not: {has: {nthChild: 3}}
                has: {nthChild: 1, kind: identifier, all: [{pattern: $N}, {pattern: $C}]}
            - not: {has: {kind: elif_clause}}
message: "Assign `$N` in the `if` with `:=`"
fix:
  - remove: $PREV
  - replace: { target: $C, with: "($N := $V)" }
```

## Examples

### Python

#### Don't

```python
found = pattern.search(line)
if found:
    handle(found)
```

#### Do

```python
if (found := pattern.search(line)):
    handle(found)
```
