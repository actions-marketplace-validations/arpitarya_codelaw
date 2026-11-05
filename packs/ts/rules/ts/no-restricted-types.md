---
id: ts/no-restricted-types
title: "No type the project restricts"
languages: [typescript, tsx]
fix: suggest
fix_reason: "the replacement comes from the option's `fixWith`, and another type may not fit every use; a type with no `fixWith` is reported without an edit"
stability: preview
options:
  types: { type: "object", default: {}, description: "type text → a message, or { message, fixWith }" }
ported_from: [typescript-eslint/no-restricted-types]
since: 0.5.0
---

A type named in the `types` option isn't used. With no option set, the rule does nothing.

**Why:** A project bans types it has replaced, such as `Object` or a legacy alias.

```codelaw
native: ts/no-restricted-types
```

## How to fix

1. Use the type the message names.

## Examples

### TypeScript

#### Don't

```ts options={"types":{"Object":{"message":"lowercase","fixWith":"object"}}}
let a: Object = {};
```

#### Do

```ts options={"types":{"Object":{"message":"lowercase","fixWith":"object"}}}
let a: object = {};
```

### TSX

#### Don't

```tsx options={"types":{"Object":{"message":"lowercase","fixWith":"object"}}}
let a: Object = {};
```

#### Do

```tsx options={"types":{"Object":{"message":"lowercase","fixWith":"object"}}}
let a: object = {};
```
