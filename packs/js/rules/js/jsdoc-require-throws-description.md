---
id: js/jsdoc-require-throws-description
title: "@throws says when"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the description is the author's words"
stability: preview
ported_from: [eslint-plugin-jsdoc/require-throws-description]
since: 0.5.0
---

A JSDoc `@throws` tag carries a description of when the error is thrown.

**Why:** An error type alone does not tell callers what condition leads to it.

```codelaw
match:
  kind: comment
  all:
  - regex: ^/\*\*[^*/]
  - regex: (?m)^[ \t]*(?:/\*\*|\*)[ \t]*@(?:throws|exception)\b[ \t]*(?:\{(?:[^{}\n]|\{(?:[^{}\n]|\{[^{}\n]*\})*\})*\}[ \t]*)?[ \t]*(?:\*/|\r?\n[ \t]*\*(?:[ \t]*@|[ \t]*\r?\n|/))
message: Say when this error is thrown after the `@throws` type
```

## How to fix

1. After the `@throws` type, write the condition that throws it.

## Examples

### JavaScript

#### Don't

```js
/**
 * Parses a port.
 * @param {string} s The text.
 * @throws {RangeError}
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
 * @throws {RangeError}
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
 * @throws {RangeError}
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
 * @throws {RangeError}
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
