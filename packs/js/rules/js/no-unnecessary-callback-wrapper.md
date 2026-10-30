---
id: js/no-unnecessary-callback-wrapper
title: "Pass the function, not a wrapper"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "unwrapping `x => f(x)` to `f` passes every argument the caller supplies (`map` passes the index too) and loses the receiver of `obj.f`"
stability: preview
ported_from: [tslint/no-unnecessary-callback-wrapper]
since: 0.5.0
---

An arrow function that only forwards its own parameters, in order, to another function is replaced by that function.

**Why:** The wrapper is a closure allocated on every evaluation and one more layer to read through for no behaviour.

```codelaw
match:
  kind: arrow_function
  not: { regex: ^async\b }
  any:
    - pattern: $A => $F($A)
    - pattern: ($A) => $F($A)
    - pattern: ($A, $B) => $F($A, $B)
    - pattern: ($A, $B, $C) => $F($A, $B, $C)
constraints:
  F: { any: [{kind: identifier}, {kind: member_expression}] }
message: "This arrow only forwards to `$F`; pass `$F` itself"
fix: "$F"
```

## Examples

### JavaScript

#### Don't

```js
items.forEach((item) => render(item));
```

#### Do

```js
items.forEach(render);
```

### JSX

#### Don't

```jsx
items.forEach((item) => render(item));
```

#### Do

```jsx
items.forEach(render);
```

### TypeScript

#### Don't

```ts
items.forEach((item) => render(item));
```

#### Do

```ts
items.forEach(render);
```

### TSX

#### Don't

```tsx
items.forEach((item) => render(item));
```

#### Do

```tsx
items.forEach(render);
```
