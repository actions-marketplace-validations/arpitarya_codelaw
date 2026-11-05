---
id: js/no-eval
title: "No eval"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what replaces it depends on what the string held"
fix_exception: security
stability: preview
ported_from: [eslint/no-eval]
since: 0.5.0
---

`eval` is not called.

**Why:** It runs a string as code: slow, impossible to analyse, and an injection risk.

```codelaw
match:
  any:
    - pattern: $F($$$A)
    - pattern: window.$F($$$A)
    - pattern: globalThis.$F($$$A)
constraints:
  F: { regex: '^eval$' }
message: "Do not use `eval`"
```

## How to fix

1. Parse data with `JSON.parse`, or call the function the string named directly.

## Examples

### JavaScript

#### Don't

```js
const data = eval('(' + text + ')');
```

#### Do

```js
const data = JSON.parse(text);
```

### JSX

#### Don't

```jsx
const data = eval('(' + text + ')');
```

#### Do

```jsx
const data = JSON.parse(text);
```

### TypeScript

#### Don't

```ts
const data = eval('(' + text + ')');
```

#### Do

```ts
const data = JSON.parse(text);
```

### TSX

#### Don't

```tsx
const data = eval('(' + text + ')');
```

#### Do

```tsx
const data = JSON.parse(text);
```
