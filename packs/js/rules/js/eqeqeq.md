---
id: js/eqeqeq
title: "=== rather than =="
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the strict operator replaces the loose one token for token, and one fix template cannot carry both `===` and `!==`"
fix_pending: "suggest; needs computed-edit"
stability: preview
ported_from: [eslint/eqeqeq]
since: 0.5.0
---

`==` and `!=` are written `===` and `!==`. Comparisons with `null` are left alone (`x == null` tests for both `null` and `undefined`). The fix covers `==`; a `!=` finding is reported for the same reason.

**Why:** `==` converts types before comparing, so `"0" == 0` and `[] == false` are true.

```codelaw
match:
  any:
    - pattern: $A == $B
    - pattern: $A != $B
  not: { any: [{ has: { field: right, kind: "null" } }, { has: { field: left, kind: "null" } }] }
message: "Use `===` or `!==`; `==` and `!=` convert their operands"
```

## How to fix

1. Write `===` for `==` and `!==` for `!=`; check that both sides have the same type first.

## Examples

### JavaScript

#### Don't

```js
if (count == limit) {
  stop();
}
```

#### Do

```js
if (count === limit) {
  stop();
}
```

### JSX

#### Don't

```jsx
if (count == limit) {
  stop();
}
```

#### Do

```jsx
if (count === limit) {
  stop();
}
```

### TypeScript

#### Don't

```ts
if (count == limit) {
  stop();
}
```

#### Do

```ts
if (count === limit) {
  stop();
}
```

### TSX

#### Don't

```tsx
if (count == limit) {
  stop();
}
```

#### Do

```tsx
if (count === limit) {
  stop();
}
```
