---
id: js/no-uniq-key
title: "No React key made fresh on each render"
languages: [jsx, tsx]
fix: manual
fix_reason: "a stable key comes from the data, which the author knows"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-uniq-key]
since: 0.5.0
---

A JSX `key` is not computed by `Math.random()`, `Date.now()`, `crypto.randomUUID()`, `uuid…()`, `nanoid()` or `uniqueId()`: a new key each render makes React drop and rebuild the element.

**Why:** Keys tell React which element is which across renders; a key that changes every time destroys state and costs a rebuild.

```codelaw
match:
  kind: jsx_attribute
  regex: '^key\s*='
  has:
    stopBy: end
    kind: call_expression
    has: { field: function, regex: '^(Math\.random|Date\.now|crypto\.randomUUID|uuid(v[1-7])?|v[1-7]|nanoid|_?\.?uniqueId)$' }
message: "A key made on each render changes every time; use a stable id"
```

## How to fix

1. Use an id the item already has (a database id, a slug), or one assigned once when the item is created.

## Examples

### JSX

#### Don't

```jsx
const list = items.map((item) => <Row key={Math.random()} item={item} />);
```

#### Do

```jsx
const list = items.map((item) => <Row key={item.id} item={item} />);
```


### TSX

#### Don't

```tsx
const list = items.map((item) => <Row key={Math.random()} item={item} />);
```

#### Do

```tsx
const list = items.map((item) => <Row key={item.id} item={item} />);
```
