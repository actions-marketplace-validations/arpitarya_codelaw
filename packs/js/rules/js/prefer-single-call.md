---
id: js/prefer-single-call
title: "One push() with several items"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "merging two statements is a cross-statement edit"
fix_pending: "suggest; needs computed-edit"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-single-call]
since: 0.5.0
---

Consecutive `push` calls on the same array are merged. Upstream also covers `classList.add` and others; this port covers `push`.

**Why:** One call says the same in one statement.

```codelaw

match:
  kind: expression_statement
  pattern: $A.push($$$X);
  precedes: { pattern: $A.push($$$Y); }
message: "Push both values in one call: `$A.push(a, b)`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
list.push(a);
list.push(b);
```

#### Do

```js
list.push(a, b);
```

### JSX

#### Don't

```jsx
list.push(a);
list.push(b);
```

#### Do

```jsx
list.push(a, b);
```

### TypeScript

#### Don't

```ts
list.push(a);
list.push(b);
```

#### Do

```ts
list.push(a, b);
```

### TSX

#### Don't

```tsx
list.push(a);
list.push(b);
```

#### Do

```tsx
list.push(a, b);
```
