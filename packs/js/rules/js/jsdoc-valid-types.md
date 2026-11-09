---
id: js/jsdoc-valid-types
title: "Valid types, names and namepaths in JSDoc tags"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the right type, name or namepath is the author's"
fix_exception: value
options: { allow_empty_namepaths: { type: "boolean", default: false, description: "do not require a name or type on the tags that need one of them" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/valid-types]
since: 0.5.0
---

A tag's `{type}` parses as a TypeScript, JSDoc or Closure type; a namepath is well formed; a tag that takes a name has one (`@callback`, `@event`, `@external`, `@fires`, `@name`, …), a tag that takes a type has one (`@type`, `@this`, `@satisfies`, `@implements`), and a tag that takes neither does not carry it (`@internal` and `@this` have no name; `@function`, `@mixin`, `@private`, `@static` have no `{type}`). `@borrows` needs an `as` expression; an inline `{@link}`, `{@label}`, `{@include}` or `{@inheritDoc}` has content. A nullable `?` after an object field's type (`{a: string?}`) is rejected in TypeScript mode. `@param`, `@property` and their names are `require-param-name`'s and `require-property-name`'s to demand. Names are checked in a simplified grammar: an identifier-like segment, `.`, `#`, `~` or `/` between segments, `module:`, `external:` and `event:` prefixes, quoted segments, and `[]`; the tag lists are the plugin's TypeScript-mode defaults, and the "in another mode" part of its messages is left out.

**Why:** A type that does not parse is ignored or mis-read by every tool that reads the comment, and a mistyped tag silently documents nothing.

```codelaw
native: js/jsdoc-valid-types
```

## How to fix

1. Correct the type or the name (for a type, check the brackets and `|` separators).
2. Add the missing name or type, or remove the one the tag does not take.

## Examples

### JavaScript

#### Don't

```js
/**
 * @param {Array<string} names
 * @returns {string[}
 */
function join(names) {}
```

#### Do

```js
/**
 * @param {Array<string>} names
 * @returns {string[]}
 */
function join(names) {}
```

### JSX

#### Don't

```jsx
/**
 * @param {Array<string} names
 * @returns {string[}
 */
function join(names) {}
```

#### Do

```jsx
/**
 * @param {Array<string>} names
 * @returns {string[]}
 */
function join(names) {}
```

### TypeScript

#### Don't

```ts
/**
 * @param {Array<string} names
 * @returns {string[}
 */
function join(names) {}
```

#### Do

```ts
/**
 * @param {Array<string>} names
 * @returns {string[]}
 */
function join(names) {}
```

### TSX

#### Don't

```tsx
/**
 * @param {Array<string} names
 * @returns {string[}
 */
function join(names) {}
```

#### Do

```tsx
/**
 * @param {Array<string>} names
 * @returns {string[]}
 */
function join(names) {}
```
