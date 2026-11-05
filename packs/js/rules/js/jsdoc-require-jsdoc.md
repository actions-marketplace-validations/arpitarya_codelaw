---
id: js/jsdoc-require-jsdoc
title: "Function declarations have JSDoc"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the documentation is the author's to write"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/require-jsdoc]
since: 0.5.0
---

Every function declaration (generators included, exported or not) has a JSDoc block directly above it — upstream's default, which checks function declarations only. A block separated from the function by a blank line still counts here.

**Why:** Function declarations are a module's named operations; without a JSDoc block, hovers and generated docs have nothing to show for them.

```codelaw
match:
  any:
    - kind: function_declaration
    - kind: generator_function_declaration
  not:
    any:
      - follows: {kind: comment, regex: '^/\*\*[^*/]'}
      - inside: {kind: export_statement, follows: {kind: comment, regex: '^/\*\*[^*/]'}}
message: "Document this function with a `/** … */` block saying what it does"
```

## How to fix

1. Add a `/** … */` block directly above the function saying what it does, its parameters and what it returns.

## Examples

### JavaScript

#### Don't

```js
function double(n) {
  return n * 2;
}
```

#### Do

```js
/** Doubles a number. */
function double(n) {
  return n * 2;
}
```

### JSX

#### Don't

```jsx
function double(n) {
  return n * 2;
}
```

#### Do

```jsx
/** Doubles a number. */
function double(n) {
  return n * 2;
}
```

### TypeScript

#### Don't

```ts
function double(n) {
  return n * 2;
}
```

#### Do

```ts
/** Doubles a number. */
function double(n) {
  return n * 2;
}
```

### TSX

#### Don't

```tsx
function double(n) {
  return n * 2;
}
```

#### Do

```tsx
/** Doubles a number. */
function double(n) {
  return n * 2;
}
```
