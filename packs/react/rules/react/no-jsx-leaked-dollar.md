---
id: react/no-jsx-leaked-dollar
title: "No `$` left in front of a JSX expression"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "the `$` is sometimes meant (a price); the author removes it or writes it as `{\"$\"}`"
stability: preview
ported_from: [biome/suspicious/noJsxLeakedDollar, "@eslint-react/jsx-no-leaked-dollar"]
since: 0.5.1
---

JSX text that ends in `$` directly before a `{…}` expression is reported: it is a template literal's `${…}` written in JSX, and renders a literal `$`. The suggestion removes the `$`. Behaviour of Biome's `noJsxLeakedDollar`.

**Why:** `<p>Total: ${total}</p>` prints `Total: $5`, not `Total: 5`: the `$` was carried over from a template string.

```codelaw
match:
  kind: jsx_expression
  follows: { kind: jsx_text, regex: '\$$', pattern: $T }
message: "A `$` before `{…}` is printed; remove it if it was copied from a template string"
fix:
  - edit-text: { target: $T, pattern: '\$$', replace: '' }
```

## Examples

### JavaScript

#### Don't

```js
const t = <p>Total: ${total}</p>;
```

#### Do

```js
const t = <p>Total: {total}</p>;
```

#### Fixed

```js
const t = <p>Total: {total}</p>;
```

### JSX

#### Don't

```jsx
const t = <p>Total: ${total}</p>;
```

#### Do

```jsx
const t = <p>Total: {total}</p>;
```

#### Fixed

```jsx
const t = <p>Total: {total}</p>;
```

### TSX

#### Don't

```tsx
const t = <p>Total: ${total}</p>;
```

#### Do

```tsx
const t = <p>Total: {total}</p>;
```

#### Fixed

```tsx
const t = <p>Total: {total}</p>;
```
