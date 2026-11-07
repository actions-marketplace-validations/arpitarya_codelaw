---
id: html/no-duplicate-id
title: "Ids are unique in the document"
languages: [html]
fix: suggest
fix_reason: "the choices are dropping the later id or renaming it `<id>-2`; links, labels and scripts that meant the later element then need the new id, which the reviewer checks"
stability: preview
ported_from: [html-validate/no-dup-id, htmlhint/id-unique]
tags: [a11y]
since: 0.1.0
---

No two elements in the document have the same `id`. Ids inside a `<template>` only need to be unique within it.

**Why:** `getElementById`, `label for`, `aria-labelledby` and fragment links all find the first element with the id and miss the others.

```codelaw
native: html/no-duplicate-id
```

## How to fix

1. Decide which element keeps the id.
2. Rename the other one, and update the CSS, scripts, `for`, `aria-*` and links that refer to it.

## Examples

### HTML

#### Don't

```html
<h2 id="intro">Intro</h2>
<p id="intro">Text</p>
```

#### Fixed (drop the id)

```html
<h2 id="intro">Intro</h2>
<p>Text</p>
```

#### Fixed (rename)

```html
<h2 id="intro">Intro</h2>
<p id="intro-2">Text</p>
```

#### Do

```html
<h2 id="intro">Intro</h2>
<p id="intro-text">Text</p>
```
