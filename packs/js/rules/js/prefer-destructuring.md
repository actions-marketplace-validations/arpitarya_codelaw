---
id: js/prefer-destructuring
title: "Destructure a property into a variable of its name"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a getter on the object runs the same, but a TypeScript annotation on the variable is dropped"
stability: preview
ported_from: [eslint/prefer-destructuring]
since: 0.5.0
---

A `const` declaring one variable from the property of the same name is destructured (`const { name } = user`). Array indexes are left alone in this port.

**Why:** Destructuring names the property once.

```codelaw

match:
  kind: lexical_declaration
  any:
    - pattern: "const $X = $O.$X;"
message: "Destructure the property: `{ $X } = $O`"
fix: "const { $X } = $O;"
```

## Examples

### JavaScript

#### Don't

```js
const name = user.name;
```

#### Do

```js
const { name } = user;
```

### JSX

#### Don't

```jsx
const name = user.name;
```

#### Do

```jsx
const { name } = user;
```

### TypeScript

#### Don't

```ts
const name = user.name;
```

#### Do

```ts
const { name } = user;
```

### TSX

#### Don't

```tsx
const name = user.name;
```

#### Do

```tsx
const { name } = user;
```
