---
id: ts/explicit-module-boundary-types
title: "Exported functions declare their types"
languages: [typescript, tsx]
fix: manual
fix_reason: "the types are the author's to write"
fix_pending: "suggest; needs type-info (W-105)"
stability: preview
ported_from: [typescript-eslint/explicit-module-boundary-types]
since: 0.5.0
---

An exported function declares its return type.

**Why:** The exported signature is the module's contract; inferring it lets an internal change silently change the API.

```codelaw
match:
  kind: function_declaration
  inside: { kind: export_statement }
  not: { has: { field: return_type, regex: '.' } }
message: "Declare the return type of this exported function"
```

## How to fix

1. Add `: ReturnType` after the parameter list.

## Examples

### TypeScript

#### Don't

```ts
export function load(id: string) {
  return fetch(id);
}
```

#### Do

```ts
export function load(id: string): Promise<Response> {
  return fetch(id);
}
```

### TSX

#### Don't

```tsx
export function load(id: string) {
  return fetch(id);
}
```

#### Do

```tsx
export function load(id: string): Promise<Response> {
  return fetch(id);
}
```

## Differences from the source

Only exported function declarations' return types are checked; the source also checks arrow functions, parameters and class methods.
