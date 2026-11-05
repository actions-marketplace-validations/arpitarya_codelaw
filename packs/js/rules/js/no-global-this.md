---
id: js/no-global-this
title: "No this at the top level"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether `globalThis` or `undefined` was meant is the author's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-global-this]
since: 0.5.0
---

`this` is not used outside any function or class.

**Why:** Its value depends on how the file is loaded, which the code cannot see.

```codelaw

match:
  kind: this
  not: { inside: { stopBy: end, any: [{kind: function_declaration}, {kind: function_expression}, {kind: method_definition}, {kind: class_body}, {kind: generator_function_declaration}, {kind: generator_function}] } }
message: "At the top level `this` is `undefined` in a module and the global object in a script; name what you mean"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
this.ready = true;
```

#### Do

```js
globalThis.ready = true;
```

### JSX

#### Don't

```jsx
this.ready = true;
```

#### Do

```jsx
globalThis.ready = true;
```

### TypeScript

#### Don't

```ts
this.ready = true;
```

#### Do

```ts
globalThis.ready = true;
```

### TSX

#### Don't

```tsx
this.ready = true;
```

#### Do

```tsx
globalThis.ready = true;
```
