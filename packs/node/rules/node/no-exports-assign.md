---
id: node/no-exports-assign
title: "No assigning to exports"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "assigning to `module.exports` makes the module export that value, which it did not before; the reviewer confirms it was meant and that no `exports.x` use depends on the old object"
stability: preview
ported_from: [eslint-plugin-n/no-exports-assign]
since: 0.1.0
---

Code doesn't assign to the CommonJS `exports` variable; `exports = module.exports = …` is allowed.

**Why:** Assigning to `exports` only rebinds a local variable, so nothing is exported.

```codelaw
match:
  kind: assignment_expression
  has: { field: left, kind: identifier, regex: ^exports$, pattern: $E }
  not:
    any:
      - has: { field: right, regex: '^module\.exports\b' }
      - inside:
          stopBy: neighbor
          kind: assignment_expression
          has: { field: left, regex: '^module\.exports$' }
where:
  $E: undeclared
message: "Assigning to `exports` exports nothing; assign to `module.exports`"
fix:
  - replace: { target: $E, with: "module.exports" }
```

## How to fix

1. Assign to `module.exports`, or set properties on `exports` (`exports.name = …`).

## Examples

### JavaScript

#### Don't

```js
exports = { start };
```

#### Do

```js
module.exports = { start };
```

### JSX

#### Don't

```jsx
exports = { start };
```

#### Do

```jsx
module.exports = { start };
```

### TypeScript

#### Don't

```ts
exports = { start };
```

#### Do

```ts
module.exports = { start };
```

### TSX

#### Don't

```tsx
exports = { start };
```

#### Do

```tsx
module.exports = { start };
```
