---
id: css/no-duplicate-selector
title: "No selector declared twice"
languages: [css, scss]
fix: auto
fix_reason: "auto for a selector repeated in its own list, which goes, and for two rules side by side with the same selector, which cascade as one and merge; rules apart are manual, since moving declarations past others changes the cascade"
fix_exception: restructure
stability: preview
ported_from: [stylelint/no-duplicate-selectors]
since: 0.1.0
---

Two rules in the same at-rule context do not have the same selector list (order and whitespace aside, nesting resolved), and one selector list does not name a selector twice.

**Why:** Split declarations for one selector are hard to follow and often contradict each other.

```codelaw
native: css/no-duplicate-selector
```

## How to fix

1. Merge the declarations of both rules into the first.
2. Check that rules between them did not depend on the order.

## Examples

### CSS

#### Don't

```css
.card {
  color: red;
}

.card {
  margin: 0;
}
```

#### Do

```css
.card {
  color: red;
  margin: 0;
}
```
### SCSS

#### Don't

```scss
.card {
  color: red;
}

.card {
  margin: 0;
}
```

#### Do

```scss
.card {
  color: red;
  margin: 0;
}
```
