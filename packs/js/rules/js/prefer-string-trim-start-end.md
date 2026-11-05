---
id: js/prefer-string-trim-start-end
title: "trimStart() rather than trimLeft()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-string-trim-start-end]
since: 0.5.0
---

`trimLeft` and `trimRight` are written `trimStart` and `trimEnd`.

**Why:** The `Start`/`End` names are the standard ones and match `padStart`/`padEnd`.

```codelaw
match:
  pattern: "$S.trimLeft()"
message: "Use `.trimStart()`"
fix: "$S.trimStart()"
```

## Examples

### JavaScript

#### Don't

```js
const s = line.trimLeft();
```

#### Do

```js
const s = line.trimStart();
```

### JSX

#### Don't

```jsx
const s = line.trimLeft();
```

#### Do

```jsx
const s = line.trimStart();
```

### TypeScript

#### Don't

```ts
const s = line.trimLeft();
```

#### Do

```ts
const s = line.trimStart();
```

### TSX

#### Don't

```tsx
const s = line.trimLeft();
```

#### Do

```tsx
const s = line.trimStart();
```
