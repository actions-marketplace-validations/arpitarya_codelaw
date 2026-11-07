---
id: js/func-style
title: "Functions as expressions, not declarations"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a declaration is hoisted and an expression is not, so callers above it may break; the author moves it"
fix_exception: restructure
stability: preview
ported_from: [eslint/func-style]
since: 0.5.0
---

A function is written as a `const` assigned a function expression or arrow, not as a `function` declaration (ESLint's default, `expression`). Exported declarations are left alone.

**Why:** An expression is defined where it is written, never before. A matter of taste, so in the opt-in `js/taste` preset.

```codelaw
match:
  kind: function_declaration
  not:
    inside: { kind: export_statement }
message: "Write this function as an expression assigned to a `const`"
```

## How to fix

1. Write `const name = function (…) { … };` or an arrow, and check nothing calls it before that line.

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
const double = function (n) {
  return n * 2;
};
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
const double = function (n) {
  return n * 2;
};
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
const double = function (n) {
  return n * 2;
};
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
const double = function (n) {
  return n * 2;
};
```
