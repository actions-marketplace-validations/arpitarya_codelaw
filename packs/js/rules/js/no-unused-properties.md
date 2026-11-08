---
id: js/no-unused-properties
title: "No property of a local object literal that nothing reads"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the property's value may have a side effect, so removing it is a change the author should accept"
stability: preview
ported_from: [eslint-plugin-unicorn/no-unused-properties]
since: 0.5.0
---

A variable initialised once with an object literal and never reassigned, whose uses are all reads of named properties (`o.a`, `o["a"]`) or destructurings of named keys, has every other property reported — nested objects read only through `o.p.q` are checked the same way. Any other use (passing the object, spreading it, a computed or dynamic access, a call of a property, a write to a property) counts as using everything. An exported object, or one with a method, getter, setter or function value, is not checked. The suggestion removes the property.

**Why:** A property nothing reads is dead data that still looks meaningful.

```codelaw
native: js/no-unused-properties
```

## Examples

### JavaScript

#### Don't

```js
function size() {
  const box = { width: 1, height: 2, depth: 3 };
  return box.width * box.height;
}
```

#### Do

```js
function size() {
  const box = { width: 1, height: 2 };
  return box.width * box.height;
}
```

#### Fixed

```js
function size() {
  const box = { width: 1, height: 2 };
  return box.width * box.height;
}
```


### JSX

#### Don't

```jsx
function size() {
  const box = { width: 1, height: 2, depth: 3 };
  return box.width * box.height;
}
```

#### Do

```jsx
function size() {
  const box = { width: 1, height: 2 };
  return box.width * box.height;
}
```

#### Fixed

```jsx
function size() {
  const box = { width: 1, height: 2 };
  return box.width * box.height;
}
```


### TypeScript

#### Don't

```ts
function size() {
  const box = { width: 1, height: 2, depth: 3 };
  return box.width * box.height;
}
```

#### Do

```ts
function size() {
  const box = { width: 1, height: 2 };
  return box.width * box.height;
}
```

#### Fixed

```ts
function size() {
  const box = { width: 1, height: 2 };
  return box.width * box.height;
}
```


### TSX

#### Don't

```tsx
function size() {
  const box = { width: 1, height: 2, depth: 3 };
  return box.width * box.height;
}
```

#### Do

```tsx
function size() {
  const box = { width: 1, height: 2 };
  return box.width * box.height;
}
```

#### Fixed

```tsx
function size() {
  const box = { width: 1, height: 2 };
  return box.width * box.height;
}
```
