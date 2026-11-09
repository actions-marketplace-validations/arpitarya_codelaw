---
id: js/jsdoc-check-param-names
title: "JSDoc @param names match the function parameters"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a renamed or removed tag changes what the documentation says, so the author checks it against the code"
options: { allow_extra_trailing_param_docs: { type: "boolean", default: false, description: "allow @param tags after the last parameter" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/check-param-names]
since: 0.5.0
---

The `@param` tags of a block name the function's parameters, in order: a tag with no matching parameter is reported (`@param "x" does not match an existing function parameter.`), a name that differs from the parameter in its place is reported with the names expected, a name written twice is reported, and a path (`options.limit`, `items[].id`) comes after its root. A destructured parameter takes any root name, and the properties written under it are not compared with the pattern; a TypeScript `this` parameter is skipped unless it is documented. The finding stops at the first mismatch of a block, as the plugin's does. The block must sit right before a function, method, function-valued variable, property or field, or a TypeScript signature. The plugin's options (`checkDestructured`, `checkRestProperty`, `useDefaultObjectProperties`, `disableMissingParamChecks`, …) are not built; `allow_extra_trailing_param_docs` is.

**Why:** A `@param` that names no parameter is stale documentation, usually left from a rename, and tools that read the block show the wrong signature.

```codelaw
native: js/jsdoc-check-param-names
```

## How to fix

1. Rename the tag to the parameter's name (the suggestion does), or remove a tag for a parameter that no longer exists.

## Examples

### JavaScript

#### Don't

```js
/**
 * @param {string} nme
 */
function greet(name) {}
```

#### Do

```js
/**
 * @param {string} name
 */
function greet(name) {}
```

### JSX

#### Don't

```jsx
/**
 * @param {string} nme
 */
function greet(name) {}
```

#### Do

```jsx
/**
 * @param {string} name
 */
function greet(name) {}
```

### TypeScript

#### Don't

```ts
/**
 * @param {string} nme
 */
function greet(name) {}
```

#### Do

```ts
/**
 * @param {string} name
 */
function greet(name) {}
```

### TSX

#### Don't

```tsx
/**
 * @param {string} nme
 */
function greet(name) {}
```

#### Do

```tsx
/**
 * @param {string} name
 */
function greet(name) {}
```
