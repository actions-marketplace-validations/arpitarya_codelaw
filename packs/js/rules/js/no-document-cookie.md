---
id: js/no-document-cookie
title: "No document.cookie"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the replacement is async and its arguments come from parsing the string"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-document-cookie]
since: 0.5.0
---

`document.cookie` is not assigned to.

**Why:** Its string format is easy to get wrong (encoding, attributes), and each assignment silently sets one cookie.

```codelaw

match:
  kind: assignment_expression
  has: { field: left, pattern: document.cookie }
message: "Write cookies through the Cookie Store API or a cookie library"
```

## How to fix

1. Use `cookieStore.set(name, value)` or a cookie library that encodes the value and attributes.

## Examples

### JavaScript

#### Don't

```js
document.cookie = "theme=dark";
```

#### Do

```js
await cookieStore.set("theme", "dark");
```

### JSX

#### Don't

```jsx
document.cookie = "theme=dark";
```

#### Do

```jsx
await cookieStore.set("theme", "dark");
```

### TypeScript

#### Don't

```ts
document.cookie = "theme=dark";
```

#### Do

```ts
await cookieStore.set("theme", "dark");
```

### TSX

#### Don't

```tsx
document.cookie = "theme=dark";
```

#### Do

```tsx
await cookieStore.set("theme", "dark");
```
