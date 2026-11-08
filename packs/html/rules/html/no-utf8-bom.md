---
id: html/no-utf8-bom
title: "No UTF-8 byte-order mark"
languages: [html]
fix: auto
fix_reason: "the mark is in the file's bytes, outside its text: the fix is the file-level operation `strip_bom`, applied by `codelaw fix`, never on save or by the agent hook (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: [html-validate/no-utf8-bom]
since: 0.1.0
---

An HTML file doesn't start with a UTF-8 byte-order mark (`EF BB BF`).

**Why:** UTF-8 needs no byte-order mark. Served or concatenated, the mark can show up as stray characters, or break a server-side include or a `<!DOCTYPE>` that must come first.

```codelaw
native: html/no-utf8-bom
```

## How to fix

1. `codelaw fix` saves the file without its byte-order mark.
2. In an editor, save it as UTF-8 without a mark: in VS Code, click the encoding in the status bar, choose *Save with Encoding*, then *UTF-8*.

## Examples

### HTML

#### Don't

```html
﻿<!DOCTYPE html>
<html lang="en"><title>Home</title></html>
```

#### Do

```html
<!DOCTYPE html>
<html lang="en"><title>Home</title></html>
```

## Differences from the source

Only the file itself is checked: an HTML region inside another file (a Markdown fence, a template) holds no byte-order mark of its own.
