---
id: js/no-void-type-return
title: "A void function returns no value"
languages: [typescript, tsx]
fix: manual
fix_reason: "either the return type or the returned value is wrong; the author decides which"
fix_pending: "suggest-choices"
stability: preview
ported_from: [biome/correctness/noVoidTypeReturn]
since: 0.5.0
---

A function whose return type is `void` does not `return` a value.

**Why:** The caller has been told nothing comes back, so the value is silently thrown away.

```codelaw
match:
  kind: return_statement
  regex: '^return\s*[^\s;]'
  inside:
    stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}] }
    any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}]
    has: { field: return_type, regex: ':\s*void$' }
message: "This function's return type is `void`; it should not return a value"
```

## How to fix

1. Remove the value from the `return`, or change the return type.

## Examples

### TypeScript

#### Don't

```ts
function log(msg: string): void {
  return console.log(msg);
}
```

#### Do

```ts
function log(msg: string): void {
  console.log(msg);
}
```

### TSX

#### Don't

```tsx
function log(msg: string): void {
  return console.log(msg);
}
```

#### Do

```tsx
function log(msg: string): void {
  console.log(msg);
}
```
