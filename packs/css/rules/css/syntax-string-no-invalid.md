---
id: css/syntax-string-no-invalid
title: "`@property` syntax strings are valid"
languages: [css, scss]
fix: suggest
fix_reason: "an unknown type name gets the names it is written for (`<size>` is `<length>` or `<length-percentage>`) or the names within two edits as choices, the first the default; `@property` is dropped today, so registering it changes behaviour. A string with another problem is manual, as the intended syntax is the author's call"
fix_exception: value
stability: stable
ported_from: [stylelint/syntax-string-no-invalid]
since: 0.1.0
---

The `syntax` of a `@property` rule is `"*"` or a `|`-separated list of supported `<type>` names and keywords, each with an optional `+` or `#`.

**Why:** A browser ignores a `@property` rule whose syntax is invalid, so the custom property is not registered.

```codelaw
native: css/syntax-string-no-invalid
```

## How to fix

1. Use a supported type name such as `<length>`, `<color>` or `<number>`.

## Examples

### CSS

#### Don't

```css
@property --size {
  syntax: "<size>";
  inherits: false;
  initial-value: 0px;
}
```

#### Fixed (length)

```css
@property --size {
  syntax: "<length>";
  inherits: false;
  initial-value: 0px;
}
```

#### Fixed (length-percentage)

```css
@property --size {
  syntax: "<length-percentage>";
  inherits: false;
  initial-value: 0px;
}
```

#### Do

```css
@property --size {
  syntax: "<length>";
  inherits: false;
  initial-value: 0px;
}
```
### SCSS

#### Don't

```scss
@property --size {
  syntax: "<size>";
  inherits: false;
  initial-value: 0px;
}
```

#### Fixed (length)

```scss
@property --size {
  syntax: "<length>";
  inherits: false;
  initial-value: 0px;
}
```

#### Fixed (length-percentage)

```scss
@property --size {
  syntax: "<length-percentage>";
  inherits: false;
  initial-value: 0px;
}
```

#### Do

```scss
@property --size {
  syntax: "<length>";
  inherits: false;
  initial-value: 0px;
}
```
