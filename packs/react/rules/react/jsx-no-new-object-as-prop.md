---
id: react/jsx-no-new-object-as-prop
title: "No object literal in a component's prop"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "hoisting or memoising the object moves code out of the render"
fix_exception: restructure
stability: preview
ported_from: [oxlint/react_perf/jsx-no-new-object-as-prop]
since: 0.5.0
---

A component's prop isn't an object literal written in place: `<Map center={{ x: 0 }} />`.

**Why:** It is a new object every render, so the child always re-renders.

```codelaw
native: react/jsx-no-new-object-as-prop
```

## How to fix

1. Hoist the object to a constant, or memoise it.

## Examples

### JavaScript

#### Don't

```js
const e = <Map center={{ x: 0 }} />;
```

#### Do

```js
const center = { x: 0 };
const e = <Map center={center} />;
```

### JSX

#### Don't

```jsx
const e = <Map center={{ x: 0 }} />;
```

#### Do

```jsx
const center = { x: 0 };
const e = <Map center={center} />;
```

### TSX

#### Don't

```tsx
const e = <Map center={{ x: 0 }} />;
```

#### Do

```tsx
const center = { x: 0 };
const e = <Map center={center} />;
```
