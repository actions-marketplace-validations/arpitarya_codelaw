---
id: html/class-pattern
title: "Class names follow the project's pattern"
languages: [html]
fix: manual
fix_reason: "renaming the class must also change the stylesheets and scripts that use it"
fix_exception: value
stability: preview
options:
  pattern: { type: "string", default: "^[a-z][a-z0-9]*(-[a-z0-9]+)*$", description: "a regular expression class names must match" }
ported_from: [html-validate/class-pattern]
since: 0.5.0
---

A class name matches `pattern`, kebab-case by default.

**Why:** Consistent names are predictable.

```codelaw
native: html/class-pattern
```

## How to fix

1. Rename the class, and its uses in CSS and scripts.

## Examples

### HTML

#### Don't

```html
<div class="navBar"></div>
```

#### Do

```html
<div class="nav-bar"></div>
```
