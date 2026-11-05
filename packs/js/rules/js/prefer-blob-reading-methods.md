---
id: js/prefer-blob-reading-methods
title: "blob.text() rather than FileReader"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the event handlers become an `await`"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-blob-reading-methods]
since: 0.5.0
---

`FileReader#readAsText` and `#readAsArrayBuffer` are not used.

**Why:** The `Blob` methods return a promise directly, without the event dance.

```codelaw

match:
  kind: call_expression
  has: { field: function, kind: member_expression, has: { field: property, regex: '^(readAsText|readAsArrayBuffer)$' } }
message: "Use `await blob.text()` or `await blob.arrayBuffer()`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
reader.readAsText(file);
```

#### Do

```js
const text = await file.text();
```

### JSX

#### Don't

```jsx
reader.readAsText(file);
```

#### Do

```jsx
const text = await file.text();
```

### TypeScript

#### Don't

```ts
reader.readAsText(file);
```

#### Do

```ts
const text = await file.text();
```

### TSX

#### Don't

```tsx
reader.readAsText(file);
```

#### Do

```tsx
const text = await file.text();
```
