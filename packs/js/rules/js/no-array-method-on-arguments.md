---
id: js/no-array-method-on-arguments
title: "arguments has no array methods"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the call throws today; `Array.from(arguments)` makes it run, on a copy, so a mutating method no longer touches `arguments`"
stability: preview
ported_from: [oxlint/bad-array-method-on-arguments]
since: 0.5.0
---

Code converts `arguments` to an array (or uses a rest parameter) before calling an array method on it.

**Why:** `arguments` is array-like, not an array: `arguments.map(...)` throws a TypeError.

```codelaw
match:
  pattern: $ARGS.$M($$$A)
constraints:
  ARGS: { regex: ^arguments$ }
  M: { regex: ^(at|concat|copyWithin|entries|every|fill|filter|find|findIndex|findLast|findLastIndex|flat|flatMap|forEach|includes|indexOf|join|keys|lastIndexOf|map|pop|push|reduce|reduceRight|reverse|shift|slice|some|sort|splice|toReversed|toSorted|toSpliced|unshift|values|with)$ }
message: "`arguments` has no `$M`; call it on `Array.from(arguments)`, or use a rest parameter"
fix: "Array.from(arguments).$M($$$A)"
```

## Examples

### JavaScript

#### Don't

```js
function sum() {
  return arguments.reduce((a, b) => a + b, 0);
}
```

#### Do

```js
function sum() {
  return Array.from(arguments).reduce((a, b) => a + b, 0);
}
```

### JSX

#### Don't

```jsx
function sum() {
  return arguments.reduce((a, b) => a + b, 0);
}
```

#### Do

```jsx
function sum() {
  return Array.from(arguments).reduce((a, b) => a + b, 0);
}
```

### TypeScript

#### Don't

```ts
function sum() {
  return arguments.reduce((a, b) => a + b, 0);
}
```

#### Do

```ts
function sum() {
  return Array.from(arguments).reduce((a, b) => a + b, 0);
}
```

### TSX

#### Don't

```tsx
function sum() {
  return arguments.reduce((a, b) => a + b, 0);
}
```

#### Do

```tsx
function sum() {
  return Array.from(arguments).reduce((a, b) => a + b, 0);
}
```
