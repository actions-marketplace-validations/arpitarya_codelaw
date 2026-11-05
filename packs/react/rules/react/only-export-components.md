---
id: react/only-export-components
title: "A module that exports components exports only components"
languages: [jsx, tsx]
fix: manual
fix_reason: "moving the export to its own module changes its import paths"
fix_exception: restructure
stability: preview
ported_from: [oxlint/react/only-export-components]
since: 0.5.0
---

A `.jsx` or `.tsx` module that exports a component exports no other values (constants in `UPPER_CASE` and types are allowed).

**Why:** Fast Refresh keeps state only for modules that export components alone.

```codelaw
native: react/only-export-components
```

## How to fix

1. Move the other export to its own module.

## Examples

### JSX

#### Don't

```jsx
export function Card() {
  return null;
}
export const helper = () => 1;
```

#### Do

```jsx
export function Card() {
  return null;
}
```

### TSX

#### Don't

```tsx
export function Card() {
  return null;
}
export const helper = () => 1;
```

#### Do

```tsx
export function Card() {
  return null;
}
```
