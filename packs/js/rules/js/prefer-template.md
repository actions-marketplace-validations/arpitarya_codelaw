---
id: js/prefer-template
title: "Template literal rather than string concatenation"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a value joined with `+` is read through its `valueOf` and in a template through its `toString`, which differ for some objects; a join holding a comment, or a line continuation in a string, is rewritten by hand"
stability: preview
ported_from: [eslint/prefer-template]
since: 0.5.0
---

A string is not built by adding a string literal and another value with `+`.

**Why:** A template literal shows the shape of the result at a glance.

```codelaw
native: js/prefer-template
```

## How to fix

1. Write one template literal: each string part as text, each other operand as `${…}`.

## Examples

### JavaScript

#### Don't

```js
const greeting = "Hello, " + name;
```

#### Do

```js
const greeting = `Hello, ${name}`;
```

### JSX

#### Don't

```jsx
const greeting = "Hello, " + name;
```

#### Do

```jsx
const greeting = `Hello, ${name}`;
```

### TypeScript

#### Don't

```ts
const greeting = "Hello, " + name;
```

#### Do

```ts
const greeting = `Hello, ${name}`;
```

### TSX

#### Don't

```tsx
const greeting = "Hello, " + name;
```

#### Do

```tsx
const greeting = `Hello, ${name}`;
```
