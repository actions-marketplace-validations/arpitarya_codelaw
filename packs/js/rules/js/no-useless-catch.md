---
id: js/no-useless-catch
title: "No catch that only rethrows"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the try block stays as a block, keeping its declarations scoped, and the same error rethrows; with a `finally` only the catch clause goes"
stability: preview
ported_from: [eslint/no-useless-catch]
since: 0.1.0
---

A `catch` clause does more than rethrow the error it caught.

**Why:** `catch (e) { throw e; }` does nothing but add noise and a level of nesting.

```codelaw
match:
  kind: catch_clause
  inside:
    kind: try_statement
    any:
      - pattern: $TRY
        has: { field: body, pattern: $BODY }
        not: { has: { field: finalizer, kind: finally_clause } }
      - all:
          - has: { field: handler, pattern: $CATCH }
          - has: { field: finalizer, kind: finally_clause }
  all:
    - has: { field: parameter, pattern: $E }
    - has:
        field: body
        regex: ^\{\s*throw\s+[A-Za-z_$][A-Za-z0-9_$]*\s*;?\s*\}$
        has: { kind: throw_statement, has: { pattern: $E }, stopBy: end }
message: "This catch only rethrows `$E`; remove it"
fix:
  - replace: { target: $TRY, with: "$BODY" }
  - remove: $CATCH
```

## How to fix

1. If there is no `finally`, replace the `try` statement with the statements of its block (the fix keeps them in a block).
2. If there is a `finally`, remove only the `catch` clause.

## Examples

### JavaScript

#### Don't

```js
try {
  load();
} catch (e) {
  throw e;
}
```

#### Fixed

```js
{
  load();
}
```

#### Do

```js
load();
```

### JSX

#### Don't

```jsx
try {
  load();
} catch (e) {
  throw e;
}
```

#### Fixed

```jsx
{
  load();
}
```

#### Do

```jsx
load();
```

### TypeScript

#### Don't

```ts
try {
  load();
} catch (e) {
  throw e;
}
```

#### Fixed

```ts
{
  load();
}
```

#### Do

```ts
load();
```

### TSX

#### Don't

```tsx
try {
  load();
} catch (e) {
  throw e;
}
```

#### Fixed

```tsx
{
  load();
}
```

#### Do

```tsx
load();
```
