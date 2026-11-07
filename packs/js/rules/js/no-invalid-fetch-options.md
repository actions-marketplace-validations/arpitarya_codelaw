---
id: js/no-invalid-fetch-options
title: "No body on a GET or HEAD request"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the method or the body is wrong is the author's to say"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-fetch-options]
since: 0.5.0
---

`fetch()` and `new Request()` are not given a `body` with the `GET` or `HEAD` method, or with no method (which is `GET`): the call throws a `TypeError`.

**Why:** A GET or HEAD request cannot carry a body, so the request is never sent.

```codelaw
utils:
  body:
    any:
      - kind: pair
        has: { field: key, regex: '^["'']?body["'']?$' }
        not: { has: { field: value, regex: '^(null|undefined)$' } }
      - kind: shorthand_property_identifier
        regex: '^body$'
  method-key: { kind: pair, has: { field: key, regex: '^["'']?method["'']?$' } }
  bad:
    kind: object
    has: { matches: body }
    not: { has: { kind: spread_element } }
    any:
      - has: { matches: method-key, all: [{ has: { field: value, regex: '^["''`](?i:get|head)["''`]$' } }] }
      - not: { has: { matches: method-key } }
match:
  any:
    - pattern: "fetch($U, $O)"
    - pattern: "new Request($U, $O)"
constraints:
  O: { matches: bad }
message: "A GET or HEAD request can't have a body"
```

## How to fix

1. Send the body with a method that takes one (`POST`, `PUT`, `PATCH`), or move the data into the URL and drop the body.

## Examples

### JavaScript

#### Don't

```js
fetch("/api/search", { body: JSON.stringify(query) });
```

#### Do

```js
fetch("/api/search", { method: "POST", body: JSON.stringify(query) });
```


### JSX

#### Don't

```jsx
fetch("/api/search", { body: JSON.stringify(query) });
```

#### Do

```jsx
fetch("/api/search", { method: "POST", body: JSON.stringify(query) });
```


### TypeScript

#### Don't

```ts
fetch("/api/search", { body: JSON.stringify(query) });
```

#### Do

```ts
fetch("/api/search", { method: "POST", body: JSON.stringify(query) });
```


### TSX

#### Don't

```tsx
fetch("/api/search", { body: JSON.stringify(query) });
```

#### Do

```tsx
fetch("/api/search", { method: "POST", body: JSON.stringify(query) });
```
