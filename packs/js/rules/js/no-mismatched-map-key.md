---
id: js/no-mismatched-map-key
title: "No `map.has(a)` guarding `map.get(b)`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which of the two keys is the intended one is the author's knowledge"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-mismatched-map-key]
since: 0.5.0
---

When `m.has(k)` (or `!m.has(k)`) guards an `if`, a `while`, a `?:` or the right side of `&&`, `||` or `??`, a `m.get(k2)` or `m.set(k2, …)` on the same receiver in the guarded code uses the same key. Two different literals, or two different names or member chains, are reported at the second key; a key with a call in it is not compared. The guarded branches of an `if` are read when they are one statement; a nested function, a new guard on the same map, or an assignment to it stops the search.

**Why:** A guard on one key and a read of another is a copy-paste slip: the read may return `undefined` the guard was meant to rule out.

```codelaw
native: js/no-mismatched-map-key
```

## How to fix

1. Use the key the guard checked, or guard the key you read.

## Examples

### JavaScript

#### Don't

```js
const label = names.has(id) ? names.get(key) : "?";
```

#### Do

```js
const label = names.has(id) ? names.get(id) : "?";
```


### JSX

#### Don't

```jsx
const label = names.has(id) ? names.get(key) : "?";
```

#### Do

```jsx
const label = names.has(id) ? names.get(id) : "?";
```


### TypeScript

#### Don't

```ts
const label = names.has(id) ? names.get(key) : "?";
```

#### Do

```ts
const label = names.has(id) ? names.get(id) : "?";
```


### TSX

#### Don't

```tsx
const label = names.has(id) ? names.get(key) : "?";
```

#### Do

```tsx
const label = names.has(id) ? names.get(id) : "?";
```
