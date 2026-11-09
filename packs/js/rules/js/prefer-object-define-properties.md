---
id: js/prefer-object-define-properties
title: "One `Object.defineProperties()` for consecutive `Object.defineProperty()` calls"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-object-define-properties]
since: 0.5.0
---

Two or more `Object.defineProperty(target, key, descriptor)` statements in a row on the same target are reported at the first and replaced by one `Object.defineProperties(target, { key: descriptor, … })`. The fix is withheld — the run is still reported — when a key is not a literal or `Symbol.*`, keys repeat, the new object would enumerate them in another order (an integer key after a string, a string after a symbol, integers out of order), a later key or descriptor could see or change the target (it calls something or mentions it), a descriptor spans lines, or a comment is inside.

**Why:** One call states the whole set of properties and repeats the target once.

```codelaw
native: js/prefer-object-define-properties
```

## Examples

### JavaScript

#### Don't

```js
Object.defineProperty(point, "x", { value: 1 });
Object.defineProperty(point, "y", { value: 2 });
```

#### Do

```js
Object.defineProperties(point, {
  x: { value: 1 },
  y: { value: 2 },
});
```

#### Fixed

```js
Object.defineProperties(point, {
  x: { value: 1 },
  y: { value: 2 },
});
```


### JSX

#### Don't

```jsx
Object.defineProperty(point, "x", { value: 1 });
Object.defineProperty(point, "y", { value: 2 });
```

#### Do

```jsx
Object.defineProperties(point, {
  x: { value: 1 },
  y: { value: 2 },
});
```

#### Fixed

```jsx
Object.defineProperties(point, {
  x: { value: 1 },
  y: { value: 2 },
});
```


### TypeScript

#### Don't

```ts
Object.defineProperty(point, "x", { value: 1 });
Object.defineProperty(point, "y", { value: 2 });
```

#### Do

```ts
Object.defineProperties(point, {
  x: { value: 1 },
  y: { value: 2 },
});
```

#### Fixed

```ts
Object.defineProperties(point, {
  x: { value: 1 },
  y: { value: 2 },
});
```


### TSX

#### Don't

```tsx
Object.defineProperty(point, "x", { value: 1 });
Object.defineProperty(point, "y", { value: 2 });
```

#### Do

```tsx
Object.defineProperties(point, {
  x: { value: 1 },
  y: { value: 2 },
});
```

#### Fixed

```tsx
Object.defineProperties(point, {
  x: { value: 1 },
  y: { value: 2 },
});
```
