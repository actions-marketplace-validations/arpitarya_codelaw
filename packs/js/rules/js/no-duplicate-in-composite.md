---
id: js/no-duplicate-in-composite
title: "No type written twice in a union or intersection"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-sonarjs/no-duplicate-in-composite]
since: 0.5.0
---

A union or intersection does not name the same type twice side by side: `A | A` is `A`.

**Why:** The repeat adds nothing and usually hides the type that was meant.

```codelaw
match:
  any:
    - pattern: { context: "type T = $A | $A", selector: union_type }
    - pattern: { context: "type T = $A & $A", selector: intersection_type }
message: "`$A` appears twice"
fix: "$A"
```

## Examples

### TypeScript

#### Don't

```ts
type Id = string | string;
```

#### Do

```ts
type Id = string;
```


### TSX

#### Don't

```tsx
type Id = string | string;
```

#### Do

```tsx
type Id = string;
```
