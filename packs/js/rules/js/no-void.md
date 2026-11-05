---
id: js/no-void
title: "No void operator"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "`void <literal>` is `undefined` exactly; `void e;` as a statement stays a suggestion, as the `void` may mark a promise left unhandled on purpose"
stability: preview
ported_from: [eslint/no-void]
since: 0.5.0
---

The `void` operator is not used.

**Why:** `void 0` is `undefined` written obscurely; `void promise` hides an unhandled promise.

```codelaw
match:
  kind: unary_expression
  regex: '^void\b'
  any:
    - pattern: $LIT
      has: { field: argument, any: [{ kind: number }, { kind: string }, { kind: 'null' }, { kind: 'true' }, { kind: 'false' }] }
    - pattern: $STMT
      has: { field: argument, pattern: $E }
      inside: { kind: expression_statement }
    - kind: unary_expression
message: "Do not use `void`"
auto_when:
  has: { field: argument, any: [{ kind: number }, { kind: string }, { kind: 'null' }, { kind: 'true' }, { kind: 'false' }] }
fix:
  - replace: { target: $LIT, with: "undefined" }
  - replace: { target: $STMT, with: "$E" }
```

## How to fix

1. Write `undefined`, or handle the promise with `await` or `.catch()`; the fix writes `undefined` for `void` of a literal and offers the bare expression for a `void e;` statement.

## Examples

### JavaScript

#### Don't

```js
const nothing = void 0;
```

#### Fixed

```js
const nothing = undefined;
```

#### Do

```js
const nothing = undefined;
```

### JSX

#### Don't

```jsx
const nothing = void 0;
```

#### Fixed

```jsx
const nothing = undefined;
```

#### Do

```jsx
const nothing = undefined;
```

### TypeScript

#### Don't

```ts
const nothing = void 0;
```

#### Fixed

```ts
const nothing = undefined;
```

#### Do

```ts
const nothing = undefined;
```

### TSX

#### Don't

```tsx
const nothing = void 0;
```

#### Fixed

```tsx
const nothing = undefined;
```

#### Do

```tsx
const nothing = undefined;
```
