---
id: js/no-invalid-url-protocol-comparison
title: "url.protocol ends with a colon"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the comparison changes from never true to sometimes true"
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-url-protocol-comparison]
since: 0.5.0
---

A URL's `protocol` is compared with a scheme ending in `:` (`"https:"`).

**Why:** `URL#protocol` and `location.protocol` include the trailing colon, so `"https"` never matches.

```codelaw
match:
  kind: binary_expression
  has:
    field: operator
    regex: ^(===|!==|==|!=)$
  all:
  - has:
      kind: member_expression
      has:
        field: property
        regex: ^protocol$
  - has:
      kind: string
      has:
        kind: string_fragment
        regex: ^[a-z][a-z0-9+.-]*$
        pattern: $P
message: '`protocol` includes the colon: compare with `"$P:"`'
fix:
- insert-after:
    target: $P
    text: ':'
```

## Examples

### JavaScript

#### Don't

```js
if (url.protocol === "https") {
  secure();
}
```

#### Do

```js
if (url.protocol === "https:") {
  secure();
}
```

### JSX

#### Don't

```jsx
if (url.protocol === "https") {
  secure();
}
```

#### Do

```jsx
if (url.protocol === "https:") {
  secure();
}
```

### TypeScript

#### Don't

```ts
if (url.protocol === "https") {
  secure();
}
```

#### Do

```ts
if (url.protocol === "https:") {
  secure();
}
```

### TSX

#### Don't

```tsx
if (url.protocol === "https") {
  secure();
}
```

#### Do

```tsx
if (url.protocol === "https:") {
  secure();
}
```
