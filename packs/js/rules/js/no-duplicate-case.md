---
id: js/no-duplicate-case
title: "No two case labels in a switch are the same"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "no value reaches the later case: the whole clause goes when the clause before it exits, otherwise only its label, so what falls through still runs; the reviewer confirms the case was not meant to test another value"
stability: preview
ported_from: [eslint/no-duplicate-case]
since: 0.1.0
---

Each `case` in a `switch` tests a different expression.

**Why:** A repeated case can never match: the first one always wins.

```codelaw
native: js/no-duplicate-case
```

## How to fix

1. Find the label that was meant to test something else.
2. Correct it, or merge the two bodies.

## Examples

### JavaScript

#### Don't

```js
switch (status) {
  case 200:
    ok();
    break;
  case 200:
    retry();
    break;
}
```

#### Fixed

```js
switch (status) {
  case 200:
    ok();
    break;
}
```

#### Do

```js
switch (status) {
  case 200:
    ok();
    break;
  case 503:
    retry();
    break;
}
```

### JSX

#### Don't

```jsx
switch (status) {
  case 200:
    ok();
    break;
  case 200:
    retry();
    break;
}
```

#### Fixed

```jsx
switch (status) {
  case 200:
    ok();
    break;
}
```

#### Do

```jsx
switch (status) {
  case 200:
    ok();
    break;
  case 503:
    retry();
    break;
}
```

### TypeScript

#### Don't

```ts
switch (status) {
  case 200:
    ok();
    break;
  case 200:
    retry();
    break;
}
```

#### Fixed

```ts
switch (status) {
  case 200:
    ok();
    break;
}
```

#### Do

```ts
switch (status) {
  case 200:
    ok();
    break;
  case 503:
    retry();
    break;
}
```

### TSX

#### Don't

```tsx
switch (status) {
  case 200:
    ok();
    break;
  case 200:
    retry();
    break;
}
```

#### Fixed

```tsx
switch (status) {
  case 200:
    ok();
    break;
}
```

#### Do

```tsx
switch (status) {
  case 200:
    ok();
    break;
  case 503:
    retry();
    break;
}
```
