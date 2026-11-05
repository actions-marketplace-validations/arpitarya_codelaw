---
id: js/no-throw-literal
title: "Throw Error objects"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the literal is wrapped in a new Error, which catchers testing `typeof` or the value will see as a different object; the reviewer picks the error class and message"
stability: preview
ported_from: [eslint/no-throw-literal]
since: 0.5.0
---

`throw` throws an `Error` (or a subclass), not a string, number or other literal.

**Why:** A thrown literal has no stack trace, and `catch` code expecting `err.message` breaks.

```codelaw
match:
  kind: throw_statement
  any:
    - has:
        pattern: $S
        any:
          - kind: string
          - kind: template_string
          - kind: binary_expression
            has: { any: [{kind: string}, {kind: template_string}] }
    - has:
        pattern: $O
        any:
          - kind: number
          - kind: 'null'
          - kind: 'true'
          - kind: 'false'
          - { kind: identifier, regex: '^undefined$' }
message: "Throw an `Error`, not a literal"
fix:
  - replace: { target: $S, with: "new Error($S)" }
  - replace: { target: $O, with: "new Error(String($O))" }
```

## How to fix

1. Throw `new Error(message)` (or a subclass).

## Examples

### JavaScript

#### Don't

```js
throw 'not found';
```

#### Fixed

```js
throw new Error('not found');
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

#### Fixed

```jsx
throw new Error('not found');
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

#### Fixed

```ts
throw new Error('not found');
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

#### Fixed

```tsx
throw new Error('not found');
```

#### Do

```tsx
throw new Error('not found');
```
