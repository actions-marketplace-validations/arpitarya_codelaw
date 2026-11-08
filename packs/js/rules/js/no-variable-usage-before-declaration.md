---
id: js/no-variable-usage-before-declaration
title: "No `var` used before its declaration"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving the declaration above its first use reorders the code"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-variable-usage-before-declaration]
since: 0.5.0
---

A `var` that is read or written earlier in the file than its declaration — also from inside a function above it — is reported at its first such use. `let`, `const` and parameters are left to `js/no-use-before-define`.

**Why:** Hoisting makes the early use read `undefined` instead of failing, which hides the mistake.

```codelaw
native: js/no-variable-usage-before-declaration
```

## How to fix

1. Move the declaration above the first use.

## Examples

### JavaScript

#### Don't

```js
total(n);
var n = 3;
```

#### Do

```js
var n = 3;
total(n);
```


### JSX

#### Don't

```jsx
total(n);
var n = 3;
```

#### Do

```jsx
var n = 3;
total(n);
```


### TypeScript

#### Don't

```ts
total(n);
var n = 3;
```

#### Do

```ts
var n = 3;
total(n);
```


### TSX

#### Don't

```tsx
total(n);
var n = 3;
```

#### Do

```tsx
var n = 3;
total(n);
```
