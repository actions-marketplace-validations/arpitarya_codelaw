---
id: react/jsx-no-new-array-as-prop
title: "No array literal in a component's prop"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "hoisting or memoising the array moves code out of the render"
fix_exception: restructure
stability: preview
ported_from: [oxlint/react_perf/jsx-no-new-array-as-prop]
since: 0.5.0
---

A component's prop isn't an array literal written in place: `<List items={[1, 2]} />`.

**Why:** It is a new array every render, so the child always re-renders.

```codelaw
native: react/jsx-no-new-array-as-prop
```

## How to fix

1. Hoist the array to a constant, or memoise it.

## Examples

### JavaScript

#### Don't

```js
const e = <List items={[1, 2]} />;
```

#### Do

```js
const items = [1, 2];
const e = <List items={items} />;
```

### JSX

#### Don't

```jsx
const e = <List items={[1, 2]} />;
```

#### Do

```jsx
const items = [1, 2];
const e = <List items={items} />;
```

### TSX

#### Don't

```tsx
const e = <List items={[1, 2]} />;
```

#### Do

```tsx
const items = [1, 2];
const e = <List items={items} />;
```
