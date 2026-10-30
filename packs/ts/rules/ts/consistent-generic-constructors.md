---
id: ts/consistent-generic-constructors
title: "Type arguments on the constructor, not the annotation"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/consistent-generic-constructors]
since: 0.5.0
---

`const m = new Map<K, V>()` is written instead of `const m: Map<K, V> = new Map()`.

**Why:** Writing the type once, on the constructor, avoids repeating the name and lets the variable's type be inferred.

```codelaw
match:
  kind: variable_declarator
  all:
  - has: {field: name, pattern: $N}
  - has:
      field: type
      has:
        kind: generic_type
        pattern: $G
        all:
        - has: {field: name, pattern: $C}
        - has: {field: type_arguments, pattern: $A}
  - has:
      field: value
      kind: new_expression
      all:
      - has: {field: constructor, pattern: $C}
      - not:
          has: {field: type_arguments, regex: .}
      - has: {field: arguments, pattern: $ARGS}
message: 'Put the type arguments on the constructor: `new $C$A()`'
fix: $N = new $C$A$ARGS
```

## Examples

### TypeScript

#### Don't

```ts
const seen: Set<string> = new Set();
```

#### Do

```ts
const seen = new Set<string>();
```

#### Fixed

```ts
const seen = new Set<string>();
```

### TSX

#### Don't

```tsx
const seen: Set<string> = new Set();
```

#### Do

```tsx
const seen = new Set<string>();
```

#### Fixed

```tsx
const seen = new Set<string>();
```
