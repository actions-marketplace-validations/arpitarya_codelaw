---
id: react/no-ref-in-render
title: "A ref's current value is not read or written during render"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a body-level `ref.current = …` moves into `useLayoutEffect(() => { … })`, run after every render before paint, the hook added to the `react` import; a ref read during render is manual"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react-hooks/refs, "@eslint-react/refs"]
since: 0.3.0
---

A component or hook doesn't read or write `ref.current` in its own body, for a ref made by
`useRef` in that body. Reads and writes in effects, event handlers and other callbacks are fine,
and so is the one lazy initialisation React documents: `if (ref.current === null) { ref.current
= … }` (or `== null`, `=== undefined`, `!ref.current`).

**Why:** React doesn't re-render when a ref changes, so a value read during render can be stale, and a write during render runs again — or not at all — when React renders a component more than once or throws a render away.

```codelaw
native: react/no-ref-in-render
```

## How to fix

1. Move the read or write into an effect, an event handler or another callback.
2. If the value is shown on screen, keep it in state instead of a ref.
3. If the ref is created lazily, initialise it inside `if (ref.current === null) { … }`.

## Examples

### JavaScript

#### Don't

```js
function useLatest(value) {
  const ref = useRef(value);
  ref.current = value;
  return ref;
}
```

#### Do

```js
function useLatest(value) {
  const ref = useRef(value);
  useLayoutEffect(() => {
    ref.current = value;
  });
  return ref;
}
```

### JSX

#### Don't

```jsx
function useLatest(value) {
  const ref = useRef(value);
  ref.current = value;
  return ref;
}
```

#### Do

```jsx
function useLatest(value) {
  const ref = useRef(value);
  useLayoutEffect(() => {
    ref.current = value;
  });
  return ref;
}
```

### TypeScript

#### Don't

```ts
function useLatest<T>(value: T) {
  const ref = useRef(value);
  ref.current = value;
  return ref;
}
```

#### Fixed

```ts
function useLatest<T>(value: T) {
  const ref = useRef(value);
  useLayoutEffect(() => {
    ref.current = value;
  });
  return ref;
}
```

#### Do

```ts
function useStore() {
  const store = useRef<Store | null>(null);
  if (store.current === null) {
    store.current = createStore();
  }
  return store;
}
```

### TSX

#### Don't

```tsx
function useLatest(value) {
  const ref = useRef(value);
  ref.current = value;
  return ref;
}
```

#### Do

```tsx
function useLatest(value) {
  const ref = useRef(value);
  useLayoutEffect(() => {
    ref.current = value;
  });
  return ref;
}
```


## Differences from the source

- Only refs made by `useRef` in the same function are checked; upstream also tracks refs passed in as props or returned by other hooks, and reports a ref passed to a function called during render.
- A second lazy initialisation of the same ref, which upstream reports, is not reported.
