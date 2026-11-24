---
id: css/no-value-at-rule
title: "No `@value`"
languages: [css]
fix: manual
fix_reason: "the value becomes a custom property or a Sass variable, and the files that import it change"
fix_exception: restructure
stability: preview
ported_from: [biome/style/noValueAtRule]
since: 0.5.1
---

`@value` (a CSS Modules at-rule that is not CSS) is not used. A custom property (`--primary`) or a preprocessor variable does the same job. Behaviour of Biome's `noValueAtRule`.

**Why:** `@value` works only where a CSS Modules loader reads the file; every other tool treats it as an unknown at-rule.

```codelaw
match:
  kind: at_rule
  has: { kind: at_keyword, regex: '^@value$' }
message: "Use a custom property instead of `@value`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### CSS

#### Don't

```css
@value primary: #00f;
```

#### Do

```css
:root {
  --primary: #00f;
}
```
