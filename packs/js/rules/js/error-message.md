---
id: js/error-message
title: "Pass a message when creating an error"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the message is human-written text"
stability: preview
ported_from: [eslint-plugin-unicorn/error-message]
since: 0.5.0
---

A built-in error is created with a non-empty message.

**Why:** An error without a message says nothing in a log or a stack trace.

```codelaw

match:
  any:
    - pattern: new $E()
    - pattern: new $E("")
constraints:
  E: { regex: '^(Error|TypeError|RangeError|SyntaxError|ReferenceError|EvalError|URIError|AggregateError)$' }
message: "Give the `$E` a message"
```

## How to fix

1. Write a message that says what went wrong and, if known, what the caller can do.

## Examples

### JavaScript

#### Don't

```js
throw new Error();
```

#### Do

```js
throw new Error("config file is missing");
```

### JSX

#### Don't

```jsx
throw new Error();
```

#### Do

```jsx
throw new Error("config file is missing");
```

### TypeScript

#### Don't

```ts
throw new Error();
```

#### Do

```ts
throw new Error("config file is missing");
```

### TSX

#### Don't

```tsx
throw new Error();
```

#### Do

```tsx
throw new Error("config file is missing");
```
