---
id: css/use-named-layer
title: "A `@layer` block has a name"
languages: [css]
fix: manual
fix_reason: "the layer's name and its place in the order are the author's design"
fix_exception: value
stability: preview
ported_from: [biome/nursery/useNamedLayer]
since: 0.5.1
---

A `@layer { … }` block without a name is reported. `@layer a, b;` statements and named blocks are fine. Behaviour of Biome's `useNamedLayer`.

**Why:** An anonymous layer cannot be reordered, extended or reached from another file, so it only adds a level to the cascade that nobody can name.

```codelaw
match:
  kind: at_rule
  all:
    - has: { kind: at_keyword, regex: '^@layer$' }
    - has: { kind: block }
    - not: { has: { kind: keyword_query } }
message: "Give this `@layer` block a name"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### CSS

#### Don't

```css
@layer {
  a { color: red; }
}
```

#### Do

```css
@layer base {
  a { color: red; }
}
```
