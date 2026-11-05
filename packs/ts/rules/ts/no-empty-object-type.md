---
id: ts/no-empty-object-type
title: "No empty interfaces or {} types"
languages: [typescript, tsx]
fix: suggest
fix_reason: "the right replacement — `object`, `unknown`, `Record<string, never>` or `NonNullable<unknown>` — depends on intent, so each is offered (an interface with a generic or `extends` clause carries no edit)"
stability: preview
ported_from: [typescript-eslint/no-empty-object-type]
since: 0.1.0
---

Code doesn't declare an empty interface or use the type `{}`.

**Why:** `{}` accepts every value except `null` and `undefined`, which is almost never what an
empty-looking type was meant to say.

```codelaw
match:
  any:
    - kind: interface_declaration
      has: { field: body, regex: '^\{\s*\}$' }
      all:
        - pattern: $I
        - has: { field: name, pattern: $IN }
        - not: { has: { any: [{ kind: extends_type_clause }, { kind: type_parameters }] } }
    - kind: interface_declaration
      has: { field: body, regex: '^\{\s*\}$' }
    - kind: object_type
      regex: '^\{\s*\}$'
      pattern: $O
message: "`{}` means any non-null value, not an empty object; say what you mean"
fix:
  choices:
    - label: object
      default: true
      fix:
        - replace: { target: $I, with: "type $IN = object;" }
        - replace: { target: $O, with: "object" }
    - label: unknown
      fix:
        - replace: { target: $I, with: "type $IN = unknown;" }
        - replace: { target: $O, with: "unknown" }
    - label: no keys
      fix:
        - replace: { target: $I, with: "type $IN = Record<string, never>;" }
        - replace: { target: $O, with: "Record<string, never>" }
    - label: any non-null value
      fix:
        - replace: { target: $I, with: "type $IN = NonNullable<unknown>;" }
        - replace: { target: $O, with: "NonNullable<unknown>" }
```

## How to fix

1. For any object, use `object`.
2. For any value, use `unknown`.
3. For an object with no keys, use `Record<string, never>`.

## Examples

### TypeScript

#### Don't

```ts
interface Options {}
type Empty = {};
```

#### Fixed (object)

```ts
type Options = object;
type Empty = object;
```

#### Fixed (unknown)

```ts
type Options = unknown;
type Empty = unknown;
```

#### Fixed (no keys)

```ts
type Options = Record<string, never>;
type Empty = Record<string, never>;
```

#### Fixed (any non-null value)

```ts
type Options = NonNullable<unknown>;
type Empty = NonNullable<unknown>;
```

#### Do

```ts
type Options = Record<string, never>;
type Anything = unknown;
```

### TSX

#### Don't

```tsx
interface Options {}
type Empty = {};
```

#### Fixed (object)

```tsx
type Options = object;
type Empty = object;
```

#### Fixed (unknown)

```tsx
type Options = unknown;
type Empty = unknown;
```

#### Fixed (no keys)

```tsx
type Options = Record<string, never>;
type Empty = Record<string, never>;
```

#### Fixed (any non-null value)

```tsx
type Options = NonNullable<unknown>;
type Empty = NonNullable<unknown>;
```

#### Do

```tsx
type Options = Record<string, never>;
type Anything = unknown;
```
