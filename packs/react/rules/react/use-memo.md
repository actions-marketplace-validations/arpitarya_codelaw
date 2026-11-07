---
id: react/use-memo
title: "useMemo's callback computes and returns a value"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "an `async` callback with no `await` loses `async`, so the value is cached instead of a promise of it, and parameters nothing reads are dropped; every other misuse is manual"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react-hooks/use-memo, eslint-plugin-react-hooks/void-use-memo, "@eslint-react/use-memo"]
since: 0.3.0
---

`useMemo(fn, deps)`'s callback is a plain synchronous function with no parameters that returns a
value, and the call's result is used. Reported: an `async` or generator callback, a callback that
takes parameters, a callback with a block body and no `return` of a value, and a `useMemo(…)` call
whose result is thrown away.

**Why:** `useMemo` caches the value its callback returns, calling it with no arguments: an `async` callback caches a promise, a parameter is always `undefined`, and a callback that returns nothing is a side effect in render, which belongs in `useEffect`.

```codelaw
native: react/use-memo
```

## How to fix

1. For a side effect, use `useEffect` instead.
2. For an async value, load it in an effect (or a data library) and keep it in state.
3. Read the values the callback needs from the component's scope instead of parameters, and return the result.

## Examples

### JavaScript

#### Don't

```js
function useTotal(items) {
  return useMemo(async () => sum(items), [items]);
}
```

#### Do

```js
function useTotal(items) {
  return useMemo(() => sum(items), [items]);
}
```

### JSX

#### Don't

```jsx
function useTotal(items) {
  return useMemo(async () => sum(items), [items]);
}
```

#### Do

```jsx
function useTotal(items) {
  return useMemo(() => sum(items), [items]);
}
```

### TypeScript

#### Don't

```ts
function useTotal(items) {
  return useMemo(async () => sum(items), [items]);
}
```

#### Do

```ts
function useTotal(items) {
  return useMemo(() => sum(items), [items]);
}
```

### TSX

#### Don't

```tsx
function useTotal(items) {
  return useMemo(async () => sum(items), [items]);
}
```

#### Do

```tsx
function useTotal(items) {
  return useMemo(() => sum(items), [items]);
}
```


## Differences from the source

- Upstream also reports a `useMemo` callback that reassigns a variable from outside it, and a callback given by name (`useMemo(fn, deps)`) rather than written in place; codelaw does neither.
- An unused result is reported only when the call is a statement of its own; upstream follows the value through the function.
