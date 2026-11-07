---
id: css/no-descending-specificity
title: "Less specific selectors come first"
languages: [css, scss]
fix: auto
fix_reason: "auto when the more specific rule comes right before, nothing but whitespace between, each with one selector and only declarations: between two rules whose specificities differ, specificity decides and not order, so the swap changes nothing; a rule further up is manual, as moving one past others can change which declarations win"
fix_exception: restructure
stability: preview
options:
  ignore: { type: "string[]", default: [], description: "`selectors-within-list` skips rules with a selector list" }
ported_from: [stylelint/no-descending-specificity]
since: 0.1.0
---

A selector does not come after a more specific selector that targets the same element (the same last compound selector, pseudo-classes aside), within the same at-rule.

**Why:** The earlier, more specific rule wins whatever the order, so reading the sheet top to bottom misleads.

```codelaw
native: css/no-descending-specificity
```

## How to fix

1. Move the less specific rule above the more specific one.
2. Check that the page still renders the same.

## Examples

### CSS

#### Don't

```css
#nav a {
  color: red;
}

a {
  color: blue;
}
```

#### Do

```css
a {
  color: blue;
}

#nav a {
  color: red;
}
```

### SCSS

#### Don't

```scss
#nav a {
  color: red;
}

a {
  color: blue;
}
```

#### Do

```scss
a {
  color: blue;
}

#nav a {
  color: red;
}
```

## Differences from the source

A nested selector's `&` is resolved against each parent selector separately; stylelint uses the highest specificity among them, as `:is()` would.
