---
id: node/no-top-level-await
title: "No await at the top level of a module"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving the await into a function changes when the module finishes loading and what its importers see"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-n/no-top-level-await]
since: 0.5.0
---

`await` (and `for await`) is used inside an async function, not at the top level of a module.

**Why:** Top-level await makes every importer wait for this module, and a module graph that uses it can't be loaded with `require()`.

```codelaw
match:
  any:
    - kind: await_expression
    - { kind: for_in_statement, regex: ^for\s+await }
  not:
    inside: { stopBy: end, any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}] }
message: "Move this `await` into an async function"
```

## How to fix

1. Wrap the top-level work in an exported async function (`export async function init()`) that importers call.
2. In an entry script, use `main().catch(...)` with an `async function main()`.

## Examples

### JavaScript

#### Don't

```js
const config = await loadConfig();
export default config;
```

#### Do

```js
export async function getConfig() {
  return await loadConfig();
}
```

### JSX

#### Don't

```jsx
const config = await loadConfig();
export default config;
```

#### Do

```jsx
export async function getConfig() {
  return await loadConfig();
}
```

### TypeScript

#### Don't

```ts
const config = await loadConfig();
export default config;
```

#### Do

```ts
export async function getConfig() {
  return await loadConfig();
}
```

### TSX

#### Don't

```tsx
const config = await loadConfig();
export default config;
```

#### Do

```tsx
export async function getConfig() {
  return await loadConfig();
}
```
