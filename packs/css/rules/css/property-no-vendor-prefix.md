---
id: css/property-no-vendor-prefix
title: "No vendor-prefixed properties"
languages: [css, scss]
fix: auto
fix_target: { browsers: "css-property:$FIXED" }
fix_reason: "browsers that need the prefix lose the declaration; automatic only where every target browser supports the result (fix_target)"
stability: preview
options:
  ignoreProperties: { type: "string[]", default: [], description: "properties to accept; `/regex/` for a pattern" }
ported_from: [stylelint/property-no-vendor-prefix]
since: 0.1.0
---

Properties a build tool prefixes, such as `-webkit-transform`, are written without the prefix.

**Why:** Prefixing is a build step; hand-written prefixes go stale and duplicate code.

```codelaw
native: css/property-no-vendor-prefix
```

## Examples

### CSS

#### Don't

```css
a {
  -webkit-transform: scale(2);
}
```

#### Do

```css
a {
  transform: scale(2);
}
```
### SCSS

#### Don't

```scss
a {
  -webkit-transform: scale(2);
}
```

#### Do

```scss
a {
  transform: scale(2);
}
```
