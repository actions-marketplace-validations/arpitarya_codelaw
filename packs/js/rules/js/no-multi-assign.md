---
id: js/no-multi-assign
title: "No chained assignment"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "splitting the chain into statements is a change of the author's code shape"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint/no-multi-assign]
since: 0.5.0
---

An assignment's value is not itself an assignment (`a = b = c`).

**Why:** Chained assignments read left to right but run right to left, and with `const a = b = 1` create an implicit global.

```codelaw
match:
  any:
    - kind: assignment_expression
      has: { field: right, kind: assignment_expression }
    - kind: variable_declarator
      has: { field: value, kind: assignment_expression }
message: "Split this chained assignment into separate statements"
```

## How to fix

1. Write one assignment per statement.

## Examples

### JavaScript

#### Don't

```js
a = b = 0;
```

#### Do

```js
a = 0;
b = 0;
```

### JSX

#### Don't

```jsx
a = b = 0;
```

#### Do

```jsx
a = 0;
b = 0;
```

### TypeScript

#### Don't

```ts
a = b = 0;
```

#### Do

```ts
a = 0;
b = 0;
```

### TSX

#### Don't

```tsx
a = b = 0;
```

#### Do

```tsx
a = 0;
b = 0;
```
