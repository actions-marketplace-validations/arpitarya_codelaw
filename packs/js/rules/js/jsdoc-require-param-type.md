---
id: js/jsdoc-require-param-type
title: "@param has a type"
languages: [javascript, jsx]
fix: manual
fix_reason: "the type is the author's to name; codelaw cannot infer it"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsdoc/require-param-type]
since: 0.5.0
---

In JavaScript, a JSDoc `@param` tag gives the parameter's `{type}`. TypeScript files are not checked: their types live in the code (the plugin's TypeScript preset turns this rule off too).

**Why:** The JSDoc type is the only type a JavaScript parameter has; editors and `checkJs` read it.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:param|arg|argument)(?:[ \t]+[^{\s]|[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/)))
message: Give this `@param` a `{type}`
```

## How to fix

1. Write the parameter's type in braces between `@param` and its name.

## Examples

### JavaScript

#### Don't

```js
/**
 * Greets someone.
 * @param name Who to greet.
 */
function greet(name) {}
```

#### Do

```js
/**
 * Greets someone.
 * @param {string} name Who to greet.
 */
function greet(name) {}
```

### JSX

#### Don't

```jsx
/**
 * Greets someone.
 * @param name Who to greet.
 */
function greet(name) {}
```

#### Do

```jsx
/**
 * Greets someone.
 * @param {string} name Who to greet.
 */
function greet(name) {}
```
