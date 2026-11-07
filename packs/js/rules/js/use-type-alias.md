---
id: js/use-type-alias
title: "A union written three times is a type alias"
languages: [typescript, tsx]
fix: manual
fix_reason: "the alias needs a name, which is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/use-type-alias]
since: 0.5.0
---

A union or intersection type of three or more members, written out the same way three or more times in a file, is declared once with `type Name = …` and referred to by its name. The first use is reported.

**Why:** One name for one type keeps the copies from drifting apart.

```codelaw
native: js/use-type-alias
```

## How to fix

1. Declare `type Name = …` once and use `Name` in each place.

## Examples

### TypeScript

#### Don't

```ts
let a: string | number | boolean;
let b: string | number | boolean;
let c: string | number | boolean;
```

#### Do

```ts
type Scalar = string | number | boolean;
let a: Scalar;
let b: Scalar;
let c: Scalar;
```


### TSX

#### Don't

```tsx
let a: string | number | boolean;
let b: string | number | boolean;
let c: string | number | boolean;
```

#### Do

```tsx
type Scalar = string | number | boolean;
let a: Scalar;
let b: Scalar;
let c: Scalar;
```
