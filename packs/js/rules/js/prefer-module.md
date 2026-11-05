---
id: js/prefer-module
title: "ES modules rather than CommonJS"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the file's importers and its package type change"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-module]
since: 0.5.0
---

A file does not assign to `module.exports` or `exports`, and does not read `__dirname` or `__filename`. Upstream also reports `require()`; this port leaves `require` to `node/global-require` and its kin.

**Why:** ES modules are the standard; CommonJS needs a build step for browsers and interoperates awkwardly.

```codelaw

match:
  any:
    - { pattern: module.exports = $X }
    - { kind: assignment_expression, has: { field: left, kind: member_expression, has: { field: object, regex: '^(module\.)?exports$' } } }
    - { kind: identifier, regex: '^(__dirname|__filename)$' }
message: "Use ES module syntax (`export`, `import.meta.dirname`) rather than CommonJS"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
module.exports = { run };
```

#### Do

```js
export { run };
```

### JSX

#### Don't

```jsx
module.exports = { run };
```

#### Do

```jsx
export { run };
```

### TypeScript

#### Don't

```ts
module.exports = { run };
```

#### Do

```ts
export { run };
```

### TSX

#### Don't

```tsx
module.exports = { run };
```

#### Do

```tsx
export { run };
```
