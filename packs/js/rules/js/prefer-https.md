---
id: js/prefer-https
title: "https:// rather than http:// for a public address"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the address now goes over TLS; a host that doesn't serve HTTPS stops answering, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-https]
since: 0.5.0
---

A string, template or comment holding an `http://` URL whose host has a public top-level domain uses `https://`. Local and private hosts (`localhost`, IP addresses, single-label names, `.test`, `.example`, `.invalid`, `.local`, `.internal`, `.localhost`) are left, and so are XML namespace identifiers (`http://www.w3.org/…`, `http://schemas.…`, `http://purl.org/…`, `http://ns.adobe.com/…`, `http://xmlns.com/…`), which are names, not addresses.

**Why:** Plain HTTP lets anyone on the path read and change the response.

```codelaw
match:
  any:
    - kind: string
    - kind: template_string
    - kind: comment
  regex: 'http://([A-Za-z0-9-]+\.)+[A-Za-z]{2,}([/:"''`\s)]|$)'
  not:
    any:
      - regex: 'http://(localhost|\d{1,3}(\.\d{1,3}){3}|\[)'
      - regex: 'http://([A-Za-z0-9-]+\.)*[A-Za-z0-9-]+\.(test|example|invalid|local|internal|localhost)([/:"''`\s)]|$)'
      - regex: 'http://(www\.w3\.org|schemas\.|purl\.org|ns\.adobe\.com|xmlns\.com|www\.example\.)'
message: "Use `https://` for this address"
fix:
  - edit-text: { pattern: 'http://((?:[A-Za-z0-9-]+\.)+[A-Za-z]{2,})', replace: 'https://${1}' }
```

## Examples

### JavaScript

#### Don't

```js
const api = "http://api.github.com/repos";
```

#### Do

```js
const api = "https://api.github.com/repos";
```


### JSX

#### Don't

```jsx
const api = "http://api.github.com/repos";
```

#### Do

```jsx
const api = "https://api.github.com/repos";
```


### TypeScript

#### Don't

```ts
const api = "http://api.github.com/repos";
```

#### Do

```ts
const api = "https://api.github.com/repos";
```


### TSX

#### Don't

```tsx
const api = "http://api.github.com/repos";
```

#### Do

```tsx
const api = "https://api.github.com/repos";
```
