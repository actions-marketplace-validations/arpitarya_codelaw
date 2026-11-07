---
id: js/import-no-mutable-exports
title: "No exported let or var"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether importers should see later changes is the author's design"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-import/no-mutable-exports]
since: 0.5.0
---

A module exports `const` bindings, functions and classes, not `let` or `var`.

**Why:** An exported `let` changes under every importer when the module reassigns it, which is rarely what they expect.

```codelaw
match:
  kind: export_statement
  has:
    field: declaration
    any:
      - { kind: variable_declaration }
      - { kind: lexical_declaration, regex: '^let\b' }
message: "Export a `const`, not a mutable binding"
```

## How to fix

1. Make it `const`; if it must change, export a function that returns the current value.

## Examples

### JavaScript

#### Don't

```js
export let count = 0;
```

#### Do

```js
export const count = 0;
```


### JSX

#### Don't

```jsx
export let count = 0;
```

#### Do

```jsx
export const count = 0;
```


### TypeScript

#### Don't

```ts
export let count = 0;
```

#### Do

```ts
export const count = 0;
```


### TSX

#### Don't

```tsx
export let count = 0;
```

#### Do

```tsx
export const count = 0;
```
