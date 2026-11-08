---
id: js/prefer-url-search-parameters
title: "URLSearchParams rather than splitting a query on & and ="
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`URLSearchParams` decodes percent escapes and `+`, and keeps an `=` inside a value, where the split does neither; the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-url-search-parameters]
since: 0.5.0
---

`query.split('&').map(part => part.split('='))` parses a query string by hand, and gets decoding, `+` and values containing `=` wrong. Passed to `Object.fromEntries`, `new Map` or `new URLSearchParams`, the pipeline is `new URLSearchParams(query)`.

**Why:** The built-in parser already handles the cases the split misses.

```codelaw
utils:
  pipeline:
    any:
      - pattern: "$Q.split('&').map(($P) => $P.split('='))"
      - pattern: "$Q.split(\"&\").map(($P) => $P.split(\"=\"))"
      - pattern: "$Q.split('&').map($P => $P.split('='))"
      - pattern: "$Q.split(\"&\").map($P => $P.split(\"=\"))"
match:
  kind: call_expression
  matches: pipeline
  pattern: $PIPE
  inside:
    kind: arguments
    inside:
      any:
        - kind: call_expression
          pattern: "Object.fromEntries($$$)"
        - kind: new_expression
          has: { field: constructor, regex: '^(Map|URLSearchParams)$' }
message: "Parse the query with `URLSearchParams`"
fix:
  - replace: { target: $PIPE, with: "new URLSearchParams($Q)" }
```

## Examples

### JavaScript

#### Don't

```js
const params = Object.fromEntries(query.split("&").map((part) => part.split("=")));
```

#### Do

```js
const params = Object.fromEntries(new URLSearchParams(query));
```


### JSX

#### Don't

```jsx
const params = Object.fromEntries(query.split("&").map((part) => part.split("=")));
```

#### Do

```jsx
const params = Object.fromEntries(new URLSearchParams(query));
```


### TypeScript

#### Don't

```ts
const params = Object.fromEntries(query.split("&").map((part) => part.split("=")));
```

#### Do

```ts
const params = Object.fromEntries(new URLSearchParams(query));
```


### TSX

#### Don't

```tsx
const params = Object.fromEntries(query.split("&").map((part) => part.split("=")));
```

#### Do

```tsx
const params = Object.fromEntries(new URLSearchParams(query));
```
