---
id: js/jsdoc-no-blank-blocks
title: "No empty JSDoc blocks"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-jsdoc/no-blank-blocks]
since: 0.5.0
---

A JSDoc block with nothing but whitespace and asterisks is removed.

**Why:** An empty block documents nothing, yet tells tools and readers that the code below is documented.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '^/\*\*[\s*]*\*/$'
message: "This JSDoc block is empty; remove it or write the documentation"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
/**
 *
 */
function run() {}
```

#### Do

```js
function run() {}
```

### JSX

#### Don't

```jsx
/**
 *
 */
function run() {}
```

#### Do

```jsx
function run() {}
```

### TypeScript

#### Don't

```ts
/**
 *
 */
function run() {}
```

#### Do

```ts
function run() {}
```

### TSX

#### Don't

```tsx
/**
 *
 */
function run() {}
```

#### Do

```tsx
function run() {}
```
