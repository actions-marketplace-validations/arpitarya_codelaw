---
id: js/no-unnecessary-nested-ternary
title: "No nested ternary that repeats a branch"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-nested-ternary]
since: 0.5.0
---

A ternary nested in another that shares one of its results is one ternary with a combined condition: `a ? (b ? x : y) : y` is `a && b ? x : y`, and `a ? x : (b ? x : y)` is `a || b ? x : y`.

**Why:** The repeated branch and the nesting hide a plain `&&` or `||`.

```codelaw
utils:
  operand:
    any:
      - kind: identifier
      - kind: member_expression
      - kind: subscript_expression
      - kind: call_expression
      - kind: unary_expression
      - kind: parenthesized_expression
      - kind: "true"
      - kind: "false"
      - kind: binary_expression
        has: { field: operator, regex: '^(===|!==|==|!=|<|>|<=|>=|instanceof|in|\+|-|\*|/|%)$' }
match:
  any:
    - all: [{ any: [{ pattern: "$A ? ($B ? $X : $Y) : $Y" }, { pattern: "$A ? $B ? $X : $Y : $Y" }] }, { pattern: $AND }]
    - all: [{ any: [{ pattern: "$A ? $X : ($B ? $X : $Y)" }, { pattern: "$A ? $X : $B ? $X : $Y" }] }, { pattern: $OR }]
constraints:
  A: { matches: operand }
  B: { matches: operand }
message: "Combine the conditions"
fix:
  - replace: { target: $AND, with: "$A && $B ? $X : $Y" }
  - replace: { target: $OR, with: "$A || $B ? $X : $Y" }
```

## Examples

### JavaScript

#### Don't

```js
const label = signedIn ? (isAdmin ? "admin" : "guest") : "guest";
```

#### Do

```js
const label = signedIn && isAdmin ? "admin" : "guest";
```


### JSX

#### Don't

```jsx
const label = signedIn ? (isAdmin ? "admin" : "guest") : "guest";
```

#### Do

```jsx
const label = signedIn && isAdmin ? "admin" : "guest";
```


### TypeScript

#### Don't

```ts
const label = signedIn ? (isAdmin ? "admin" : "guest") : "guest";
```

#### Do

```ts
const label = signedIn && isAdmin ? "admin" : "guest";
```


### TSX

#### Don't

```tsx
const label = signedIn ? (isAdmin ? "admin" : "guest") : "guest";
```

#### Do

```tsx
const label = signedIn && isAdmin ? "admin" : "guest";
```
