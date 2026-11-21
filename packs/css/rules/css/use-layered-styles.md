---
id: css/use-layered-styles
title: "Style rules sit in a `@layer`"
languages: [css]
fix: manual
fix_reason: "which layer a rule belongs in is the stylesheet's design"
fix_exception: restructure
stability: preview
ported_from: [biome/nursery/useLayeredStyles, eslint-plugin-css/use-layers]
since: 0.5.1
---

A style rule at the top of a stylesheet, or in a block that is not inside `@layer`, is reported (a rule nested in another rule is read as part of it). At-rules such as `@import`, `@font-face` and `@keyframes` are not style rules. Behaviour of Biome's `useLayeredStyles`; a stylesheet that does not use layers turns the rule off.

**Why:** Rules outside every layer beat the rules in any layer, whatever their specificity, so one stray selector overrides the whole layer order.

```codelaw
match:
  kind: rule_set
  not:
    inside:
      stopBy: end
      any:
        - kind: rule_set
        - kind: at_rule
          has: { kind: at_keyword, regex: '^@layer$' }
message: "Put this rule inside an `@layer` block"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### CSS

#### Don't

```css
a {
  color: red;
}
```

#### Do

```css
@layer base {
  a {
    color: red;
  }
}
```
