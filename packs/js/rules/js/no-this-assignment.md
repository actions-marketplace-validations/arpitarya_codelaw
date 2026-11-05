---
id: js/no-this-assignment
title: "No this alias"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the functions reading the alias become arrow functions"
stability: preview
ported_from: [eslint-plugin-unicorn/no-this-assignment, eslint/consistent-this]
since: 0.5.0
---

`this` is not assigned to a variable (`const self = this`).

**Why:** Arrow functions keep `this`, so an alias is a leftover that hides which object is meant.

```codelaw
match:
  kind: variable_declarator
  has:
    field: value
    kind: this
  all:
  - has:
      field: name
      kind: identifier
      pattern: $V
message: '`this` is aliased to `$V`; use an arrow function instead'
```

## How to fix

1. Turn the nested functions that read the alias into arrow functions and read `this` directly.

## Examples

### JavaScript

#### Don't

```js
const self = this;
items.forEach(function (x) {
  self.add(x);
});
```

#### Do

```js
items.forEach((x) => {
  this.add(x);
});
```

### JSX

#### Don't

```jsx
const self = this;
items.forEach(function (x) {
  self.add(x);
});
```

#### Do

```jsx
items.forEach((x) => {
  this.add(x);
});
```

### TypeScript

#### Don't

```ts
const self = this;
items.forEach(function (x) {
  self.add(x);
});
```

#### Do

```ts
items.forEach((x) => {
  this.add(x);
});
```

### TSX

#### Don't

```tsx
const self = this;
items.forEach(function (x) {
  self.add(x);
});
```

#### Do

```tsx
items.forEach((x) => {
  this.add(x);
});
```
