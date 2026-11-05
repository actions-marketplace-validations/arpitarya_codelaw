---
id: js/declarations-in-global-scope
title: "No var or function at the top of a script"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "turning the file into a module changes how it is loaded"
stability: preview
ported_from: [eslint-plugin-sonarjs/declarations-in-global-scope]
since: 0.5.0
---

A script (a file with no `import` or `export`) does not declare `var` variables or functions at its top level.

**Why:** In a classic script these become properties of the global object, shared with every other script on the page.

```codelaw

match:
  any: [{kind: variable_declaration}, {kind: function_declaration}]
  inside: { kind: program, not: { has: { any: [{kind: import_statement}, {kind: export_statement}] } } }
message: "This declaration becomes a global; wrap the script in a module or a function"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
var config = load();
```

#### Do

```js
export const config = load();
```

### JSX

#### Don't

```jsx
var config = load();
```

#### Do

```jsx
export const config = load();
```

### TypeScript

#### Don't

```ts
var config = load();
```

#### Do

```ts
export const config = load();
```

### TSX

#### Don't

```tsx
var config = load();
```

#### Do

```tsx
export const config = load();
```
