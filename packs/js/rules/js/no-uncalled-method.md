---
id: js/no-uncalled-method
title: "No array or string method read without being called"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the author meant to call the method or to read something else is unknown"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-uncalled-method]
since: 0.5.0
---

A known array method (`sort`, `pop`, `map`, …) on a value known to be an array, or a string method (`trim`, `toLowerCase`, …) on a value known to be a string, is reported when it is read but not called: `if (list.pop)`, `const s = name.trim;`. A value is known by its literal (`[…]`, `"…"`, a template), `Array.from()`/`Array.of()`, the name `array` or `string`, or a `const` bound to one. Calls, `.call()`/`.apply()`/`.bind()`, `Reflect.apply()`, `typeof`, assignment targets and `Array.prototype.x` are left alone.

**Why:** A method read without `()` is a function, always truthy and never the value the code wanted.

```codelaw
native: js/no-uncalled-method
```

## How to fix

1. Add the missing `()`.

## Examples

### JavaScript

#### Don't

```js
const items = [3, 1, 2];
const sorted = items.sort;
```

#### Do

```js
const items = [3, 1, 2];
const sorted = items.sort();
```


### JSX

#### Don't

```jsx
const items = [3, 1, 2];
const sorted = items.sort;
```

#### Do

```jsx
const items = [3, 1, 2];
const sorted = items.sort();
```


### TypeScript

#### Don't

```ts
const items = [3, 1, 2];
const sorted = items.sort;
```

#### Do

```ts
const items = [3, 1, 2];
const sorted = items.sort();
```


### TSX

#### Don't

```tsx
const items = [3, 1, 2];
const sorted = items.sort;
```

#### Do

```tsx
const items = [3, 1, 2];
const sorted = items.sort();
```
