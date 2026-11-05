---
id: js/no-typeof-undefined
title: "x === undefined rather than typeof x === \"undefined\""
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "for an undeclared global, `typeof` does not throw and the comparison does"
stability: preview
ported_from: [eslint-plugin-unicorn/no-typeof-undefined]
since: 0.5.0
---

A declared value is compared with `undefined` directly rather than through `typeof`. A global that may not exist is left alone.

**Why:** The direct comparison is shorter and a typo in it is caught; a typo in the string is not.

```codelaw
match:
  pattern: "typeof $X === \"undefined\""
constraints:
  X: { kind: identifier }
where:
  $X: { not: undeclared }
message: "Compare with `undefined` directly"
fix: "$X === undefined"
```

## Examples

### JavaScript

#### Don't

```js
function f(value) {
  if (typeof value === "undefined") {
    reset();
  }
}
```

#### Do

```js
function f(value) {
  if (value === undefined) {
    reset();
  }
}
```

### JSX

#### Don't

```jsx
function f(value) {
  if (typeof value === "undefined") {
    reset();
  }
}
```

#### Do

```jsx
function f(value) {
  if (value === undefined) {
    reset();
  }
}
```

### TypeScript

#### Don't

```ts
function f(value) {
  if (typeof value === "undefined") {
    reset();
  }
}
```

#### Do

```ts
function f(value) {
  if (value === undefined) {
    reset();
  }
}
```

### TSX

#### Don't

```tsx
function f(value) {
  if (typeof value === "undefined") {
    reset();
  }
}
```

#### Do

```tsx
function f(value) {
  if (value === undefined) {
    reset();
  }
}
```
