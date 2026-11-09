---
id: js/jsdoc-check-template-names
title: "Every @template is used"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to use the name or drop the tag is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsdoc/check-template-names]
since: 0.5.0
---

Each name a `@template` declares is used somewhere the block documents: in the types of its `@param` and `@returns` tags (for a function, or a `@callback` or `@function` block), in the `@typedef`'s type and `@property` types, in the heritage tags (`@augments`, `@extends`, `@implements`) and the member blocks of a class, or in the constraint or default of another `@template`; a name the code declares itself (`class Box<T>`, `type Pair<A, B>`, `interface I<T>`) counts as used. The report is `@template T not in use`, on the tag. A block that is not on a function, class, interface or type alias, and a block with two `@typedef` tags, are not checked.

**Why:** An unused type parameter documents nothing, and usually means the `@param` that used it was renamed or removed.

```codelaw
native: js/jsdoc-check-template-names
```

## How to fix

1. Use the template name in a `@param` or `@returns` type, or remove the `@template`.

## Examples

### JavaScript

#### Don't

```js
/**
 * @template T
 * @template U
 * @param {T} item
 * @returns {T}
 */
function identity(item) {}
```

#### Do

```js
/**
 * @template T
 * @param {T} item
 * @returns {T}
 */
function identity(item) {}
```

### JSX

#### Don't

```jsx
/**
 * @template T
 * @template U
 * @param {T} item
 * @returns {T}
 */
function identity(item) {}
```

#### Do

```jsx
/**
 * @template T
 * @param {T} item
 * @returns {T}
 */
function identity(item) {}
```

### TypeScript

#### Don't

```ts
/**
 * @template T
 * @template U
 * @param {T} item
 * @returns {T}
 */
function identity(item) {}
```

#### Do

```ts
/**
 * @template T
 * @param {T} item
 * @returns {T}
 */
function identity(item) {}
```

### TSX

#### Don't

```tsx
/**
 * @template T
 * @template U
 * @param {T} item
 * @returns {T}
 */
function identity(item) {}
```

#### Do

```tsx
/**
 * @template T
 * @param {T} item
 * @returns {T}
 */
function identity(item) {}
```
