---
id: js/consistent-optional-chaining
title: "One base, one kind of member access, in an && or || chain"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "adding `?.` turns a throw on a missing base into `undefined`; removing it turns that `undefined` back into a throw. The edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-optional-chaining]
since: 0.5.0
---

When both sides of `||` or `&&` read a member off the same base, they read it the same way. In `a?.x || a.y` the right side is reached when `a` may be missing, so it is `a?.y` too; in `a.x || a?.y` the left side already proved `a` is there, so the right is `a.y`. In `a?.x && a?.y` the right is reached only when `a` exists, so `a.y` is enough, and in `a.x && a?.y` likewise.

**Why:** Mixed access on one base makes a reader wonder whether the base can be missing; the consistent form says it once.

```codelaw
utils:
  base:
    any:
      - kind: identifier
      - kind: this
      - kind: member_expression
        not: { has: { kind: optional_chain } }
match:
  kind: binary_expression
  any:
    - all:
        - pattern: "$A?.$B || $A.$C"
        - has: { field: right, pattern: $R1 }
    - all:
        - pattern: "$A.$B || $A?.$C"
        - has: { field: right, pattern: $R2 }
    - all:
        - pattern: "$A?.$B && $A?.$C"
        - has: { field: right, pattern: $R2 }
    - all:
        - pattern: "$A.$B && $A?.$C"
        - has: { field: right, pattern: $R2 }
constraints:
  A: { matches: base }
message: "Read `$A` the same way on both sides"
fix:
  - replace: { target: $R1, with: "$A?.$C" }
  - replace: { target: $R2, with: "$A.$C" }
```

## Examples

### JavaScript

#### Don't

```js
const label = user?.name || user.email;
```

#### Do

```js
const label = user?.name || user?.email;
```


### JSX

#### Don't

```jsx
const label = user?.name || user.email;
```

#### Do

```jsx
const label = user?.name || user?.email;
```


### TypeScript

#### Don't

```ts
const label = user?.name || user.email;
```

#### Do

```ts
const label = user?.name || user?.email;
```


### TSX

#### Don't

```tsx
const label = user?.name || user.email;
```

#### Do

```tsx
const label = user?.name || user?.email;
```
