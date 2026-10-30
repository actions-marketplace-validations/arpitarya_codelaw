---
id: js/no-flat-map-identity
title: "flat, not flatMap of the identity"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`flat()` and `flatMap` agree on arrays; review on an object whose `flat` differs"
stability: preview
ported_from: [biome/complexity/noFlatMapIdentity]
since: 0.5.0
---

`xs.flatMap((x) => x)` is written `xs.flat()`.

**Why:** Mapping each item to itself and flattening is what `flat()` does, and says.

```codelaw
match:
  any:
    - pattern: $XS.flatMap(($X) => $X)
    - pattern: $XS.flatMap($X => $X)
    - pattern: "$XS.flatMap(function ($X) { return $X; })"
message: "Use `.flat()` instead of `.flatMap` with an identity callback"
fix: "$XS.flat()"
```

## Examples

### JavaScript

#### Don't

```js
const all = groups.flatMap((g) => g);
```

#### Do

```js
const all = groups.flat();
```

### JSX

#### Don't

```jsx
const all = groups.flatMap((g) => g);
```

#### Do

```jsx
const all = groups.flat();
```

### TypeScript

#### Don't

```ts
const all = groups.flatMap((g) => g);
```

#### Do

```ts
const all = groups.flat();
```

### TSX

#### Don't

```tsx
const all = groups.flatMap((g) => g);
```

#### Do

```tsx
const all = groups.flat();
```
