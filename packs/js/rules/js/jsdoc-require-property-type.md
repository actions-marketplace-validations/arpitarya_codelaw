---
id: js/jsdoc-require-property-type
title: "@property has a type"
languages: [javascript, jsx]
fix: manual
fix_reason: "the type is the author's to name; codelaw cannot infer it"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-property-type]
since: 0.5.0
---

In JavaScript, a JSDoc `@property` tag gives the property's `{type}`. TypeScript files are not checked (the plugin's TypeScript preset turns this rule off).

**Why:** The JSDoc type is the only type the property has in JavaScript; editors and `checkJs` read it.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:property|prop)(?:[ \t]+[^{\s]|[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/)))
message: Give this `@property` a `{type}`
```

## How to fix

1. Write the property's type in braces between `@property` and its name.

## Examples

### JavaScript

#### Don't

```js
/**
 * @typedef {object} Point
 * @property x Across.
 */
```

#### Do

```js
/**
 * @typedef {object} Point
 * @property {number} x Across.
 */
```

### JSX

#### Don't

```jsx
/**
 * @typedef {object} Point
 * @property x Across.
 */
```

#### Do

```jsx
/**
 * @typedef {object} Point
 * @property {number} x Across.
 */
```
