---
id: js/prefer-import-meta-properties
title: "import.meta.filename rather than fileURLToPath(import.meta.url)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`import.meta.filename` needs Node 20.11 or later"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-import-meta-properties]
since: 0.5.0
---

`fileURLToPath(import.meta.url)` is written `import.meta.filename` (and `path.dirname` of it `import.meta.dirname`).

**Why:** The properties exist in Node 20.11 and later and need no import.

```codelaw
match:
  pattern: "fileURLToPath(import.meta.url)"
message: "Use `import.meta.filename`"
fix: "import.meta.filename"
```

## Examples

### JavaScript

#### Don't

```js
const file = fileURLToPath(import.meta.url);
```

#### Do

```js
const file = import.meta.filename;
```

### JSX

#### Don't

```jsx
const file = fileURLToPath(import.meta.url);
```

#### Do

```jsx
const file = import.meta.filename;
```

### TypeScript

#### Don't

```ts
const file = fileURLToPath(import.meta.url);
```

#### Do

```ts
const file = import.meta.filename;
```

### TSX

#### Don't

```tsx
const file = fileURLToPath(import.meta.url);
```

#### Do

```tsx
const file = import.meta.filename;
```
