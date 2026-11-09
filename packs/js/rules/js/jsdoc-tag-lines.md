---
id: js/jsdoc-tag-lines
title: "No blank lines between a JSDoc description and its tags, or between tags"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-jsdoc/tag-lines]
since: 0.5.0
---

A JSDoc block has no blank line between its description and the first tag, between two tags, or after the last tag (the plugin's default: `"never"`, `startLines: 0`, `endLines: 0`). A blank line inside a tag's own description (an `@example` with a gap in it) is part of that description and is not reported. A block with no tags may have blank lines in its description. The `"always"` mode and the `count`, `startLines`, `endLines`, `maxBlockLines` and per-tag options are not built.

**Why:** A blank line splits a tag from the description that introduces it in some tools' output, and the layout is one a formatter can hold the whole codebase to.

```codelaw
native: js/jsdoc-tag-lines
```

## How to fix

1. Delete the blank line (the fix does).

## Examples

### JavaScript

#### Don't

```js
/**
 * Adds two numbers.
 *
 * @param {number} a
 *
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

#### Do

```js
/**
 * Adds two numbers.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

#### Fixed

```js
/**
 * Adds two numbers.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

### JSX

#### Don't

```jsx
/**
 * Adds two numbers.
 *
 * @param {number} a
 *
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

#### Do

```jsx
/**
 * Adds two numbers.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

#### Fixed

```jsx
/**
 * Adds two numbers.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

### TypeScript

#### Don't

```ts
/**
 * Adds two numbers.
 *
 * @param {number} a
 *
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

#### Do

```ts
/**
 * Adds two numbers.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

#### Fixed

```ts
/**
 * Adds two numbers.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

### TSX

#### Don't

```tsx
/**
 * Adds two numbers.
 *
 * @param {number} a
 *
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

#### Do

```tsx
/**
 * Adds two numbers.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```

#### Fixed

```tsx
/**
 * Adds two numbers.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
function add(a, b) {}
```
