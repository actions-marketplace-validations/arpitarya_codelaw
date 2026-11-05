---
id: js/no-unreachable-loop
title: "A loop that always exits on its first pass"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the rewrite (an `if`, or moving the exit into a condition) depends on intent"
fix_exception: value
stability: preview
ported_from: [eslint/no-unreachable-loop]
since: 0.5.0
---

A loop body does not end, at its top level, with an unconditional `return`, `throw` or `break` and contain no `continue`.

**Why:** A loop that runs at most once is a disguised `if`, and usually a bug.

```codelaw

match:
  any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}]
  has:
    field: body
    kind: statement_block
    has: { any: [{kind: return_statement}, {kind: throw_statement}, {kind: break_statement, not: { has: { kind: statement_identifier } }}] }
    not: { has: { stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}] }, kind: continue_statement } }
message: "This loop always ends during its first iteration"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
for (const user of users) {
  return user;
}
```

#### Do

```js
return users[0];
```

### JSX

#### Don't

```jsx
for (const user of users) {
  return user;
}
```

#### Do

```jsx
return users[0];
```

### TypeScript

#### Don't

```ts
for (const user of users) {
  return user;
}
```

#### Do

```ts
return users[0];
```

### TSX

#### Don't

```tsx
for (const user of users) {
  return user;
}
```

#### Do

```tsx
return users[0];
```
