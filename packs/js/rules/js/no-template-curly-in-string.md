---
id: js/no-template-curly-in-string
title: "No template placeholder in a plain string"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "switching to backticks is right only if interpolation was meant"
stability: preview
ported_from: [eslint/no-template-curly-in-string]
since: 0.5.0
---

A plain string does not contain `${…}`.

**Why:** It is almost always a template literal written with the wrong quotes, so the placeholder is printed as is.

```codelaw
match:
  kind: string
  regex: '\$\{[^}]*\}'
message: "This string holds `${…}`; did you mean a template literal?"
```

## How to fix

1. Use backticks if the value should be interpolated.

## Examples

### JavaScript

#### Don't

```js
const msg = 'Hello ${name}';
```

#### Do

```js
const msg = `Hello ${name}`;
```

### JSX

#### Don't

```jsx
const msg = 'Hello ${name}';
```

#### Do

```jsx
const msg = `Hello ${name}`;
```

### TypeScript

#### Don't

```ts
const msg = 'Hello ${name}';
```

#### Do

```ts
const msg = `Hello ${name}`;
```

### TSX

#### Don't

```tsx
const msg = 'Hello ${name}';
```

#### Do

```tsx
const msg = `Hello ${name}`;
```
