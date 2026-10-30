---
id: js/no-new
title: "No new for side effects"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the object is either needed (assign it) or the constructor should be a function"
stability: preview
ported_from: [eslint/no-new]
since: 0.5.0
---

An object made with `new` is used, not thrown away.

**Why:** A constructor called only for its side effects is a function in disguise.

```codelaw
match:
  kind: expression_statement
  has: { kind: new_expression }
message: "Do not use `new` for side effects; assign the object or call a function"
```

## How to fix

1. Assign the object, or turn the constructor's work into a function.

## Examples

### JavaScript

#### Don't

```js
new Server(port);
```

#### Do

```js
const server = new Server(port);
```

### JSX

#### Don't

```jsx
new Server(port);
```

#### Do

```jsx
const server = new Server(port);
```

### TypeScript

#### Don't

```ts
new Server(port);
```

#### Do

```ts
const server = new Server(port);
```

### TSX

#### Don't

```tsx
new Server(port);
```

#### Do

```tsx
const server = new Server(port);
```
