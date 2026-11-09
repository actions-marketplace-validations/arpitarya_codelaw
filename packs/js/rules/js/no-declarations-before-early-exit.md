---
id: js/no-declarations-before-early-exit
title: "No declaration above an early exit that doesn't need it"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-declarations-before-early-exit]
since: 0.5.0
---

A `let` or `const` whose every use comes after the next guard — an `if` where exactly one branch always leaves (`return`, `throw`, `break`, `continue`, `process.exit()`) — is reported at its name: it can be declared after the guard. A value that awaits or yields, a React hook call, or a value that shares a name with the guard's test while either has side effects stays. When the guard is not the next statement, only a declaration with no value or a literal value is reported. The fix swaps the declaration and the guard when they are adjacent, the declaration has one declarator with a literal value or none, and no comment is near.

**Why:** Work done before an exit that doesn't need it is wasted, and the reader has to carry the name past the guard.

```codelaw
native: js/no-declarations-before-early-exit
```

## Examples

### JavaScript

#### Don't

```js
function label(user) {
  const prefix = "Dr. ";
  if (!user) return "";
  return prefix + user.name;
}
```

#### Do

```js
function label(user) {
  if (!user) return "";
  const prefix = "Dr. ";
  return prefix + user.name;
}
```

#### Fixed

```js
function label(user) {
  if (!user) return "";
  const prefix = "Dr. ";
  return prefix + user.name;
}
```


### JSX

#### Don't

```jsx
function label(user) {
  const prefix = "Dr. ";
  if (!user) return "";
  return prefix + user.name;
}
```

#### Do

```jsx
function label(user) {
  if (!user) return "";
  const prefix = "Dr. ";
  return prefix + user.name;
}
```

#### Fixed

```jsx
function label(user) {
  if (!user) return "";
  const prefix = "Dr. ";
  return prefix + user.name;
}
```


### TypeScript

#### Don't

```ts
function label(user) {
  const prefix = "Dr. ";
  if (!user) return "";
  return prefix + user.name;
}
```

#### Do

```ts
function label(user) {
  if (!user) return "";
  const prefix = "Dr. ";
  return prefix + user.name;
}
```

#### Fixed

```ts
function label(user) {
  if (!user) return "";
  const prefix = "Dr. ";
  return prefix + user.name;
}
```


### TSX

#### Don't

```tsx
function label(user) {
  const prefix = "Dr. ";
  if (!user) return "";
  return prefix + user.name;
}
```

#### Do

```tsx
function label(user) {
  if (!user) return "";
  const prefix = "Dr. ";
  return prefix + user.name;
}
```

#### Fixed

```tsx
function label(user) {
  if (!user) return "";
  const prefix = "Dr. ";
  return prefix + user.name;
}
```
