---
id: js/no-useless-compound-assignment
title: "No x += 0 or x *= 1"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`+= 0` turns a string into itself and `undefined` into `NaN`, which the removal no longer does"
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-compound-assignment]
since: 0.5.0
---

A compound assignment with the identity value of its operator (`+= 0`, `*= 1`, …) is removed.

**Why:** It changes nothing and is usually a leftover.

```codelaw

match:
  kind: expression_statement
  has:
    kind: augmented_assignment_expression
    any:
      - all: [{ has: { field: operator, regex: '^(\+=|-=|\|=|\^=|<<=|>>=|>>>=)$' } }, { has: { field: right, regex: '^0$' } }]
      - all: [{ has: { field: operator, regex: '^(\*=|/=|\*\*=)$' } }, { has: { field: right, regex: '^1$' } }]
message: "This assignment does not change the value"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
total += 0;
send(total);
```

#### Do

```js
send(total);
```

### JSX

#### Don't

```jsx
total += 0;
send(total);
```

#### Do

```jsx
send(total);
```

### TypeScript

#### Don't

```ts
total += 0;
send(total);
```

#### Do

```ts
send(total);
```

### TSX

#### Don't

```tsx
total += 0;
send(total);
```

#### Do

```tsx
send(total);
```
