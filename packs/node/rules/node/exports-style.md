---
id: node/exports-style
title: "module.exports, not the exports shortcut"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "auto only where the file never reassigns `module.exports`, so both names are the same object"
stability: preview
ported_from: [eslint-plugin-n/exports-style]
since: 0.5.0
---

A CommonJS module writes its exports through `module.exports`, not the `exports` shortcut (the upstream default, `"module.exports"`). Assigning `exports = ...` is `node/no-exports-assign`'s.

**Why:** Two names for one object invite the bug where `module.exports` is replaced and the properties set on `exports` silently go nowhere.

```codelaw
match:
  pattern: $E.$P
  inside:
    stopBy: end
    kind: program
    not: { has: { stopBy: end, pattern: module.exports = $V } }
constraints:
  E: { regex: ^exports$ }
where:
  $E: undeclared
message: "Write `module.exports.$P`; one name for the exports object"
fix: "module.exports.$P"
```

## Examples

### JavaScript

#### Don't

```js
exports.parse = function parse(text) {
  return text.split(",");
};
```

#### Do

```js
module.exports.parse = function parse(text) {
  return text.split(",");
};
```

### JSX

#### Don't

```jsx
exports.parse = function parse(text) {
  return text.split(",");
};
```

#### Do

```jsx
module.exports.parse = function parse(text) {
  return text.split(",");
};
```

### TypeScript

#### Don't

```ts
exports.parse = function parse(text) {
  return text.split(",");
};
```

#### Do

```ts
module.exports.parse = function parse(text) {
  return text.split(",");
};
```

### TSX

#### Don't

```tsx
exports.parse = function parse(text) {
  return text.split(",");
};
```

#### Do

```tsx
module.exports.parse = function parse(text) {
  return text.split(",");
};
```
