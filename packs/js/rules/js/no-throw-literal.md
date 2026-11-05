---
id: js/no-throw-literal
title: "Throw Error objects"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the error class and message are the author's"
fix_pending: "suggest"
stability: preview
ported_from: [eslint/no-throw-literal]
since: 0.5.0
---

`throw` throws an `Error` (or a subclass), not a string, number or other literal.

**Why:** A thrown literal has no stack trace, and `catch` code expecting `err.message` breaks.

```codelaw
match:
  kind: throw_statement
  has:
    any:
      - kind: string
      - kind: number
      - kind: template_string
      - kind: 'null'
      - kind: 'true'
      - kind: 'false'
      - { kind: identifier, regex: '^undefined$' }
      - kind: binary_expression
        has: { any: [{kind: string}, {kind: template_string}] }
message: "Throw an `Error`, not a literal"
```

## How to fix

1. Throw `new Error(message)` (or a subclass).

## Examples

### JavaScript

#### Don't

```js
throw 'not found';
```

#### Do

```js
throw new Error('not found');
```

### JSX

#### Don't

```jsx
throw 'not found';
```

#### Do

```jsx
throw new Error('not found');
```

### TypeScript

#### Don't

```ts
throw 'not found';
```

#### Do

```ts
throw new Error('not found');
```

### TSX

#### Don't

```tsx
throw 'not found';
```

#### Do

```tsx
throw new Error('not found');
```
