---
id: js/escape-case
title: "Upper-case hex digits in escapes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the fix upper-cases the digits, which a fix template cannot compute"
fix_pending: "auto; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint-plugin-unicorn/escape-case]
since: 0.5.0
---

Hexadecimal digits in `\x`, `\u` and `\u{…}` escapes are upper case.

**Why:** Mixed case makes the same character look like two; one case is easier to search for.

```codelaw

match:
  kind: escape_sequence
  regex: '^\\(x[0-9A-Fa-f]*[a-f][0-9A-Fa-f]*|u[0-9A-Fa-f]*[a-f][0-9A-Fa-f]*|u\{[0-9A-Fa-f]*[a-f][0-9A-Fa-f]*\})$'
message: "Write the hex digits of `$$MATCH` in upper case"
```

## How to fix

1. Upper-case the letters a–f in the escape (`\u00a0` → `\u00A0`).

## Examples

### JavaScript

#### Don't

```js
const nbsp = "\u00a0";
```

#### Do

```js
const nbsp = "\u00A0";
```

### JSX

#### Don't

```jsx
const nbsp = "\u00a0";
```

#### Do

```jsx
const nbsp = "\u00A0";
```

### TypeScript

#### Don't

```ts
const nbsp = "\u00a0";
```

#### Do

```ts
const nbsp = "\u00A0";
```

### TSX

#### Don't

```tsx
const nbsp = "\u00a0";
```

#### Do

```tsx
const nbsp = "\u00A0";
```
