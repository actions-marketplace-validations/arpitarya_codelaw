---
id: js/consistent-this
title: "An alias of this is named that"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the alias is renamed through the file; the reviewer checks no other `that` is in scope"
stability: preview
ported_from: [eslint/consistent-this]
since: 0.5.0
---

A variable that holds `this` is named `that`, ESLint's default, rather than `self`, `me` or `_this`.

**Why:** One name for the alias lets a reader find every place that captured the outer `this`.

```codelaw
match:
  kind: variable_declarator
  has: { field: value, kind: this }
  all:
    - has: { field: name, kind: identifier, pattern: $A, not: { regex: '^that$' } }
message: "Name the alias of `this` `that`"
fix:
  - rename-symbol-local: { target: $A, to: that }
```

## Examples

### JavaScript

#### Don't

```js
function Timer() {
  const self = this;
  setInterval(function () { self.tick(); }, 1000);
}
```

#### Do

```js
function Timer() {
  const that = this;
  setInterval(function () { that.tick(); }, 1000);
}
```


### JSX

#### Don't

```jsx
function Timer() {
  const self = this;
  setInterval(function () { self.tick(); }, 1000);
}
```

#### Do

```jsx
function Timer() {
  const that = this;
  setInterval(function () { that.tick(); }, 1000);
}
```


### TypeScript

#### Don't

```ts
function Timer() {
  const self = this;
  setInterval(function () { self.tick(); }, 1000);
}
```

#### Do

```ts
function Timer() {
  const that = this;
  setInterval(function () { that.tick(); }, 1000);
}
```


### TSX

#### Don't

```tsx
function Timer() {
  const self = this;
  setInterval(function () { self.tick(); }, 1000);
}
```

#### Do

```tsx
function Timer() {
  const that = this;
  setInterval(function () { that.tick(); }, 1000);
}
```
