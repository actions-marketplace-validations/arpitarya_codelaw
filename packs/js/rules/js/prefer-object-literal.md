---
id: js/prefer-object-literal
title: "Properties in the object literal, not assigned after"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a later assignment may read another property or the object itself; the author moves it"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-object-literal]
since: 0.5.0
---

An object declared empty (`const o = {}`) and then given a property in the next statement is written with that property in the literal.

**Why:** The literal shows the object's shape in one place, where it is made.

```codelaw
match:
  kind: lexical_declaration
  pattern: "const $O = {};"
  precedes: { pattern: "$O.$P = $V;" }
message: "Write the properties into the object literal"
```

## How to fix

1. Move the assignments into the literal as `key: value` pairs.

## Examples

### JavaScript

#### Don't

```js
const user = {};
user.name = name;
user.role = "admin";
```

#### Do

```js
const user = { name, role: "admin" };
```


### JSX

#### Don't

```jsx
const user = {};
user.name = name;
user.role = "admin";
```

#### Do

```jsx
const user = { name, role: "admin" };
```


### TypeScript

#### Don't

```ts
const user = {};
user.name = name;
user.role = "admin";
```

#### Do

```ts
const user = { name, role: "admin" };
```


### TSX

#### Don't

```tsx
const user = {};
user.name = name;
user.role = "admin";
```

#### Do

```tsx
const user = { name, role: "admin" };
```
