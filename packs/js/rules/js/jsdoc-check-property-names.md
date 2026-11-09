---
id: js/jsdoc-check-property-names
title: "JSDoc @property names are not repeated and nested ones have a root"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing a repeated tag drops its description, so the author decides which of the two to keep"
stability: preview
ported_from: [eslint-plugin-jsdoc/check-property-names]
since: 0.5.0
---

A block does not give the same `@property` name twice (within one `@typedef`, when the block has several), and a path (`config.port`, `items[].id`) comes after a `@property` that is its root. Only `@property` is read — its alias `@prop` is `check-tag-names`' to rewrite. The first problem of a block is reported, as the plugin does; the suggestion removes a duplicate tag, which drops its description.

**Why:** A repeated property is documented twice and read as two different ones; a nested one with no root has nothing to hang from.

```codelaw
native: js/jsdoc-check-property-names
```

## How to fix

1. Remove the repeated `@property` (the suggestion does), or rename it.
2. Put the root `@property` before the ones nested under it.

## Examples

### JavaScript

#### Don't

```js
/**
 * @typedef {object} Options
 * @property {string} host
 * @property {number} host
 */
```

#### Do

```js
/**
 * @typedef {object} Options
 * @property {string} host
 * @property {number} port
 */
```

#### Fixed

```js
/**
 * @typedef {object} Options
 * @property {string} host
 */
```

### JSX

#### Don't

```jsx
/**
 * @typedef {object} Options
 * @property {string} host
 * @property {number} host
 */
```

#### Do

```jsx
/**
 * @typedef {object} Options
 * @property {string} host
 * @property {number} port
 */
```

#### Fixed

```jsx
/**
 * @typedef {object} Options
 * @property {string} host
 */
```

### TypeScript

#### Don't

```ts
/**
 * @typedef {object} Options
 * @property {string} host
 * @property {number} host
 */
```

#### Do

```ts
/**
 * @typedef {object} Options
 * @property {string} host
 * @property {number} port
 */
```

#### Fixed

```ts
/**
 * @typedef {object} Options
 * @property {string} host
 */
```

### TSX

#### Don't

```tsx
/**
 * @typedef {object} Options
 * @property {string} host
 * @property {number} host
 */
```

#### Do

```tsx
/**
 * @typedef {object} Options
 * @property {string} host
 * @property {number} port
 */
```

#### Fixed

```tsx
/**
 * @typedef {object} Options
 * @property {string} host
 */
```
