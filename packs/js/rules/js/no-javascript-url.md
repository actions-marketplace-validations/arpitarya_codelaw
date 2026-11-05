---
id: js/no-javascript-url
title: "No javascript: URLs"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the replacement handler is code the author writes"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-unicorn/no-javascript-url]
since: 0.5.0
---

A string does not start with `javascript:`.

**Why:** Such a URL runs code when followed, bypasses content-security policies that forbid inline script, and is an injection risk.

```codelaw

match:
  kind: string_fragment
  regex: '^\s*javascript:'
message: "A `javascript:` URL is `eval` in disguise; attach a handler instead"
```

## How to fix

1. Remove the URL and attach the behaviour with `addEventListener`.

## Examples

### JavaScript

#### Don't

```js
link.href = "javascript:void(0)";
```

#### Do

```js
link.addEventListener("click", (e) => e.preventDefault());
```

### JSX

#### Don't

```jsx
link.href = "javascript:void(0)";
```

#### Do

```jsx
link.addEventListener("click", (e) => e.preventDefault());
```

### TypeScript

#### Don't

```ts
link.href = "javascript:void(0)";
```

#### Do

```ts
link.addEventListener("click", (e) => e.preventDefault());
```

### TSX

#### Don't

```tsx
link.href = "javascript:void(0)";
```

#### Do

```tsx
link.addEventListener("click", (e) => e.preventDefault());
```
