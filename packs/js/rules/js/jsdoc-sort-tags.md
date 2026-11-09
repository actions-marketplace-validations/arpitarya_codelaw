---
id: js/jsdoc-sort-tags
title: "JSDoc tags in the prescribed order"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-jsdoc/sort-tags]
since: 0.5.0
---

The tags of a block follow the plugin's default sequence — `@summary`, `@module`, `@file`, `@import`, `@template`, `@typedef`, `@type`, `@callback`, `@class`, … `@param`, `@property`, `@returns`, `@async`, `@default`, `@enum`, `@augments`, `@throws`, `@yields`, `@event`, `@fires`, `@this`, `@static`, `@private`, `@public`, then every tag not in the list, then `@see`, `@example`, and the metadata tags (`@ignore`, `@author`, `@version`, `@since`, `@deprecated`, `@todo`). Tags of one name keep their order. The report is on the first tag that has to move; the fix rewrites the tags from there on, each with its own continuation lines. When the tags are in order, a blank line between two of them is reported. The `tagSequence`, `linesBetween`, `alphabetizeExtras` and `tagExceptions` options are not built. The fix is left off when the last tag shares its line with the closing `*/`, and when the block holds several `@typedef`, `@callback` or `@overload` tags (each owns the tags after it, so moving tags would change which belongs to which).

**Why:** A fixed order makes a block's `@param`s, return and notes easy to find, and keeps two authors' blocks the same shape.

```codelaw
native: js/jsdoc-sort-tags
```

## How to fix

1. Move the tags into the order above (the fix does).

## Examples

### JavaScript

#### Don't

```js
/**
 * Adds two numbers.
 * @returns {number}
 * @param {number} a
 * @param {number} b
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
 * @returns {number}
 * @param {number} a
 * @param {number} b
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
 * @returns {number}
 * @param {number} a
 * @param {number} b
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
 * @returns {number}
 * @param {number} a
 * @param {number} b
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
