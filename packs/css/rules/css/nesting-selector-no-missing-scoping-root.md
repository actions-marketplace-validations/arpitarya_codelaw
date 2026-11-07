---
id: css/nesting-selector-no-missing-scoping-root
title: "Nesting selectors have a parent"
languages: [css, scss]
fix: suggest
fix_reason: "a leading `& ` with no parent rule goes, so the selector stands on its own; the reviewer confirms it wasn't meant to be nested. A bare `&` is manual"
fix_exception: restructure
stability: stable
ported_from: [stylelint/nesting-selector-no-missing-scoping-root]
since: 0.1.0
---

The nesting selector `&` is used only inside a style rule (or under `@scope`), where it stands for the parent selector.

**Why:** At the top level `&` matches the root element, which is almost never what was meant.

```codelaw
native: css/nesting-selector-no-missing-scoping-root
```

## How to fix

1. Nest the rule inside the rule `&` should refer to, or replace `&` with that selector.

## Examples

### CSS

#### Don't

```css
& .item {
  color: red;
}
```

#### Fixed

```css
.item {
  color: red;
}
```

#### Do

```css
.list {
  & .item {
    color: red;
  }
}
```
### SCSS

#### Don't

```scss
& .item {
  color: red;
}
```

#### Fixed

```scss
.item {
  color: red;
}
```

#### Do

```scss
.list {
  & .item {
    color: red;
  }
}
```
