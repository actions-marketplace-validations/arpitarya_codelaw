---
id: css/no-self-nested-layer
title: "No `@layer` inside a layer of the same name"
languages: [css]
fix: manual
fix_reason: "the inner layer is renamed or merged into the outer one, which the author decides"
fix_exception: restructure
stability: preview
ported_from: [biome/nursery/noSelfNestedLayer]
since: 0.5.1
---

A `@layer name { … }` block that holds another `@layer name` block is reported at the inner one. Behaviour of Biome's `noSelfNestedLayer`.

**Why:** The inner layer is `name.name`, a different layer from the outer one, which is almost never what the author meant.

```codelaw
match:
  kind: at_rule
  all:
    - has: { kind: at_keyword, regex: '^@layer$' }
    - has: { kind: keyword_query, pattern: $N }
    - inside:
        stopBy: end
        kind: at_rule
        all:
          - has: { kind: at_keyword, regex: '^@layer$' }
          - has: { kind: keyword_query, pattern: $N }
message: "`@layer $N` is nested in a layer of the same name"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### CSS

#### Don't

```css
@layer base {
  @layer base {
    a { color: red; }
  }
}
```

#### Do

```css
@layer base {
  @layer links {
    a { color: red; }
  }
}
```
