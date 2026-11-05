---
id: js/no-document-cookie-biome
title: "No direct writes to document.cookie"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving to the Cookie Store API makes the code asynchronous"
fix_exception: restructure
stability: preview
ported_from: [biome/suspicious/noDocumentCookie]
since: 0.5.0
---

Cookies are not set by assigning to `document.cookie`.

**Why:** The string format is easy to get wrong (encoding, attributes) and each assignment silently sets one cookie; the Cookie Store API or a helper says what it does.

```codelaw
match:
  any:
    - pattern: document.cookie = $V
    - pattern: document.cookie += $V
message: "Do not assign to `document.cookie`; use the Cookie Store API or a cookie helper"
```

## How to fix

1. Use `cookieStore.set(name, value)` or the project's cookie helper.

## Examples

### JavaScript

#### Don't

```js
document.cookie = 'theme=dark; path=/';
```

#### Do

```js
await cookieStore.set('theme', 'dark');
```

### JSX

#### Don't

```jsx
document.cookie = 'theme=dark; path=/';
```

#### Do

```jsx
await cookieStore.set('theme', 'dark');
```

### TypeScript

#### Don't

```ts
document.cookie = 'theme=dark; path=/';
```

#### Do

```ts
await cookieStore.set('theme', 'dark');
```

### TSX

#### Don't

```tsx
document.cookie = 'theme=dark; path=/';
```

#### Do

```tsx
await cookieStore.set('theme', 'dark');
```
