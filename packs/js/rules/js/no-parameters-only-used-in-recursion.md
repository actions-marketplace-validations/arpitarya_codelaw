---
id: js/no-parameters-only-used-in-recursion
title: "No parameter that is only passed to the function's own recursive call"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing a parameter changes the signature, which callers see; the suggestion only marks it unused with a leading underscore"
stability: preview
ported_from: [biome/suspicious/noParametersOnlyUsedInRecursion, clippy/only_used_in_recursion]
since: 0.5.1
---

A parameter of a named function (or of a method called as `this.name(…)`) whose every read is the argument, at its own position, of a recursive call — alone or as the left side of `||`/`??` — is reported. A parameter read anywhere else (returned, compared, passed to another function) is used. The suggestion prefixes the parameter and those arguments with `_`. Behaviour of Biome's `noParametersOnlyUsedInRecursion`.

**Why:** The value goes around the recursion and nothing ever looks at it, so the parameter does no work and the callers pay to compute it.

```codelaw
native: js/no-parameters-only-used-in-recursion
```

## Examples

### JavaScript

#### Don't

```js
function sum(n, acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, acc);
}
```

#### Do

```js
function sum(n, _acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, _acc);
}
```

#### Fixed

```js
function sum(n, _acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, _acc);
}
```

### JSX

#### Don't

```jsx
function sum(n, acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, acc);
}
```

#### Do

```jsx
function sum(n, _acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, _acc);
}
```

#### Fixed

```jsx
function sum(n, _acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, _acc);
}
```

### TypeScript

#### Don't

```ts
function sum(n, acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, acc);
}
```

#### Do

```ts
function sum(n, _acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, _acc);
}
```

#### Fixed

```ts
function sum(n, _acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, _acc);
}
```

### TSX

#### Don't

```tsx
function sum(n, acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, acc);
}
```

#### Do

```tsx
function sum(n, _acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, _acc);
}
```

#### Fixed

```tsx
function sum(n, _acc) {
  if (n === 0) return 0;
  return n + sum(n - 1, _acc);
}
```
