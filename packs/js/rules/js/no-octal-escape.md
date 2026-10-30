---
id: js/no-octal-escape
title: "No octal escapes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the equivalent `\\u` or `\\x` escape is computed from the octal value"
stability: preview
ported_from: [eslint/no-octal-escape]
since: 0.5.0
---

Strings do not use octal escape sequences such as `\251`.

**Why:** They are deprecated and forbidden in strict mode; a Unicode escape says the same.

```codelaw
match:
  kind: string
  regex: '(^|[^\\])(\\\\)*\\([1-7]|0[0-7])'
message: "Use a `\\u` or `\\x` escape instead of an octal escape"
```

## How to fix

1. Replace the octal escape with its `\u` or `\x` form.

## Examples

### JavaScript

#### Don't

```js
const c = 'Copyright \251';
```

#### Do

```js
const c = 'Copyright \u00A9';
```

### JSX

#### Don't

```jsx
const c = 'Copyright \251';
```

#### Do

```jsx
const c = 'Copyright \u00A9';
```

### TypeScript

#### Don't

```ts
const c = 'Copyright \251';
```

#### Do

```ts
const c = 'Copyright \u00A9';
```

### TSX

#### Don't

```tsx
const c = 'Copyright \251';
```

#### Do

```tsx
const c = 'Copyright \u00A9';
```
