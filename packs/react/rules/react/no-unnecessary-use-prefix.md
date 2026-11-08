---
id: react/no-unnecessary-use-prefix
title: "A use… function calls a hook"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "auto when nothing imports the function: it is renamed without its `use` with every call in its module; an exported one is renamed in every project module that imports it too, as one fix, a diff to accept (SR-MULTI-FILE-FIXES)"
stability: preview
ported_from: ["@eslint-react/no-unnecessary-use-prefix"]
since: 0.3.0
---

A function named `use…` calls at least one hook somewhere in its body. Not reported: an empty
function, `use` and `useMDXComponents`, a function whose comments name a hook call (`// useX()`
later), and functions passed to a test mock (`vi.mock`, `jest.mock`, `mock.module`).

**Why:** The `use` prefix tells React's tools — and readers — that the Rules of Hooks apply; on a plain function it forbids calling it in a condition or a loop for no reason.

```codelaw
native: react/no-unnecessary-use-prefix
```

## How to fix

1. `codelaw fix` drops the `use` prefix and renames every call; for an exported one, check the diff and accept it with `codelaw fix --unsafe`.
2. If the function should be a hook, make it call one instead (keep its name).

## Examples

### JavaScript

#### Don't

```js
function useFormatDate(date) {
  return date.toISOString().slice(0, 10);
}
```

#### Do

```js
function formatDate(date) {
  return date.toISOString().slice(0, 10);
}
```

### JSX

#### Don't

```jsx
const useGreeting = (name) => <p>Hello, {name}</p>;
```

#### Do

```jsx
const greeting = (name) => <p>Hello, {name}</p>;
```

### TypeScript

#### Don't

```ts
function useTotal(items: number[]): number {
  return items.reduce((a, b) => a + b, 0);
}
```

#### Fixed

```ts
function total(items: number[]): number {
  return items.reduce((a, b) => a + b, 0);
}
```

#### Do

```ts
function useTotal(items: number[]): number {
  return useMemo(() => items.reduce((a, b) => a + b, 0), [items]);
}
```

### TSX

#### Don't

```tsx
const useGreeting = (name: string) => <p>Hello, {name}</p>;
```

#### Do

```tsx
const greeting = (name: string) => <p>Hello, {name}</p>;
```

## Differences from the source

- A hook call anywhere in the body counts, nested callbacks included; upstream counts the hook calls its collector attributes to the function.
