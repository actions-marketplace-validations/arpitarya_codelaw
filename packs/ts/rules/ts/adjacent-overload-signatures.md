---
id: ts/adjacent-overload-signatures
title: "Overloads of one name sit together"
languages: [typescript, tsx]
fix: manual
fix_reason: "restructure: the overloads have to be moved next to each other, and the order between them is the author's"
fix_pending: "suggest"
stability: preview
ported_from: [typescript-eslint/adjacent-overload-signatures]
since: 0.5.0
---

The overload signatures of one function or method are written one after another, with no other member between them.

**Why:** Overloads read as one declaration; one split from the others is easy to miss when the function changes.

```codelaw
utils:
  member:
    any:
      - kind: method_signature
      - kind: abstract_method_signature
      - kind: method_definition
        not: { regex: '^(?:(?:public|protected|private|static|override|async)\s+)*[gs]et\s' }
      - kind: function_signature
      - kind: function_declaration
match:
  matches: member
  all:
    - has: { field: name, pattern: $N }
    - follows:
        stopBy: end
        matches: member
        has: { field: name, pattern: $N }
    - not:
        follows:
          stopBy: { not: { any: [{ regex: '^[;,]$' }, { kind: comment }] } }
          matches: member
          has: { field: name, pattern: $N }
message: "The overloads of `$N` are not next to each other"
```

## How to fix

1. Move this signature up so that every overload of the name is together, before the implementation.

## Examples

### TypeScript

#### Don't

```ts
class Store {
  get(id: string): Item;
  clear(): void;
  get(ids: string[]): Item[];
  get(x: string | string[]) { return lookup(x); }
}
```

#### Do

```ts
class Store {
  get(id: string): Item;
  get(ids: string[]): Item[];
  get(x: string | string[]) { return lookup(x); }
  clear(): void;
}
```

### TSX

#### Don't

```tsx
class Store {
  get(id: string): Item;
  clear(): void;
  get(ids: string[]): Item[];
  get(x: string | string[]) { return lookup(x); }
}
```

#### Do

```tsx
class Store {
  get(id: string): Item;
  get(ids: string[]): Item[];
  get(x: string | string[]) { return lookup(x); }
  clear(): void;
}
```
