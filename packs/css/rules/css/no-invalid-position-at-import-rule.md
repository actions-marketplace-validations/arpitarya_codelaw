---
id: css/no-invalid-position-at-import-rule
title: "`@import` comes first"
languages: [css]
fix: suggest
fix_reason: "the choices are moving the `@import` above the rules, where it starts to apply, or deleting it, which keeps what the page renders today; only the author knows which"
stability: stable
options:
  ignoreAtRules: { type: "string[]", default: [], description: "at-rules that may come before `@import`" }
ported_from: [stylelint/no-invalid-position-at-import-rule]
since: 0.1.0
---

Every `@import` comes before all other statements except `@charset` and `@layer` statements, and is not nested.

**Why:** Browsers ignore an `@import` that comes later, so the stylesheet is never loaded.

```codelaw
native: css/no-invalid-position-at-import-rule
```

## How to fix

1. Move the `@import` to the top of the stylesheet.

## Examples

### CSS

#### Don't

```css
a {
  color: red;
}

@import url("theme.css");
```

#### Fixed (move up)

```css
@import url("theme.css");
a {
  color: red;
}

```

#### Fixed (delete)

```css
a {
  color: red;
}


```

#### Do

```css
@import url("theme.css");

a {
  color: red;
}
```
