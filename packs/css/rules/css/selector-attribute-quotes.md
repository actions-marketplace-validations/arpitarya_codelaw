---
id: css/selector-attribute-quotes
title: "Attribute selector values are quoted"
languages: [css, scss]
fix: auto
stability: preview
ported_from: [stylelint/selector-attribute-quotes]
since: 0.5.0
---

The value in an attribute selector is quoted: `[type=text]` is `[type="text"]`.

**Why:** A quoted value can hold any character.

```codelaw
native: css/selector-attribute-quotes
```

## Examples

### CSS

#### Don't

```css
[type=text] {
  color: red;
}
```

#### Do

```css
[type="text"] {
  color: red;
}
```

### SCSS

#### Don't

```scss
[type=text] {
  color: red;
}
```

#### Do

```scss
[type="text"] {
  color: red;
}
```
