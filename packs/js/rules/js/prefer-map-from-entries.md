---
id: js/prefer-map-from-entries
title: "A `Map` for an object built from entries and used as a map"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the declaration and every use are rewritten together, and only when every use is one of the map-like forms and no comment is inside; otherwise the declaration is reported without a fix"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-map-from-entries]
since: 0.5.0
---

`const x = Object.fromEntries(entries)` is reported when `entries` is `Object.entries(…)` or a non-empty array literal of pairs whose keys are string literals (not array indices), and every use of `x` is `x.key` or `x['key']` (a read), `x.key = value` or `delete x.key` as a statement, `Object.hasOwn(x, 'key')` or `Object.keys/values/entries(x)`. One other use — passing `x` along, spreading it, calling `x.key()`, `x.key++`, a computed key — leaves the variable alone: a partial conversion would break it. The fix makes it `new Map(entries)` and the uses `x.get('key')`, `x.set('key', value)`, `x.delete('key')`, `x.has('key')` and `[...x.keys()]`. An exported or annotated variable, a shadowed `Map`, and keys that are inherited object members or array indices are not converted.

**Why:** A `Map` is built for lookups by key, keeps the keys' order and has no inherited keys; an object used only like one hides that behind property syntax.

```codelaw
native: js/prefer-map-from-entries
```

## Examples

### JavaScript

#### Don't

```js
const object = Object.fromEntries(Object.entries(source));
if (Object.hasOwn(object, 'foo')) {
  console.log(object.foo);
}
```

#### Do

```js
const object = new Map(Object.entries(source));
if (object.has('foo')) {
  console.log(object.get('foo'));
}
```

#### Fixed

```js
const object = new Map(Object.entries(source));
if (object.has('foo')) {
  console.log(object.get('foo'));
}
```


### JSX

#### Don't

```jsx
const object = Object.fromEntries(Object.entries(source));
if (Object.hasOwn(object, 'foo')) {
  console.log(object.foo);
}
```

#### Do

```jsx
const object = new Map(Object.entries(source));
if (object.has('foo')) {
  console.log(object.get('foo'));
}
```

#### Fixed

```jsx
const object = new Map(Object.entries(source));
if (object.has('foo')) {
  console.log(object.get('foo'));
}
```


### TypeScript

#### Don't

```ts
const object = Object.fromEntries(Object.entries(source));
if (Object.hasOwn(object, 'foo')) {
  console.log(object.foo);
}
```

#### Do

```ts
const object = new Map(Object.entries(source));
if (object.has('foo')) {
  console.log(object.get('foo'));
}
```

#### Fixed

```ts
const object = new Map(Object.entries(source));
if (object.has('foo')) {
  console.log(object.get('foo'));
}
```


### TSX

#### Don't

```tsx
const object = Object.fromEntries(Object.entries(source));
if (Object.hasOwn(object, 'foo')) {
  console.log(object.foo);
}
```

#### Do

```tsx
const object = new Map(Object.entries(source));
if (object.has('foo')) {
  console.log(object.get('foo'));
}
```

#### Fixed

```tsx
const object = new Map(Object.entries(source));
if (object.has('foo')) {
  console.log(object.get('foo'));
}
```
