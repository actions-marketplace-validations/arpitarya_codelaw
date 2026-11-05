---
id: js/no-delete-var
title: "No delete on a variable"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`delete x` does nothing on a binding; resetting it to `undefined` is the closest live meaning, which the reviewer confirms (the author may have meant another value or to drop the line)"
stability: preview
ported_from: [eslint/no-delete-var]
since: 0.1.0
---

`delete` is applied to object properties, never to a variable.

**Why:** Deleting a variable does nothing in sloppy mode and is a syntax error in strict mode.

```codelaw
match:
  pattern: delete $X
constraints:
  X: { kind: identifier }
message: "`delete` works on properties, not on the variable `$X`"
fix: "$X = undefined"
```

## How to fix

1. To clear the value, assign `undefined` or a fresh value.
2. To remove a property, `delete obj.key`.

## Examples

### JavaScript

#### Don't

```js
let cache = {};
delete cache;
```

#### Fixed

```js
let cache = {};
cache = undefined;
```

#### Do

```js
let cache = {};
cache = {};
```

### JSX

#### Don't

```jsx
let cache = {};
delete cache;
```

#### Fixed

```jsx
let cache = {};
cache = undefined;
```

#### Do

```jsx
let cache = {};
cache = {};
```

### TypeScript

#### Don't

```ts
let cache = {};
delete cache;
```

#### Fixed

```ts
let cache = {};
cache = undefined;
```

#### Do

```ts
let cache = {};
cache = {};
```

### TSX

#### Don't

```tsx
let cache = {};
delete cache;
```

#### Fixed

```tsx
let cache = {};
cache = undefined;
```

#### Do

```tsx
let cache = {};
cache = {};
```
