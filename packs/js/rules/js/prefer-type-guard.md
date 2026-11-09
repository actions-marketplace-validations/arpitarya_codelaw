---
id: js/prefer-type-guard
title: "A type predicate for a function that only checks a cast"
languages: [typescript, tsx]
fix: suggest
fix_reason: "the new return type narrows the argument at every call site, which can change what compiles there; the author accepts that"
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-type-guard]
since: 0.5.0
---

A function declaration or class method with one statement, `return`, that tests a property of a cast of its only parameter (or of `this`) — `(x as T).p !== undefined`, `(x as T).p != undefined`, `Boolean((x as T).p)` or `!!(x as T).p`, also with `<T>x` — is reported at its name. The suggestion gives it the return type `x is T` (replacing a `boolean` annotation). A cast to `any`, a function that already returns a predicate, one with other statements, and a function with more than one parameter are not reported. Behaviour of eslint-plugin-sonarjs 4.2.2 (read, not copied).

**Why:** The check says what the value is, but the compiler is not told: every caller casts again.

```codelaw
native: js/prefer-type-guard
```

## Examples

### TypeScript

#### Don't

```ts
function isFish(pet: Pet): boolean {
  return (pet as Fish).swim !== undefined;
}
```

#### Do

```ts
function isFish(pet: Pet): pet is Fish {
  return (pet as Fish).swim !== undefined;
}
```

#### Fixed

```ts
function isFish(pet: Pet): pet is Fish {
  return (pet as Fish).swim !== undefined;
}
```


### TSX

#### Don't

```tsx
function isFish(pet: Pet): boolean {
  return (pet as Fish).swim !== undefined;
}
```

#### Do

```tsx
function isFish(pet: Pet): pet is Fish {
  return (pet as Fish).swim !== undefined;
}
```

#### Fixed

```tsx
function isFish(pet: Pet): pet is Fish {
  return (pet as Fish).swim !== undefined;
}
```
