---
id: js/jsdoc-type-formatting
title: "One format for the types in JSDoc tags"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { enable_fixer: { type: "boolean", default: true, description: "offer the rewrite" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/type-formatting]
since: 0.5.0
---

The `{type}` of `@param`, `@property`, `@returns`, `@this`, `@throws`, `@type`, `@typedef` and `@yields` is written the plugin's default way: unions `A | B`, array generics as `T[]` (`Array<T>` is rewritten; other generics keep `Foo<T, U>`, with no dot), object types `{a: string, b: number}` with commas, no padding inside the braces, no trailing separator and unquoted keys that are plain names, strings in double quotes, function types `function(a, b): R` and `(a: A) => R`, tuples `[A, B]`, elements and parameters separated by `, `, and no space inside the `{}` that wrap the type. The fix rewrites the whole type in that format. A type written over several lines, or one that does not parse (that is `valid-types`'), is left alone; so are the constructs the rule does not model (conditional, indexed, template-literal and `import()` types, mapped types), which are kept as written. The plugin's formatting options are not built: this is its default configuration.

**Why:** One way of writing a type makes a codebase's docs easy to scan, and lets a person tell a real difference from a stylistic one.

```codelaw
native: js/jsdoc-type-formatting
```

## How to fix

1. Write the type as the rule prints it (the fix does).

## Examples

### JavaScript

#### Don't

```js
/**
 * @param {Array<string>|string} names
 * @param {{a: string; b: number}} opts
 * @returns {'ok'|'fail'}
 */
function f(names, opts) {}
```

#### Do

```js
/**
 * @param {string[] | string} names
 * @param {{a: string, b: number}} opts
 * @returns {"ok" | "fail"}
 */
function f(names, opts) {}
```

#### Fixed

```js
/**
 * @param {string[] | string} names
 * @param {{a: string, b: number}} opts
 * @returns {"ok" | "fail"}
 */
function f(names, opts) {}
```

### JSX

#### Don't

```jsx
/**
 * @param {Array<string>|string} names
 * @param {{a: string; b: number}} opts
 * @returns {'ok'|'fail'}
 */
function f(names, opts) {}
```

#### Do

```jsx
/**
 * @param {string[] | string} names
 * @param {{a: string, b: number}} opts
 * @returns {"ok" | "fail"}
 */
function f(names, opts) {}
```

#### Fixed

```jsx
/**
 * @param {string[] | string} names
 * @param {{a: string, b: number}} opts
 * @returns {"ok" | "fail"}
 */
function f(names, opts) {}
```

### TypeScript

#### Don't

```ts
/**
 * @param {Array<string>|string} names
 * @param {{a: string; b: number}} opts
 * @returns {'ok'|'fail'}
 */
function f(names, opts) {}
```

#### Do

```ts
/**
 * @param {string[] | string} names
 * @param {{a: string, b: number}} opts
 * @returns {"ok" | "fail"}
 */
function f(names, opts) {}
```

#### Fixed

```ts
/**
 * @param {string[] | string} names
 * @param {{a: string, b: number}} opts
 * @returns {"ok" | "fail"}
 */
function f(names, opts) {}
```

### TSX

#### Don't

```tsx
/**
 * @param {Array<string>|string} names
 * @param {{a: string; b: number}} opts
 * @returns {'ok'|'fail'}
 */
function f(names, opts) {}
```

#### Do

```tsx
/**
 * @param {string[] | string} names
 * @param {{a: string, b: number}} opts
 * @returns {"ok" | "fail"}
 */
function f(names, opts) {}
```

#### Fixed

```tsx
/**
 * @param {string[] | string} names
 * @param {{a: string, b: number}} opts
 * @returns {"ok" | "fail"}
 */
function f(names, opts) {}
```
