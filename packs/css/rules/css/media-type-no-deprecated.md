---
id: css/media-type-no-deprecated
title: "No deprecated media types"
languages: [css, scss]
fix: auto
fix_reason: "auto when the deprecated type is a query of its own in a list with another query: it matches nothing, so it goes; a list of only that query is manual"
fix_exception: restructure
stability: stable
options:
  ignoreMediaTypes: { type: "string[]", default: [], description: "media types to accept" }
ported_from: [stylelint/media-type-no-deprecated]
since: 0.1.0
---

Media queries use no media type the specifications deprecated: `tv`, `handheld`, `projection`, `aural`, `braille`, `embossed`, `speech` and `tty`.

**Why:** Deprecated media types match nothing, so the block never applies.

```codelaw
native: css/media-type-no-deprecated
```

## How to fix

1. Replace the media type with `screen`, `print` or `all`, adding media features that describe the device.

## Examples

### CSS

#### Don't

```css
@media tv, print {
  a {
    color: red;
  }
}
```

#### Fixed

```css
@media print {
  a {
    color: red;
  }
}
```

#### Do

```css
@media screen {
  a {
    color: red;
  }
}
```
### SCSS

#### Don't

```scss
@media tv, print {
  a {
    color: red;
  }
}
```

#### Fixed

```scss
@media print {
  a {
    color: red;
  }
}
```

#### Do

```scss
@media screen {
  a {
    color: red;
  }
}
```
