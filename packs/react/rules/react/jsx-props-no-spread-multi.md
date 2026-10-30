---
id: react/jsx-props-no-spread-multi
title: "One object is not spread twice into a JSX element"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "the later spread wins for every key it sets, so removing it can change a prop set between the two"
stability: preview
ported_from: [eslint-plugin-react/jsx-props-no-spread-multi]
since: 0.5.0
---

An element does not spread the same expression twice (`<A {...p} {...p} />`).

**Why:** The second spread repeats the first; it's almost always a copy-paste slip.

```codelaw
match:
  kind: jsx_expression
  all:
    - has: { kind: spread_element, pattern: $S }
    - follows: { stopBy: end, kind: jsx_expression, has: { kind: spread_element, pattern: $S } }
message: "`$S` is already spread into this element"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
const c = <Card {...props} title="x" {...props} />;
```

#### Do

```js
const c = <Card {...props} title="x" />;
```

#### Fixed

```js
const c = <Card {...props} title="x"  />;
```

### JSX

#### Don't

```jsx
const c = <Card {...props} title="x" {...props} />;
```

#### Do

```jsx
const c = <Card {...props} title="x" />;
```

#### Fixed

```jsx
const c = <Card {...props} title="x"  />;
```

### TSX

#### Don't

```tsx
const c = <Card {...props} title="x" {...props} />;
```

#### Do

```tsx
const c = <Card {...props} title="x" />;
```

#### Fixed

```tsx
const c = <Card {...props} title="x"  />;
```
