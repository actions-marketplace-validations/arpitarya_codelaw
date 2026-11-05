---
id: js/no-literal-call
title: "No call on a literal"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what was meant (a missing `;`, a variable) is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-literal-call]
since: 0.5.0
---

A literal is not called as a function.

**Why:** It always throws a TypeError; usually a missing operator or a missing line break before a parenthesis.

```codelaw

match:
  kind: call_expression
  has: { field: function, any: [{kind: string}, {kind: number}, {kind: "true"}, {kind: "false"}, {kind: "null"}, {kind: regex}, {kind: array}, {kind: object}, {kind: template_string}] }
message: "A literal is not a function; this call throws"
```

## How to fix

1. Add the missing operator or semicolon, or call the function that was meant.

## Examples

### JavaScript

#### Don't

```js
const x = "ready"();
```

#### Do

```js
const x = ready();
```

### JSX

#### Don't

```jsx
const x = "ready"();
```

#### Do

```jsx
const x = ready();
```

### TypeScript

#### Don't

```ts
const x = "ready"();
```

#### Do

```ts
const x = ready();
```

### TSX

#### Don't

```tsx
const x = "ready"();
```

#### Do

```tsx
const x = ready();
```
