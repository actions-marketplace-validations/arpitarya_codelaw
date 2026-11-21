---
id: js/jsdoc-check-examples
title: "The code in an `@example` is code"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the example should say is the author's"
fix_exception: value
stability: preview
options:
  ignore: { type: "string[]", default: [], description: "regular expressions; an example whose code one matches is not checked (pseudo-code, a `...` for what is left out)" }
ported_from: [eslint-plugin-jsdoc/check-examples]
since: 0.5.2
---

Each `@example` of a `/** … */` comment holds code that parses in the file's language. The example is the lines after the tag, up to the next tag; what follows `@example` on its own line is code, unless it is a `<caption>`; a fenced block takes the fence's language (`json`, `ts`) and leaves the prose around it. An example that does not parse is reported on its first line of code: `add(1,` or a `...` standing for what is left out. A fragment is code only if it parses on its own, so an object literal needs parentheses or a statement around it. A project lists the examples it leaves alone in `ignore`.

eslint-plugin-jsdoc's `check-examples` runs each example through ESLint and reports what the project's rules find, and its own README says it does not work on ESLint 8 and later. Here the two halves are apart: this rule reports the example that is not code, and the engine lints the ones that are, with the file's own rules, when `[lint] jsdoc_examples = true` (SR-LINTER decision 3). An example that does not parse is not linted by anything else.

**Why:** An example that does not parse is wrong the day a reader copies it, and it stays wrong because nothing runs it.

```codelaw
native: js/jsdoc-check-examples
```

## How to fix

1. Make the example parse: close the brackets, and write out what `...` stood for.
2. If it is pseudo-code on purpose, add a pattern for it to `ignore`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Adds two numbers.
 * @example
 * add(1,
 */
export function add(a, b) {
  return a + b;
}
```

#### Do

```js
/**
 * Adds two numbers.
 * @example
 * add(1, 2);
 */
export function add(a, b) {
  return a + b;
}
```

### JSX

#### Don't

```jsx
/**
 * Adds two numbers.
 * @example
 * add(1,
 */
export function add(a, b) {
  return a + b;
}
```

#### Do

```jsx
/**
 * Adds two numbers.
 * @example
 * add(1, 2);
 */
export function add(a, b) {
  return a + b;
}
```

### TypeScript

#### Don't

```ts
/**
 * Adds two numbers.
 * @example
 * const total: = add(1, 2);
 */
export function add(a: number, b: number): number {
  return a + b;
}
```

#### Do

```ts
/**
 * Adds two numbers.
 * @example
 * const total: number = add(1, 2);
 */
export function add(a: number, b: number): number {
  return a + b;
}
```

### TSX

#### Don't

```tsx
/**
 * Adds two numbers.
 * @example
 * const total: = add(1, 2);
 */
export function add(a: number, b: number): number {
  return a + b;
}
```

#### Do

```tsx
/**
 * Adds two numbers.
 * @example
 * const total: number = add(1, 2);
 */
export function add(a: number, b: number): number {
  return a + b;
}
```
