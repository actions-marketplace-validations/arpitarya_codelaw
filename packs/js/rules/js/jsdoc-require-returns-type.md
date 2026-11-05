---
id: js/jsdoc-require-returns-type
title: "@returns has a type"
languages: [javascript, jsx]
fix: manual
fix_reason: "the type is the author's to name; codelaw cannot infer it"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsdoc/require-returns-type]
since: 0.5.0
---

In JavaScript, a JSDoc `@returns` tag gives the `{type}` of the return value. TypeScript files are not checked (the plugin's TypeScript preset turns this rule off).

**Why:** In JavaScript the JSDoc type is the only declared return type; editors and `checkJs` read it.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@returns?(?:[ \t]+[^{\s]|[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/)))
message: Give the `@returns` tag a `{type}`
```

## How to fix

1. Write the return type in braces right after `@returns`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns The sum.
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```js
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```

### JSX

#### Don't

```jsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns The sum.
 */
function add(a, b) {
  return a + b;
}
```

#### Do

```jsx
/**
 * Adds two numbers.
 * @param {number} a The first.
 * @param {number} b The second.
 * @returns {number} The sum.
 */
function add(a, b) {
  return a + b;
}
```
