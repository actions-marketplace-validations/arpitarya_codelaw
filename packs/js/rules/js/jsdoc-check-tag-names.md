---
id: js/jsdoc-check-tag-names
title: "Known JSDoc tag names, written the preferred way"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { defined_tags: { type: "string[]", default: [], description: "tags the project uses besides the built-in ones" }, inline_tags: { type: "string[]", default: [], description: "inline tags allowed besides the built-in ones" }, jsx_tags: { type: "boolean", default: false, description: "allow @jsx, @jsxFrag, @jsxImportSource and @jsxRuntime" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/check-tag-names]
since: 0.5.0
---

Every block tag is one the plugin's default (TypeScript) mode knows — JSDoc's, TSDoc's, TypeDoc's and TypeScript's own (`@import`, `@overload`, `@satisfies`, `@template`) — and an alias is written as its preferred tag: `@return` is `@returns`, `@arg` and `@argument` are `@param`, `@prop` is `@property`, `@extends` is `@augments`, `@const` is `@constant`, `@func` and `@method` are `@function`, `@desc` is `@description`, `@exception` is `@throws`, `@yield` is `@yields`, `@virtual` is `@abstract`. An inline tag (`{@link …}`) is one of `link`, `linkcode`, `linkplain`, `tutorial`, `inheritDoc`, `label`, `include`, `includeCode`. A tag inside a fenced code block is not read. The upstream `typed` option and its `tagNamePreference` setting are not read: the preferred names are the plugin's defaults, and `defined_tags` adds tags the project uses.

**Why:** A misspelt tag is silently dropped by documentation tools, and two spellings of one tag make a codebase's docs hard to search.

```codelaw
native: js/jsdoc-check-tag-names
```

## How to fix

1. Write the alias as its preferred tag (`@returns`, `@param`, `@property`).
2. Fix a misspelt tag, or list a tag the project uses on purpose in the rule's `defined_tags` option.

## Examples

### JavaScript

#### Don't

```js
/**
 * Gets the name.
 * @return {string} the name
 */
function getName() {}
```

#### Do

```js
/**
 * Gets the name.
 * @returns {string} the name
 */
function getName() {}
```

#### Fixed

```js
/**
 * Gets the name.
 * @returns {string} the name
 */
function getName() {}
```

### JSX

#### Don't

```jsx
/**
 * Gets the name.
 * @return {string} the name
 */
function getName() {}
```

#### Do

```jsx
/**
 * Gets the name.
 * @returns {string} the name
 */
function getName() {}
```

#### Fixed

```jsx
/**
 * Gets the name.
 * @returns {string} the name
 */
function getName() {}
```

### TypeScript

#### Don't

```ts
/**
 * Gets the name.
 * @return {string} the name
 */
function getName() {}
```

#### Do

```ts
/**
 * Gets the name.
 * @returns {string} the name
 */
function getName() {}
```

#### Fixed

```ts
/**
 * Gets the name.
 * @returns {string} the name
 */
function getName() {}
```

### TSX

#### Don't

```tsx
/**
 * Gets the name.
 * @return {string} the name
 */
function getName() {}
```

#### Do

```tsx
/**
 * Gets the name.
 * @returns {string} the name
 */
function getName() {}
```

#### Fixed

```tsx
/**
 * Gets the name.
 * @returns {string} the name
 */
function getName() {}
```
