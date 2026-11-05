---
id: react/jsx-boolean-value
title: "A true prop is written bare"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react/jsx-boolean-value]
since: 0.5.0
---

A prop set to `{true}` is written as the bare prop name: `<Input disabled />`.

**Why:** `disabled` and `disabled={true}` mean the same; one spelling reads faster.

```codelaw
match:
  kind: jsx_attribute
  all:
    - has: { nthChild: 1, pattern: $P }
    - has: { kind: jsx_expression, regex: '^\{\s*true\s*\}$' }
message: "Write `$P`, not `$P={true}`"
fix: "$P"
```

## Examples

### JavaScript

#### Don't

```js
const i = <Input disabled={true} />;
```

#### Do

```js
const i = <Input disabled />;
```

### JSX

#### Don't

```jsx
const i = <Input disabled={true} />;
```

#### Do

```jsx
const i = <Input disabled />;
```

### TSX

#### Don't

```tsx
const i = <Input disabled={true} />;
```

#### Do

```tsx
const i = <Input disabled />;
```
