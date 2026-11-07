---
id: js/no-shadow-restricted-names
title: "No declaring undefined, NaN, Infinity, arguments, eval or globalThis"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "renamed automatically to the first of `_NaN`, `_NaN2`, … that the file does not use, with every reference to it; an exported name, or one in a shorthand `{ NaN }`, is renamed by hand"
stability: preview
ported_from: [eslint/no-shadow-restricted-names, eslint-plugin-sonarjs/no-globals-shadowing]
since: 0.1.0
---

No variable, parameter or function is named `undefined`, `NaN`, `Infinity`, `arguments`, `eval` or
`globalThis`.

**Why:** Shadowing these names makes code that relies on them behave in confusing ways.

```codelaw
match:
  kind: identifier
  regex: ^(undefined|NaN|Infinity|arguments|eval|globalThis)$
where:
  $$MATCH: declaration
message: "`$$MATCH` is a restricted name; don't declare it"
fix:
  - rename-symbol: { target: $$MATCH, to: "$FRESH(_$$MATCH)", scope: binding }
```

## How to fix

1. Rename the declaration and its uses.

## Examples

### JavaScript

#### Don't

```js
function check(NaN) {
  return NaN;
}
```

#### Do

```js
function check(_NaN) {
  return _NaN;
}
```

### JSX

#### Don't

```jsx
function check(NaN) {
  return NaN;
}
```

#### Do

```jsx
function check(_NaN) {
  return _NaN;
}
```

### TypeScript

#### Don't

```ts
function check(NaN) {
  return NaN;
}
```

#### Do

```ts
function check(_NaN) {
  return _NaN;
}
```

### TSX

#### Don't

```tsx
function check(NaN) {
  return NaN;
}
```

#### Do

```tsx
function check(_NaN) {
  return _NaN;
}
```
