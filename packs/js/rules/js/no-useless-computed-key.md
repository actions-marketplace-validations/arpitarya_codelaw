---
id: js/no-useless-computed-key
title: "No computed key that is a literal"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/no-useless-computed-key]
since: 0.5.0
---

An object or class key is not written `["name"]` when `name` (or `"name"`) says the same.

**Why:** The brackets suggest a computed key where there is none.

```codelaw
match:
  kind: computed_property_name
  has: { any: [{kind: string}, {kind: number}], pattern: $K }
  not: { has: { kind: string, regex: '^["'']__proto__["'']$' } }
  inside: { any: [{kind: pair}, {kind: method_definition}, {kind: field_definition}, {kind: public_field_definition}, {kind: pair_pattern}] }
message: "This computed key is a literal; drop the brackets"
fix: "$K"
```

## Examples

### JavaScript

#### Don't

```js
const o = { ['a']: 1 };
```

#### Do

```js
const o = { 'a': 1 };
```

#### Fixed

```js
const o = { 'a': 1 };
```

### JSX

#### Don't

```jsx
const o = { ['a']: 1 };
```

#### Do

```jsx
const o = { 'a': 1 };
```

#### Fixed

```jsx
const o = { 'a': 1 };
```

### TypeScript

#### Don't

```ts
const o = { ['a']: 1 };
```

#### Do

```ts
const o = { 'a': 1 };
```

#### Fixed

```ts
const o = { 'a': 1 };
```

### TSX

#### Don't

```tsx
const o = { ['a']: 1 };
```

#### Do

```tsx
const o = { 'a': 1 };
```

#### Fixed

```tsx
const o = { 'a': 1 };
```
