---
id: js/cookie-no-httponly
title: "Cookies are HttpOnly"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "whether the page needs the cookie in script is the author's call; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/cookie-no-httponly]
since: 0.5.0
---

A cookie options object does not set `httpOnly: false`.

**Why:** A cookie readable from JavaScript is stolen by any XSS on the page.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^(httpOnly|"httpOnly")$
  all:
  - has:
      field: value
      pattern: $V
      kind: 'false'
message: 'Script can read this cookie; set `httpOnly: true` unless the page must read it'
fix:
- replace:
    target: $V
    with: 'true'
```

## Examples

### JavaScript

#### Don't

```js
res.cookie("session", id, { httpOnly: false });
```

#### Do

```js
res.cookie("session", id, { httpOnly: true });
```

### JSX

#### Don't

```jsx
res.cookie("session", id, { httpOnly: false });
```

#### Do

```jsx
res.cookie("session", id, { httpOnly: true });
```

### TypeScript

#### Don't

```ts
res.cookie("session", id, { httpOnly: false });
```

#### Do

```ts
res.cookie("session", id, { httpOnly: true });
```

### TSX

#### Don't

```tsx
res.cookie("session", id, { httpOnly: false });
```

#### Do

```tsx
res.cookie("session", id, { httpOnly: true });
```
