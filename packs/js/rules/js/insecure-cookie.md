---
id: js/insecure-cookie
title: "Cookies are secure"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a site served only over HTTP in development needs its own setting; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/insecure-cookie]
since: 0.5.0
---

A cookie options object does not set `secure: false`.

**Why:** A cookie without `Secure` is sent over unencrypted connections, where it can be read.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^(secure|"secure")$
  inside:
    kind: object
    has:
      kind: pair
      has:
        field: key
        regex: ^(httpOnly|sameSite|maxAge|expires|domain|path|signed)$
  all:
  - has:
      field: value
      pattern: $V
      kind: 'false'
message: 'This cookie is sent over plain HTTP; set `secure: true`'
fix:
- replace:
    target: $V
    with: 'true'
```

## Examples

### JavaScript

#### Don't

```js
res.cookie("id", id, { httpOnly: true, secure: false });
```

#### Do

```js
res.cookie("id", id, { httpOnly: true, secure: true });
```

### JSX

#### Don't

```jsx
res.cookie("id", id, { httpOnly: true, secure: false });
```

#### Do

```jsx
res.cookie("id", id, { httpOnly: true, secure: true });
```

### TypeScript

#### Don't

```ts
res.cookie("id", id, { httpOnly: true, secure: false });
```

#### Do

```ts
res.cookie("id", id, { httpOnly: true, secure: true });
```

### TSX

#### Don't

```tsx
res.cookie("id", id, { httpOnly: true, secure: false });
```

#### Do

```tsx
res.cookie("id", id, { httpOnly: true, secure: true });
```
