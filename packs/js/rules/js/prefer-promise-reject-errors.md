---
id: js/prefer-promise-reject-errors
title: "Reject with an Error"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the error class and message are the author's"
stability: preview
ported_from: [eslint/prefer-promise-reject-errors]
since: 0.5.0
---

`Promise.reject` and an executor's `reject` are called with an `Error`, not a literal.

**Why:** A rejection without an `Error` has no stack trace, and handlers reading `err.message` break.

```codelaw
match:
  any:
    - pattern: Promise.reject($L)
    - pattern: Promise.reject()
    - pattern: $R($L)
      inside: { stopBy: end, kind: new_expression, has: { field: constructor, regex: '^Promise$' } }
constraints:
  R: { regex: '^reject$' }
  L: { any: [{kind: string}, {kind: number}, {kind: template_string}, {kind: 'null'}, {kind: 'true'}, {kind: 'false'}, {kind: object}, {kind: undefined}] }
message: "Reject with an `Error`"
```

## How to fix

1. Pass `new Error(message)`.

## Examples

### JavaScript

#### Don't

```js
return Promise.reject('timeout');
```

#### Do

```js
return Promise.reject(new Error('timeout'));
```

### JSX

#### Don't

```jsx
return Promise.reject('timeout');
```

#### Do

```jsx
return Promise.reject(new Error('timeout'));
```

### TypeScript

#### Don't

```ts
return Promise.reject('timeout');
```

#### Do

```ts
return Promise.reject(new Error('timeout'));
```

### TSX

#### Don't

```tsx
return Promise.reject('timeout');
```

#### Do

```tsx
return Promise.reject(new Error('timeout'));
```
