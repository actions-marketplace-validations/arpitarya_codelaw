---
id: js/no-bitwise
title: "No bitwise operators"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether `&&` / `||` was meant, or the bit operation is intended, is the author's call"
stability: preview
ported_from: [eslint/no-bitwise]
since: 0.5.0
---

The bitwise operators `&`, `|`, `^`, `~`, `<<`, `>>` and `>>>` are not used.

**Why:** In most application code a bitwise operator is a typo for `&&` or `||`.

```codelaw
match:
  any:
    - kind: binary_expression
      has: { field: operator, regex: '^(&|\||\^|<<|>>|>>>)$' }
    - kind: unary_expression
      regex: '^~'
    - kind: augmented_assignment_expression
      regex: '(&|\||\^|<<|>>|>>>)=[^=]'
      not: { regex: '(&&|\|\||\?\?)=' }
message: "Unexpected bitwise operator"
```

## How to fix

1. Use `&&` or `||` if a logical operator was meant; otherwise turn the rule off for this line with a reason.

## Examples

### JavaScript

#### Don't

```js
if (a | b) run();
```

#### Do

```js
if (a || b) run();
```

### JSX

#### Don't

```jsx
if (a | b) run();
```

#### Do

```jsx
if (a || b) run();
```

### TypeScript

#### Don't

```ts
if (a | b) run();
```

#### Do

```ts
if (a || b) run();
```

### TSX

#### Don't

```tsx
if (a | b) run();
```

#### Do

```tsx
if (a || b) run();
```
