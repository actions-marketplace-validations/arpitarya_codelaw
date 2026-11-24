---
id: react/use-unique-element-ids
title: "No static `id` on an element"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the id comes from `useId()` and the elements that point at it use the same value"
fix_exception: restructure
stability: preview
ported_from: [biome/correctness/useUniqueElementIds]
since: 0.5.1
---

A JSX `id` attribute is not a string literal (`id="name"` or `id={"name"}`). Behaviour of Biome's `useUniqueElementIds`.

**Why:** A component can be rendered twice on one page, and the second copy then repeats the id: labels, anchors and ARIA references point at the wrong element. `useId()` gives each copy its own.

```codelaw
match:
  kind: jsx_attribute
  all:
    - has: { kind: property_identifier, regex: '^id$' }
    - has: { any: [{ kind: string }, { kind: jsx_expression, has: { kind: string } }] }
message: "Use an id from `useId()` instead of a fixed string"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const f = <input id="email" />;
```

#### Do

```js
function F() {
  const id = useId();
  return <input id={id} />;
}
```

### JSX

#### Don't

```jsx
const f = <input id="email" />;
```

#### Do

```jsx
function F() {
  const id = useId();
  return <input id={id} />;
}
```

### TSX

#### Don't

```tsx
const f = <input id="email" />;
```

#### Do

```tsx
function F() {
  const id = useId();
  return <input id={id} />;
}
```
