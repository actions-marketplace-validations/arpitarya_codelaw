---
id: py/doc-line-too-long
title: "Comment and docstring lines are not longer than the limit"
languages: [python]
fix: manual
fix_reason: "where to break the text is the author's"
fix_exception: value
stability: preview
options: { max_doc_length: { type: "integer", default: 0, description: "the longest comment or docstring line, in columns; 0 turns the rule off" } }
ported_from: [ruff/W505]
since: 0.5.0
---

A comment on a line of its own, or a line of a string statement (a docstring), is not wider than `max_doc_length` columns (0, the default, turns the rule off). A tab counts to the next multiple of four; every other character counts as one. A line of one word, a line that ends in a URL which starts within the limit, and an SPDX header are not reported.

**Why:** Prose is read in a narrow column; code line length is the formatter's business, but a long comment is the author's.

```codelaw
native: py/doc-line-too-long
```

## How to fix

1. Wrap the line.

## Examples

### Python

#### Don't

```python options={"max_doc_length":40}
# This comment goes on for a long time, well past the column limit set here.
x = 1
```

#### Do

```python options={"max_doc_length":40}
# This comment is short.
x = 1
```
