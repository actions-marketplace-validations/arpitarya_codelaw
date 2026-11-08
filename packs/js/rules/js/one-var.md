---
id: js/one-var
title: "One declaration per scope for each of `var`, `let` and `const`"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the subset: two adjacent declarations of one kind are merged; a later one further away is reported without a fix"
stability: preview
ported_from: [eslint/one-var]
since: 0.5.0
---

ESLint's default (`always`): each scope declares its variables in one statement per kind — `var`s once per function, `let`s and `const`s once per block. A later declaration of the same kind in that scope is reported; when it directly follows the earlier one, the fix merges them (`let a = 1, b = 2;`). Loop heads and ambient declarations are left alone.

**Why:** A taste rule: one declaration lists a scope's variables in one place.

```codelaw
native: js/one-var
```

## Examples

### JavaScript

#### Don't

```js
let a = 1;
let b = 2;
```

#### Do

```js
let a = 1, b = 2;
```

#### Fixed

```js
let a = 1, b = 2;
```


### JSX

#### Don't

```jsx
let a = 1;
let b = 2;
```

#### Do

```jsx
let a = 1, b = 2;
```

#### Fixed

```jsx
let a = 1, b = 2;
```


### TypeScript

#### Don't

```ts
let a = 1;
let b = 2;
```

#### Do

```ts
let a = 1, b = 2;
```

#### Fixed

```ts
let a = 1, b = 2;
```


### TSX

#### Don't

```tsx
let a = 1;
let b = 2;
```

#### Do

```tsx
let a = 1, b = 2;
```

#### Fixed

```tsx
let a = 1, b = 2;
```
