---
id: js/no-url-in-search-params
title: "No full URL passed to URLSearchParams"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the result is no longer the same object: `new URL(…).searchParams` is live, a copy from `url.search` is not, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-url-in-search-params]
since: 0.5.0
---

`URLSearchParams` parses a query string. Given a whole URL it makes the scheme, host and path part of the first parameter's name. A literal URL string becomes `new URL(string).searchParams`, and a `.href` becomes the URL's `.search`. Upstream also follows variables known to hold a `URL`; that needs data-flow, and is not reported here.

**Why:** The first parameter is silently lost under a name nobody asks for.

```codelaw
match:
  kind: new_expression
  any:
    - all:
        - pattern: "new URLSearchParams($S)"
        - has: { field: arguments, has: { nthChild: 1, kind: string, regex: '^["''][a-zA-Z][a-zA-Z0-9+.-]*://' } }
        - pattern: $W1
    - all:
        - pattern: "new URLSearchParams($U.href)"
        - has: { field: arguments, has: { nthChild: 1, has: { field: property, pattern: $HREF } } }
message: "`URLSearchParams` takes the query, not the whole URL"
fix:
  - replace: { target: $W1, with: "new URL($S).searchParams" }
  - replace: { target: $HREF, with: search }
```

## Examples

### JavaScript

#### Don't

```js
const params = new URLSearchParams("https://example.com/?q=1");
```

#### Do

```js
const params = new URL("https://example.com/?q=1").searchParams;
```


### JSX

#### Don't

```jsx
const params = new URLSearchParams("https://example.com/?q=1");
```

#### Do

```jsx
const params = new URL("https://example.com/?q=1").searchParams;
```


### TypeScript

#### Don't

```ts
const params = new URLSearchParams("https://example.com/?q=1");
```

#### Do

```ts
const params = new URL("https://example.com/?q=1").searchParams;
```


### TSX

#### Don't

```tsx
const params = new URLSearchParams("https://example.com/?q=1");
```

#### Do

```tsx
const params = new URL("https://example.com/?q=1").searchParams;
```
