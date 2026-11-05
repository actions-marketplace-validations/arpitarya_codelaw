---
id: js/eqeqeq
title: "=== rather than =="
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`===` compares without converting, so code that relied on `\"1\" == 1` changes behaviour; the author confirms the types match"
stability: preview
ported_from: [eslint/eqeqeq]
since: 0.5.0
---

`==` and `!=` are written `===` and `!==`. Comparisons with `null` are left alone (`x == null` tests for both `null` and `undefined`). The fix writes the strict operator; it is `suggest`, as the result changes when the operands' types differ.

**Why:** `==` converts types before comparing, so `"0" == 0` and `[] == false` are true.

```codelaw
match:
  any:
    - pattern: $A == $B
    - pattern: $A != $B
  not: { any: [{ has: { field: right, kind: "null" } }, { has: { field: left, kind: "null" } }] }
  has: { field: operator, pattern: $$OP }
message: "Use `===` or `!==`; `==` and `!=` convert their operands"
fix:
  - replace: { target: $OP, with: "$OP|map(==:===, !=:!==)" }
```

## How to fix

1. Write `===` for `==` and `!==` for `!=`; check that both sides have the same type first.

## Examples

### JavaScript

#### Don't

```js
if (count == limit || state != "done") {
  stop();
}
```

#### Do

```js
if (count === limit || state !== "done") {
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
