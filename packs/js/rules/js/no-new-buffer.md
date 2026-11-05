---
id: js/no-new-buffer
title: "Buffer.from rather than new Buffer"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-new-buffer]
since: 0.5.0
---

`new Buffer(x)` is written `Buffer.from(x)` (or `Buffer.alloc(n)` for a size).

**Why:** `new Buffer` is deprecated: given a number it returns uninitialised memory.

```codelaw
match:
  pattern: "new Buffer($X)"
constraints:
  X: { not: { kind: number } }
message: "`new Buffer()` is deprecated; use `Buffer.from()` or `Buffer.alloc()`"
fix: "Buffer.from($X)"
```

## Examples

### JavaScript

#### Don't

```js
const b = new Buffer("hi");
```

#### Do

```js
const b = Buffer.from("hi");
```

### JSX

#### Don't

```jsx
const b = new Buffer("hi");
```

#### Do

```jsx
const b = Buffer.from("hi");
```

### TypeScript

#### Don't

```ts
const b = new Buffer("hi");
```

#### Do

```ts
const b = Buffer.from("hi");
```

### TSX

#### Don't

```tsx
const b = new Buffer("hi");
```

#### Do

```tsx
const b = Buffer.from("hi");
```
