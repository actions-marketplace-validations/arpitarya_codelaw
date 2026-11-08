---
id: js/import-export
title: "No name exported twice from one module"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which of the two exports is the intended one is the author's knowledge"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/export]
since: 0.5.0
---

A name exported more than once from the same file — two declarations, a declaration and an `export { … }`, two specifiers, or two default exports — is reported at each. A value and a type of one name are different exports, and TypeScript declaration merging is allowed. Names an `export * from` brings in from another module are the project graph's, and not checked here.

**Why:** A duplicate export is a syntax error in a real module loader, or silently picks one of the two.

```codelaw
native: js/import-export
```

## How to fix

1. Remove or rename one of the exports.

## Examples

### JavaScript

#### Don't

```js
const a = 1;
const b = 2;
export { a, b as a };
```

#### Do

```js
const a = 1;
const b = 2;
export { a, b };
```


### JSX

#### Don't

```jsx
const a = 1;
const b = 2;
export { a, b as a };
```

#### Do

```jsx
const a = 1;
const b = 2;
export { a, b };
```


### TypeScript

#### Don't

```ts
const a = 1;
const b = 2;
export { a, b as a };
```

#### Do

```ts
const a = 1;
const b = 2;
export { a, b };
```


### TSX

#### Don't

```tsx
const a = 1;
const b = 2;
export { a, b as a };
```

#### Do

```tsx
const a = 1;
const b = 2;
export { a, b };
```
