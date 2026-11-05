---
id: js/no-nested-template-literals
title: "No template literal inside another"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the variable for the inner string needs a name"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-nested-template-literals]
since: 0.5.0
---

A template literal is not nested inside another's `${…}`.

**Why:** Nested backticks are hard to read and to match up.

```codelaw

match:
  kind: template_string
  inside: { stopBy: end, kind: template_substitution }
message: "Build the inner string in a variable first"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const s = `a ${ok ? `b ${c}` : "d"}`;
```

#### Do

```js
const inner = ok ? `b ${c}` : "d";
const s = `a ${inner}`;
```

### JSX

#### Don't

```jsx
const s = `a ${ok ? `b ${c}` : "d"}`;
```

#### Do

```jsx
const inner = ok ? `b ${c}` : "d";
const s = `a ${inner}`;
```

### TypeScript

#### Don't

```ts
const s = `a ${ok ? `b ${c}` : "d"}`;
```

#### Do

```ts
const inner = ok ? `b ${c}` : "d";
const s = `a ${inner}`;
```

### TSX

#### Don't

```tsx
const s = `a ${ok ? `b ${c}` : "d"}`;
```

#### Do

```tsx
const inner = ok ? `b ${c}` : "d";
const s = `a ${inner}`;
```
