---
id: js/custom-error-definition
title: "Custom errors set name and extend Error"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the constructor must call `super`, and its body changes"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/custom-error-definition]
since: 0.5.0
---

A class whose name ends in `Error` extends `Error` (or another error class). Upstream also checks the `name` property and the constructor; this port checks the base class.

**Why:** Without `extends Error` the object has no stack trace and fails `instanceof Error`.

```codelaw

match:
  kind: class_declaration
  has: { field: name, regex: 'Error$' }
  not: { has: { kind: class_heritage, regex: 'Error\b' } }
message: "A class named `…Error` should extend `Error`"
```

## How to fix

1. Add `extends Error`, call `super(message)` first in the constructor, and set `this.name` to the class name.

## Examples

### JavaScript

#### Don't

```js
class TimeoutError {
  constructor(message) {
    this.message = message;
  }
}
```

#### Do

```js
class TimeoutError extends Error {
  constructor(message) {
    super(message);
  }
}
```

### JSX

#### Don't

```jsx
class TimeoutError {
  constructor(message) {
    this.message = message;
  }
}
```

#### Do

```jsx
class TimeoutError extends Error {
  constructor(message) {
    super(message);
  }
}
```

### TypeScript

#### Don't

```ts
class TimeoutError {
  constructor(message) {
    this.message = message;
  }
}
```

#### Do

```ts
class TimeoutError extends Error {
  constructor(message) {
    super(message);
  }
}
```

### TSX

#### Don't

```tsx
class TimeoutError {
  constructor(message) {
    this.message = message;
  }
}
```

#### Do

```tsx
class TimeoutError extends Error {
  constructor(message) {
    super(message);
  }
}
```
