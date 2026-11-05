---
id: js/consistent-conditional-object-spread
title: "Spread a conditional object one way"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "if the condition is a truthy non-object (a number or a string), `&&` spreads it"
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-conditional-object-spread]
since: 0.5.0
---

A conditional object spread is written `...(cond && {…})`, not `...(cond ? {…} : {})`.

**Why:** Spreading `false`, `null` or `undefined` into an object adds nothing, so the empty branch is noise.

```codelaw

match:
  kind: spread_element
  inside: { kind: object }
  has:
    kind: parenthesized_expression
    has: { kind: ternary_expression, has: { field: alternative, kind: object, regex: '^\{\s*\}$' }, pattern: "$C ? $O : {}" }
message: "Write `...($C && $O)`; spreading `false` adds nothing"
fix: "...($C && $O)"
```

## Examples

### JavaScript

#### Don't

```js
const opts = { ...base, ...(debug ? { verbose: true } : {}) };
```

#### Do

```js
const opts = { ...base, ...(debug && { verbose: true }) };
```

### JSX

#### Don't

```jsx
const opts = { ...base, ...(debug ? { verbose: true } : {}) };
```

#### Do

```jsx
const opts = { ...base, ...(debug && { verbose: true }) };
```

### TypeScript

#### Don't

```ts
const opts = { ...base, ...(debug ? { verbose: true } : {}) };
```

#### Do

```ts
const opts = { ...base, ...(debug && { verbose: true }) };
```

### TSX

#### Don't

```tsx
const opts = { ...base, ...(debug ? { verbose: true } : {}) };
```

#### Do

```tsx
const opts = { ...base, ...(debug && { verbose: true }) };
```
