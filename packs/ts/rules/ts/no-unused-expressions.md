---
id: ts/no-unused-expressions
title: "No expression statements that do nothing"
languages: [typescript, tsx]
fix: suggest
fix_reason: "the statement does nothing, so the choices are what it was meant to do: delete it (the default), call it, or make a comparison an assignment; the reviewer picks, since only the author knows the intent"
stability: preview
ported_from: [typescript-eslint/no-unused-expressions, eslint/no-unused-expressions]
since: 0.1.0
---

Every expression statement does something: a call, an assignment, an update, `await`, `yield`,
`delete` or `void`. Directives such as `"use strict"` are allowed.

**Why:** An expression whose value is thrown away is usually a forgotten call or assignment.

```codelaw
native: ts/no-unused-expressions
```

## How to fix

1. Add the call or assignment that was meant, or remove the statement.

## Examples

### TypeScript

#### Don't

```ts
function close(conn: Connection) {
  conn.close;
}
```

#### Fixed (delete)

```ts
function close(conn: Connection) {
}
```

#### Fixed (call)

```ts
function close(conn: Connection) {
  conn.close();
}
```

#### Do

```ts
function close(conn: Connection) {
  conn.close();
}
```

### TSX

#### Don't

```tsx
function close(conn: Connection) {
  conn.close;
}
```

#### Fixed (delete)

```tsx
function close(conn: Connection) {
}
```

#### Fixed (call)

```tsx
function close(conn: Connection) {
  conn.close();
}
```

#### Do

```tsx
function close(conn: Connection) {
  conn.close();
}
```
