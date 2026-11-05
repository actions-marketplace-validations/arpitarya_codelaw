---
id: js/no-new-wrappers
title: "No primitive wrapper objects"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "dropping `new` changes the value from an object to a primitive, which the author confirms"
fix_pending: "suggest"
stability: preview
ported_from: [eslint/no-new-wrappers, eslint-plugin-sonarjs/no-primitive-wrappers]
since: 0.5.0
---

`String`, `Number` and `Boolean` are not called with `new`.

**Why:** `new Boolean(false)` is an object, so it is truthy; wrapper objects behave unlike their primitives everywhere.

```codelaw
match:
  pattern: new $W($$$A)
constraints:
  W: { kind: identifier, regex: '^(String|Number|Boolean)$' }
where:
  $W: builtin
message: "Do not use `$W` as a constructor"
```

## How to fix

1. Remove `new` to convert to the primitive, or use the literal.

## Examples

### JavaScript

#### Don't

```js
const ok = new Boolean(flag);
```

#### Do

```js
const ok = Boolean(flag);
```

### JSX

#### Don't

```jsx
const ok = new Boolean(flag);
```

#### Do

```jsx
const ok = Boolean(flag);
```

### TypeScript

#### Don't

```ts
const ok = new Boolean(flag);
```

#### Do

```ts
const ok = Boolean(flag);
```

### TSX

#### Don't

```tsx
const ok = new Boolean(flag);
```

#### Do

```tsx
const ok = Boolean(flag);
```
