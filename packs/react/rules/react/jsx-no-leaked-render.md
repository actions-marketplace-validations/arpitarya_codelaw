---
id: react/jsx-no-leaked-render
title: "No && that can render 0 or \"\""
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "a ternary renders nothing where `&&` rendered `0` or `\"\"`, which is the intent but is a visible change"
stability: preview
ported_from: [eslint-plugin-react/jsx-no-leaked-render]
since: 0.5.0
---

A JSX child is not written `{x && <El />}` with a left side that may be a number or a string: `0 && …` renders `0`. A left side that is already a boolean — `!x`, a comparison, `Boolean(x)` — is fine.

**Why:** `{count && <Badge />}` shows a stray `0` when the count is zero, and on React Native it crashes.

```codelaw
match:
  kind: jsx_expression
  inside: { kind: jsx_element }
  has:
    kind: binary_expression
    pattern: $A && $B
    not:
      has:
        field: left
        any:
          - { kind: unary_expression, regex: '^!' }
          - { kind: binary_expression, regex: '^[\s\S]*?(===|!==|==|!=|<=|>=|<|>|\binstanceof\b|\bin\b)' , not: { regex: '&&|\|\|' } }
          - { kind: call_expression, regex: '^Boolean\(' }
          - { kind: "true" }
          - { kind: "false" }
message: "`$A` may render as `0` or `\"\"`; use a ternary"
fix: "{$A ? $B : null}"
```

## Examples

### JavaScript

#### Don't

```js
const l = <ul>{count && <Badge n={count} />}</ul>;
```

#### Do

```js
const l = <ul>{count > 0 && <Badge n={count} />}</ul>;
```

#### Fixed

```js
const l = <ul>{count ? <Badge n={count} /> : null}</ul>;
```

### JSX

#### Don't

```jsx
const l = <ul>{count && <Badge n={count} />}</ul>;
```

#### Do

```jsx
const l = <ul>{count > 0 && <Badge n={count} />}</ul>;
```

#### Fixed

```jsx
const l = <ul>{count ? <Badge n={count} /> : null}</ul>;
```

### TSX

#### Don't

```tsx
const l = <ul>{count && <Badge n={count} />}</ul>;
```

#### Do

```tsx
const l = <ul>{count > 0 && <Badge n={count} />}</ul>;
```

#### Fixed

```tsx
const l = <ul>{count ? <Badge n={count} /> : null}</ul>;
```
