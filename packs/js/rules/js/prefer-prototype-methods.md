---
id: js/prefer-prototype-methods
title: "Array.prototype.x rather than [].x"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-prototype-methods]
since: 0.5.0
---

A method borrowed from a literal (`[].slice`, `{}.hasOwnProperty`) is taken from the prototype. This port covers `[]`.

**Why:** The literal allocates an object just to reach its prototype.

```codelaw
match:
  pattern: "[].$M.call($$$A)"
message: "Borrow the method from `Array.prototype`"
fix: "Array.prototype.$M.call($$$A)"
```

## Examples

### JavaScript

#### Don't

```js
const args = [].slice.call(arguments);
```

#### Do

```js
const args = Array.prototype.slice.call(arguments);
```

### JSX

#### Don't

```jsx
const args = [].slice.call(arguments);
```

#### Do

```jsx
const args = Array.prototype.slice.call(arguments);
```

### TypeScript

#### Don't

```ts
const args = [].slice.call(arguments);
```

#### Do

```ts
const args = Array.prototype.slice.call(arguments);
```

### TSX

#### Don't

```tsx
const args = [].slice.call(arguments);
```

#### Do

```tsx
const args = Array.prototype.slice.call(arguments);
```
