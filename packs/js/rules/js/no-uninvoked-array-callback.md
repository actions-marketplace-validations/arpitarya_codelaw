---
id: js/no-uninvoked-array-callback
title: "map() over new Array(n) never runs"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the callback starts running, once per index, where today it never runs"
stability: preview
ported_from: [oxlint/uninvoked-array-callback]
since: 0.5.0
---

Code builds an array of `n` items with `Array.from({ length: n }, ...)`, not by calling `map`, `forEach` or another iterating method on `new Array(n)`.

**Why:** `new Array(n)` has `n` empty slots, not `n` undefined values, and the iterating methods skip empty slots: the callback is never called.

```codelaw
match:
  any:
    - pattern: new Array($N).$M($$$A)
    - pattern: Array($N).$M($$$A)
constraints:
  N: { not: { any: [{kind: string}, {kind: template_string}, {kind: spread_element}] } }
  M: { regex: ^(map|forEach|filter|every|some|flatMap|reduce|reduceRight)$ }
message: "`$M` skips the empty slots of `Array($N)`; use `Array.from({ length: $N })`"
fix: "Array.from({ length: $N }).$M($$$A)"
```

## Examples

### JavaScript

#### Don't

```js
const ids = new Array(5).map((_, i) => i);
```

#### Do

```js
const ids = Array.from({ length: 5 }).map((_, i) => i);
```

### JSX

#### Don't

```jsx
const ids = new Array(5).map((_, i) => i);
```

#### Do

```jsx
const ids = Array.from({ length: 5 }).map((_, i) => i);
```

### TypeScript

#### Don't

```ts
const ids = new Array(5).map((_, i) => i);
```

#### Do

```ts
const ids = Array.from({ length: 5 }).map((_, i) => i);
```

### TSX

#### Don't

```tsx
const ids = new Array(5).map((_, i) => i);
```

#### Do

```tsx
const ids = Array.from({ length: 5 }).map((_, i) => i);
```
