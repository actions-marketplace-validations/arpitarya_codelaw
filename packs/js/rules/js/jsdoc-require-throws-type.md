---
id: js/jsdoc-require-throws-type
title: "@throws has a type"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the type is the author's to name; codelaw cannot infer it"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-throws-type]
since: 0.5.0
---

A JSDoc `@throws` tag gives the `{type}` of the error thrown.

**Why:** The error type is what callers branch on in `catch`; a description alone cannot be checked.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:throws|exception)(?:[ \t]+[^{\s]|[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/)))
message: Give the `@throws` tag a `{type}`
```

## How to fix

1. Write the error's type in braces right after `@throws`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Parses a port.
 * @param {string} s The text.
 * @throws When the text is not a number.
 */
function port(s) {}
```

#### Do

```js
/**
 * Parses a port.
 * @param {string} s The text.
 * @throws {RangeError} When the text is not a number.
 */
function port(s) {}
```

### JSX

#### Don't

```jsx
/**
 * Parses a port.
 * @param {string} s The text.
 * @throws When the text is not a number.
 */
function port(s) {}
```

#### Do

```jsx
/**
 * Parses a port.
 * @param {string} s The text.
 * @throws {RangeError} When the text is not a number.
 */
function port(s) {}
```

### TypeScript

#### Don't

```ts
/**
 * Parses a port.
 * @param {string} s The text.
 * @throws When the text is not a number.
 */
function port(s) {}
```

#### Do

```ts
/**
 * Parses a port.
 * @param {string} s The text.
 * @throws {RangeError} When the text is not a number.
 */
function port(s) {}
```

### TSX

#### Don't

```tsx
/**
 * Parses a port.
 * @param {string} s The text.
 * @throws When the text is not a number.
 */
function port(s) {}
```

#### Do

```tsx
/**
 * Parses a port.
 * @param {string} s The text.
 * @throws {RangeError} When the text is not a number.
 */
function port(s) {}
```
