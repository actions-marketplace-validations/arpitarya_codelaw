---
id: js/branches-sharing-code
title: "No statement repeated at the start or end of both branches"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "moving the statement out of the branches changes when it runs relative to the condition"
stability: preview
ported_from: [oxlint/oxc/branches-sharing-code]
since: 0.5.0
---

The two branches of an `if`/`else` don't start (or end) with the same statement.

**Why:** The repeated statement belongs outside the `if`.

```codelaw
native: js/branches-sharing-code
```

## How to fix

1. Move the statement before (or after) the `if`.

## Examples

### JavaScript

#### Don't

```js
if (a) {
  log();
  x();
} else {
  log();
  y();
}
```

#### Do

```js
log();
if (a) {
  x();
} else {
  y();
}
```

### JSX

#### Don't

```jsx
if (a) {
  log();
  x();
} else {
  log();
  y();
}
```

#### Do

```jsx
log();
if (a) {
  x();
} else {
  y();
}
```

### TypeScript

#### Don't

```ts
if (a) {
  log();
  x();
} else {
  log();
  y();
}
```

#### Do

```ts
log();
if (a) {
  x();
} else {
  y();
}
```

### TSX

#### Don't

```tsx
if (a) {
  log();
  x();
} else {
  log();
  y();
}
```

#### Do

```tsx
log();
if (a) {
  x();
} else {
  y();
}
```
