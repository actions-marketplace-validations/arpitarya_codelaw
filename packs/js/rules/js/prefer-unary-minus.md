---
id: js/prefer-unary-minus
title: "Unary minus rather than multiplying by -1"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-unary-minus]
since: 0.5.0
---

A value is negated with `-x`, not `x * -1` or `-1 * x`.

**Why:** `-x` says negate; multiplying by `-1` makes the reader do the arithmetic.

```codelaw
utils:
  simple:
    any:
      - kind: identifier
      - kind: member_expression
      - kind: subscript_expression
      - kind: call_expression
      - kind: parenthesized_expression
match:
  any:
    - pattern: "$X * -1"
    - pattern: "-1 * $X"
constraints:
  X: { matches: simple }
message: "Negate with `-$X`"
fix: "-$X"
```

## Examples

### JavaScript

#### Don't

```js
const offset = delta * -1;
```

#### Do

```js
const offset = -delta;
```


### JSX

#### Don't

```jsx
const offset = delta * -1;
```

#### Do

```jsx
const offset = -delta;
```


### TypeScript

#### Don't

```ts
const offset = delta * -1;
```

#### Do

```ts
const offset = -delta;
```


### TSX

#### Don't

```tsx
const offset = delta * -1;
```

#### Do

```tsx
const offset = -delta;
```
