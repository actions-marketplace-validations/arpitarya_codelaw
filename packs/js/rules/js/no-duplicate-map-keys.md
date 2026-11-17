---
id: js/no-duplicate-map-keys
title: "No key twice in a `new Map([...])`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which of the two entries is meant is the author's; the later one silently replaces the earlier"
fix_exception: restructure
stability: preview
ported_from: [biome/nursery/noDuplicateMapKeys]
since: 0.5.1
---

A `new Map([...])` whose array literal holds two entries with the same string or number key is reported. Keys that are not literals are not compared. Behaviour of Biome's `noDuplicateMapKeys` for the constructor form.

**Why:** The later entry replaces the earlier one without a word, so one of the two values written is dead code.

```codelaw
match:
  all:
    - pattern: "[$K, $V]"
    - follows: { stopBy: end, pattern: "[$K, $V1]" }
    - inside:
        kind: array
        inside:
          kind: arguments
          inside: { kind: new_expression, has: { field: constructor, regex: '^Map$' } }
constraints:
  K: { any: [{ kind: string }, { kind: number }] }
message: "This `Map` has the key `$K` twice; the later entry replaces the earlier one"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const roles = new Map([["admin", 1], ["admin", 2]]);
```

#### Do

```js
const roles = new Map([["admin", 1], ["guest", 2]]);
```

### JSX

#### Don't

```jsx
const roles = new Map([["admin", 1], ["admin", 2]]);
```

#### Do

```jsx
const roles = new Map([["admin", 1], ["guest", 2]]);
```

### TypeScript

#### Don't

```ts
const roles = new Map([["admin", 1], ["admin", 2]]);
```

#### Do

```ts
const roles = new Map([["admin", 1], ["guest", 2]]);
```

### TSX

#### Don't

```tsx
const roles = new Map([["admin", 1], ["admin", 2]]);
```

#### Do

```tsx
const roles = new Map([["admin", 1], ["guest", 2]]);
```
