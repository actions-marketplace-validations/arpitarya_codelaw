---
id: css/at-rule-no-vendor-prefix
title: "No vendor-prefixed at-rules"
languages: [css, scss]
fix: auto
fix_target: { browsers: "css-at-rule:$FIXED" }
fix_reason: "browsers that need the prefix lose the rule; automatic only where every target browser supports the result (fix_target)"
stability: preview
options:
  ignoreAtRules: { type: "string[]", default: [], description: "at-rules to accept; `/regex/` for a pattern" }
ported_from: [stylelint/at-rule-no-vendor-prefix]
since: 0.1.0
---

At-rules a build tool prefixes, such as `@-webkit-keyframes`, are written without the prefix.

**Why:** Prefixing is a build step; hand-written prefixes go stale and duplicate code.

```codelaw
native: css/at-rule-no-vendor-prefix
```

## Examples

### CSS

#### Don't

```css
@-webkit-keyframes fade {
  from {
    opacity: 0;
  }
}
```

#### Do

```css
@keyframes fade {
  from {
    opacity: 0;
  }
}
```
### SCSS

#### Don't

```scss
@-webkit-keyframes fade {
  from {
    opacity: 0;
  }
}
```

#### Do

```scss
@keyframes fade {
  from {
    opacity: 0;
  }
}
```
