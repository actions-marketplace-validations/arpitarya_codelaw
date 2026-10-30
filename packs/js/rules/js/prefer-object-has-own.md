---
id: js/prefer-object-has-own
title: "Object.hasOwn rather than hasOwnProperty.call"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/prefer-object-has-own]
since: 0.5.0
---

`Object.hasOwn(obj, key)` is used instead of `Object.prototype.hasOwnProperty.call(obj, key)`.

**Why:** It says the same in one call and cannot be shadowed by the object's own `hasOwnProperty`.

```codelaw
match:
  any:
    - pattern: Object.prototype.hasOwnProperty.call($O, $K)
    - pattern: "{}.hasOwnProperty.call($O, $K)"
message: "Use `Object.hasOwn`"
fix: "Object.hasOwn($O, $K)"
```

## Examples

### JavaScript

#### Don't

```js
if (Object.prototype.hasOwnProperty.call(o, k)) use(o[k]);
```

#### Do

```js
if (Object.hasOwn(o, k)) use(o[k]);
```

#### Fixed

```js
if (Object.hasOwn(o, k)) use(o[k]);
```

### JSX

#### Don't

```jsx
if (Object.prototype.hasOwnProperty.call(o, k)) use(o[k]);
```

#### Do

```jsx
if (Object.hasOwn(o, k)) use(o[k]);
```

#### Fixed

```jsx
if (Object.hasOwn(o, k)) use(o[k]);
```

### TypeScript

#### Don't

```ts
if (Object.prototype.hasOwnProperty.call(o, k)) use(o[k]);
```

#### Do

```ts
if (Object.hasOwn(o, k)) use(o[k]);
```

#### Fixed

```ts
if (Object.hasOwn(o, k)) use(o[k]);
```

### TSX

#### Don't

```tsx
if (Object.prototype.hasOwnProperty.call(o, k)) use(o[k]);
```

#### Do

```tsx
if (Object.hasOwn(o, k)) use(o[k]);
```

#### Fixed

```tsx
if (Object.hasOwn(o, k)) use(o[k]);
```
