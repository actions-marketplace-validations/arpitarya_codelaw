---
id: js/require-array-join-separator
title: "join() names its separator"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/require-array-join-separator]
since: 0.5.0
---

`.join()` is called with its separator.

**Why:** The default comma is easy to forget and looks like a mistake.

```codelaw
match:
  pattern: "$A.join()"
message: "Pass the separator to `.join()`"
fix: "$A.join(\",\")"
```

## Examples

### JavaScript

#### Don't

```js
const csv = cells.join();
```

#### Do

```js
const csv = cells.join(",");
```

### JSX

#### Don't

```jsx
const csv = cells.join();
```

#### Do

```jsx
const csv = cells.join(",");
```

### TypeScript

#### Don't

```ts
const csv = cells.join();
```

#### Do

```ts
const csv = cells.join(",");
```

### TSX

#### Don't

```tsx
const csv = cells.join();
```

#### Do

```tsx
const csv = cells.join(",");
```
