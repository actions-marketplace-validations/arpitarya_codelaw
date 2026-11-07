---
id: js/file-uploads
title: "File uploads have size limits"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the right limits are the owner's"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/file-uploads]
since: 0.5.0
---

`multer` is configured with `limits` (a `fileSize` among them).

**Why:** Without a limit a client can upload files until the disk or memory runs out.

```codelaw
match:
  pattern: "multer($O)"
  not:
    has:
      field: arguments
      any:
        - has: { kind: object, has: { kind: pair, has: { field: key, regex: '^["'']?limits["'']?$' } } }
        - has: { kind: identifier }
        - has: { kind: object, has: { kind: spread_element } }
message: "Set `limits` on this upload handler"
```

## How to fix

1. Pass `limits: { fileSize: … }` (and `files`, `fields` where it matters) sized for what the endpoint accepts.

## Examples

### JavaScript

#### Don't

```js
const upload = multer({ dest: "uploads/" });
```

#### Do

```js
const upload = multer({ dest: "uploads/", limits: { fileSize: 5 * 1024 * 1024 } });
```


### JSX

#### Don't

```jsx
const upload = multer({ dest: "uploads/" });
```

#### Do

```jsx
const upload = multer({ dest: "uploads/", limits: { fileSize: 5 * 1024 * 1024 } });
```


### TypeScript

#### Don't

```ts
const upload = multer({ dest: "uploads/" });
```

#### Do

```ts
const upload = multer({ dest: "uploads/", limits: { fileSize: 5 * 1024 * 1024 } });
```


### TSX

#### Don't

```tsx
const upload = multer({ dest: "uploads/" });
```

#### Do

```tsx
const upload = multer({ dest: "uploads/", limits: { fileSize: 5 * 1024 * 1024 } });
```
