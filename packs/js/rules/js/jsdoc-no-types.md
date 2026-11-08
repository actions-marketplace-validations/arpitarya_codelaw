---
id: js/jsdoc-no-types
title: "No JSDoc types in TypeScript"
languages: [typescript, tsx]
fix: suggest
fix_reason: "the JSDoc type may say more than the signature (a narrower type, a union the code checks); the author confirms it can go"
stability: preview
ported_from: [eslint-plugin-jsdoc/no-types]
since: 0.5.0
---

In TypeScript, `@param` and `@returns` carry no `{type}`: the signature already states it. This port runs on TypeScript files only; in JavaScript the JSDoc type is the only type there is.

**Why:** A type in the comment duplicates the signature, and when one changes the other is left saying something false.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@(param|arg|argument|returns?)[ \t]*\{'
message: "The signature already gives this type; drop the `{type}` from the JSDoc tag"
fix:
  - edit-text: { pattern: '((?:\A/\*\*|\n)[ \t]*(?:\*[ \t]*)?@(?:param|arg|argument|returns?))[ \t]*\{[^{}\n]*\}', replace: '${1}' }
```

## How to fix

1. Remove the `{type}` from each `@param` and `@returns` tag; keep the name and description.

## Examples

### TypeScript

#### Don't

```ts
/**
 * Doubles a number.
 * @param {number} n the number
 */
function double(n: number) {
  return n * 2;
}
```

#### Do

```ts
/**
 * Doubles a number.
 * @param n the number
 */
function double(n: number) {
  return n * 2;
}
```

### TSX

#### Don't

```tsx
/**
 * Doubles a number.
 * @param {number} n the number
 */
function double(n: number) {
  return n * 2;
}
```

#### Do

```tsx
/**
 * Doubles a number.
 * @param n the number
 */
function double(n: number) {
  return n * 2;
}
```
