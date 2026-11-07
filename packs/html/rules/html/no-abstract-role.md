---
id: html/no-abstract-role
title: "No abstract ARIA roles"
languages: [html]
fix: suggest
fix_reason: "the choices are removing the role, which browsers ignore anyway, or the concrete role it stands for (`landmark` as `region`); which is meant is the reviewer's call"
fix_exception: value
stability: preview
ported_from: [html-validate/no-abstract-role]
tags: [a11y]
since: 0.1.0
---

`role` does not name one of ARIA's abstract roles (`widget`, `input`, `landmark`, `section`, `command`, …).

**Why:** Abstract roles only organise the ARIA specification; browsers don't map them, so the element gets no role at all.

```codelaw
native: html/no-abstract-role
```

## How to fix

1. Use the concrete role that describes the element, or a native element.

## Examples

### HTML

#### Don't

```html
<div role="landmark">News</div>
```

#### Fixed (remove)

```html
<div>News</div>
```

#### Fixed (concrete role)

```html
<div role="region">News</div>
```

#### Do

```html
<div role="region" aria-label="News">News</div>
```
