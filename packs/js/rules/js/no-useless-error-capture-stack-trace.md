---
id: js/no-useless-error-capture-stack-trace
title: "No captureStackTrace in an Error subclass"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the call trims the constructor frame from the trace, which the trace now keeps"
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-error-capture-stack-trace]
since: 0.5.0
---

A class that extends `Error` does not call `Error.captureStackTrace(this, …)`.

**Why:** `super()` already captures the stack.

```codelaw

match:
  kind: expression_statement
  pattern: Error.captureStackTrace(this, $$$A);
  inside: { stopBy: end, kind: class_body, inside: { has: { kind: class_heritage, regex: 'Error\b' } } }
message: "An `Error` subclass already records its stack; this call is not needed"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
class TimeoutError extends Error {
  constructor(m) {
    super(m);
    Error.captureStackTrace(this, TimeoutError);
  }
}
```

#### Do

```js
class TimeoutError extends Error {
  constructor(m) {
    super(m);
  }
}
```

### JSX

#### Don't

```jsx
class TimeoutError extends Error {
  constructor(m) {
    super(m);
    Error.captureStackTrace(this, TimeoutError);
  }
}
```

#### Do

```jsx
class TimeoutError extends Error {
  constructor(m) {
    super(m);
  }
}
```

### TypeScript

#### Don't

```ts
class TimeoutError extends Error {
  constructor(m) {
    super(m);
    Error.captureStackTrace(this, TimeoutError);
  }
}
```

#### Do

```ts
class TimeoutError extends Error {
  constructor(m) {
    super(m);
  }
}
```

### TSX

#### Don't

```tsx
class TimeoutError extends Error {
  constructor(m) {
    super(m);
    Error.captureStackTrace(this, TimeoutError);
  }
}
```

#### Do

```tsx
class TimeoutError extends Error {
  constructor(m) {
    super(m);
  }
}
```
