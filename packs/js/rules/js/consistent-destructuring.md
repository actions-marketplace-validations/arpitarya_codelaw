---
id: js/consistent-destructuring
title: "Use the name a property was destructured into"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the property is read again later in the code; whether it may have changed since is for the author to confirm"
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-destructuring]
since: 0.5.0
---

After `const { a } = obj` (or `const { a: b } = obj`, or from `this`), a later `obj.a` in the same scope is reported: the value already has a name. It is left alone when `obj` or `obj.a` may have changed in between — a write to either, a loop that writes `obj`, or any call between the two in the same function — when it is called, deleted or assigned to, when it is guarded by `'a' in obj`, or when the name is shadowed there. The suggestion replaces `obj.a` with the name.

**Why:** Reading the same value two ways makes the reader check whether they differ.

```codelaw
native: js/consistent-destructuring
```

## Examples

### JavaScript

#### Don't

```js
function show(user) {
  const { name } = user;
  return name + user.name;
}
```

#### Do

```js
function show(user) {
  const { name } = user;
  return name + name;
}
```

#### Fixed

```js
function show(user) {
  const { name } = user;
  return name + name;
}
```


### JSX

#### Don't

```jsx
function show(user) {
  const { name } = user;
  return name + user.name;
}
```

#### Do

```jsx
function show(user) {
  const { name } = user;
  return name + name;
}
```

#### Fixed

```jsx
function show(user) {
  const { name } = user;
  return name + name;
}
```


### TypeScript

#### Don't

```ts
function show(user) {
  const { name } = user;
  return name + user.name;
}
```

#### Do

```ts
function show(user) {
  const { name } = user;
  return name + name;
}
```

#### Fixed

```ts
function show(user) {
  const { name } = user;
  return name + name;
}
```


### TSX

#### Don't

```tsx
function show(user) {
  const { name } = user;
  return name + user.name;
}
```

#### Do

```tsx
function show(user) {
  const { name } = user;
  return name + name;
}
```

#### Fixed

```tsx
function show(user) {
  const { name } = user;
  return name + name;
}
```
