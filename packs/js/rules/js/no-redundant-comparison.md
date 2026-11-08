---
id: js/no-redundant-comparison
title: "No comparison an earlier one in the same `&&` chain already decides"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the subset: `auto` when the removed operand compares plain names with `===`/`!==`; a member read (a getter) or an ordering comparison (`valueOf`) could run code, so those are suggestions"
stability: preview
ported_from: [eslint-plugin-unicorn/no-redundant-comparison]
since: 0.5.0
---

In one `a && b && …` chain, a comparison is redundant when an earlier one already makes it true: the same comparison again, a looser bound after a stricter one (`x > 5 && x > 3`), or one made true through `===` links (`p === q && p === 3 && q === 3`). The redundant operand is reported and removed with its `&&`. A chain with a call, assignment or `new` between the comparisons, `==`, and optional chains are left alone.

**Why:** A comparison that can't change the result is noise, and often a sign that a different variable was meant.

```codelaw
native: js/no-redundant-comparison
```

## Examples

### JavaScript

#### Don't

```js
const ok = p === q && p === 3 && q === 3;
```

#### Do

```js
const ok = p === q && p === 3;
```

#### Fixed

```js
const ok = p === q && p === 3;
```


### JSX

#### Don't

```jsx
const ok = p === q && p === 3 && q === 3;
```

#### Do

```jsx
const ok = p === q && p === 3;
```

#### Fixed

```jsx
const ok = p === q && p === 3;
```


### TypeScript

#### Don't

```ts
const ok = p === q && p === 3 && q === 3;
```

#### Do

```ts
const ok = p === q && p === 3;
```

#### Fixed

```ts
const ok = p === q && p === 3;
```


### TSX

#### Don't

```tsx
const ok = p === q && p === 3 && q === 3;
```

#### Do

```tsx
const ok = p === q && p === 3;
```

#### Fixed

```tsx
const ok = p === q && p === 3;
```
