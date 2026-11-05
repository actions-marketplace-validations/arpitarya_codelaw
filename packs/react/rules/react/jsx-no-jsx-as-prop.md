---
id: react/jsx-no-jsx-as-prop
title: "No JSX element built in a component's prop"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "hoisting or memoising the element moves code out of the render"
fix_exception: restructure
stability: preview
ported_from: [oxlint/react_perf/jsx-no-jsx-as-prop]
since: 0.5.0
---

A component's prop isn't a JSX element written in place: `<Card icon={<Star />} />`.

**Why:** It is a new element every render, so the child always re-renders.

```codelaw
native: react/jsx-no-jsx-as-prop
```

## How to fix

1. Hoist the element to a constant, or memoise it.

## Examples

### JavaScript

#### Don't

```js
const e = <Card icon={<Star />} />;
```

#### Do

```js
const star = <Star />;
const e = <Card icon={star} />;
```

### JSX

#### Don't

```jsx
const e = <Card icon={<Star />} />;
```

#### Do

```jsx
const star = <Star />;
const e = <Card icon={star} />;
```

### TSX

#### Don't

```tsx
const e = <Card icon={<Star />} />;
```

#### Do

```tsx
const star = <Star />;
const e = <Card icon={star} />;
```
