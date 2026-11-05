---
id: js/consistent-return
title: "return undefined explicitly beside valued returns"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the behaviour is unchanged, but whether `undefined` is the right value is the author's to confirm"
stability: preview
ported_from: [eslint/consistent-return]
since: 0.5.0
---

A function that returns a value with some `return` statements does not also use a bare `return;`. Upstream also reports falling off the end of such a function; that needs control flow and is left out.

**Why:** A bare `return` beside valued ones looks like a forgotten value; `return undefined` says the empty result is meant.

```codelaw

match:
  kind: return_statement
  regex: '^return\s*;?$'
  inside:
    stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}] }
    any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}]
    has:
      stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}] }
      kind: return_statement
      regex: '^return\s*[^\s;]'
message: "This function returns a value elsewhere; say what this `return` gives back"
fix: "return undefined;"
```

## Examples

### JavaScript

#### Don't

```js
function find(list, id) {
  if (!list) return;
  return list.get(id);
}
```

#### Do

```js
function find(list, id) {
  if (!list) return undefined;
  return list.get(id);
}
```

### JSX

#### Don't

```jsx
function find(list, id) {
  if (!list) return;
  return list.get(id);
}
```

#### Do

```jsx
function find(list, id) {
  if (!list) return undefined;
  return list.get(id);
}
```

### TypeScript

#### Don't

```ts
function find(list, id) {
  if (!list) return;
  return list.get(id);
}
```

#### Do

```ts
function find(list, id) {
  if (!list) return undefined;
  return list.get(id);
}
```

### TSX

#### Don't

```tsx
function find(list, id) {
  if (!list) return;
  return list.get(id);
}
```

#### Do

```tsx
function find(list, id) {
  if (!list) return undefined;
  return list.get(id);
}
```
