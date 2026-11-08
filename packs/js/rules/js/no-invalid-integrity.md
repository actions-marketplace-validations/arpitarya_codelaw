---
id: js/no-invalid-integrity
title: "No malformed `integrity` hash on a `<script>` or `<link>`"
languages: [jsx, tsx]
fix: manual
fix_reason: "the right hash comes from the resource itself"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-integrity]
since: 0.5.0
---

In JSX, the `integrity` of a `<script>` or `<link>` holds hashes the browser can check: `sha256-`, `sha384-`, `sha512-` (or `ed25519-`) followed by a base64 value of exactly the algorithm's length, optionally `?options`. A recognised algorithm with a malformed or wrong-length value, and a value naming no known algorithm at all, are reported. Upstream checks HTML files too; that is the `html` pack's.

**Why:** The browser refuses a resource whose integrity it can't parse or match, so the page breaks.

```codelaw
native: js/no-invalid-integrity
```

## How to fix

1. Generate the hash from the exact file (`openssl dgst -sha384 -binary file | openssl base64 -A`).

## Examples

### JSX

#### Don't

```jsx
const tag = <link rel="stylesheet" href="a.css" integrity="sha512-short" />;
```

#### Do

```jsx
const tag = <link rel="stylesheet" href="a.css" integrity="sha256-47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=" />;
```


### TSX

#### Don't

```tsx
const tag = <link rel="stylesheet" href="a.css" integrity="sha512-short" />;
```

#### Do

```tsx
const tag = <link rel="stylesheet" href="a.css" integrity="sha256-47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=" />;
```
