---
id: js/no-shadow
title: "No declaration that shadows an outer one"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new name is the author's"
fix_exception: value
stability: preview
ported_from: [eslint/no-shadow]
since: 0.5.0
---

A name declared in an inner scope is not one already declared in a scope around it.

**Why:** Inside the inner scope the outer value can't be reached, and a reader may think it is being used.

```codelaw
match:
  kind: identifier
where:
  $$MATCH: [declaration, shadowed]
message: "`$$MATCH` shadows an outer declaration"
```

## How to fix

1. Rename the inner declaration, or reuse the outer one if that was meant.

## Examples

### JavaScript

#### Don't

```js
const user = load();
function greet() {
  const user = guest();
  return user.name;
}
```

#### Do

```js
const user = load();
function greet() {
  const visitor = guest();
  return visitor.name;
}
```


### JSX

#### Don't

```jsx
const user = load();
function greet() {
  const user = guest();
  return user.name;
}
```

#### Do

```jsx
const user = load();
function greet() {
  const visitor = guest();
  return visitor.name;
}
```


### TypeScript

#### Don't

```ts
const user = load();
function greet() {
  const user = guest();
  return user.name;
}
```

#### Do

```ts
const user = load();
function greet() {
  const visitor = guest();
  return visitor.name;
}
```


### TSX

#### Don't

```tsx
const user = load();
function greet() {
  const user = guest();
  return user.name;
}
```

#### Do

```tsx
const user = load();
function greet() {
  const visitor = guest();
  return visitor.name;
}
```
