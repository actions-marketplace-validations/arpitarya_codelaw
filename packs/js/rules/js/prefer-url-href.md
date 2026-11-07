---
id: js/prefer-url-href
title: "url.href rather than toString() on a new URL"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-url-href]
since: 0.5.0
---

A URL's text is read from `.href`, not by calling `.toString()` on it. Upstream also follows variables known to hold a `URL`; this port covers `new URL(…).toString()`.

**Why:** `href` is the property `toString()` returns, and it can be assigned too.

```codelaw
match:
  pattern: "new URL($$$A).toString()"
message: "Read `.href`"
fix: "new URL($$$A).href"
```

## Examples

### JavaScript

#### Don't

```js
const link = new URL(path, base).toString();
```

#### Do

```js
const link = new URL(path, base).href;
```


### JSX

#### Don't

```jsx
const link = new URL(path, base).toString();
```

#### Do

```jsx
const link = new URL(path, base).href;
```


### TypeScript

#### Don't

```ts
const link = new URL(path, base).toString();
```

#### Do

```ts
const link = new URL(path, base).href;
```


### TSX

#### Don't

```tsx
const link = new URL(path, base).toString();
```

#### Do

```tsx
const link = new URL(path, base).href;
```
