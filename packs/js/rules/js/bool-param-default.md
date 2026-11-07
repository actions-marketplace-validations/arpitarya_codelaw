---
id: js/bool-param-default
title: "An optional boolean parameter has a default"
languages: [typescript, tsx]
fix: manual
fix_reason: "the default value is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/bool-param-default]
since: 0.5.0
---

An optional parameter typed `boolean` (`flag?: boolean`) is given a default value instead.

**Why:** Without a default the parameter is `true`, `false` or `undefined`, and the code must handle three cases where two were meant.

```codelaw
match:
  kind: optional_parameter
  has: { kind: type_annotation, has: { kind: predefined_type, regex: '^boolean$' } }
  not: { has: { field: value } }
message: "Give this optional boolean a default value"
```

## How to fix

1. Write `flag = false` (or `true`) instead of `flag?: boolean`.

## Examples

### TypeScript

#### Don't

```ts
function render(compact?: boolean) {}
```

#### Do

```ts
function render(compact = false) {}
```


### TSX

#### Don't

```tsx
function render(compact?: boolean) {}
```

#### Do

```tsx
function render(compact = false) {}
```
