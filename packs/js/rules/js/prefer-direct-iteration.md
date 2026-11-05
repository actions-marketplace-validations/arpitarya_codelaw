---
id: js/prefer-direct-iteration
title: "for…of an array, not array.values()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a map is iterated as entries, not values, once `.values()` is gone"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-direct-iteration]
since: 0.5.0
---

A `for … of` loop does not iterate `.values()` of what it loops over.

**Why:** Arrays, sets and maps are iterable already; `.values()` only adds a call.

```codelaw
match:
  kind: call_expression
  pattern: $A.values()
  inside: { kind: for_in_statement, field: right, regex: '^for\s*\(.+\bof\b' }
message: "`for … of` already iterates the values; drop `.values()`"
fix: "$A"
```

## Examples

### JavaScript

#### Don't

```js
for (const x of list.values()) {
  use(x);
}
```

#### Do

```js
for (const x of list) {
  use(x);
}
```

### JSX

#### Don't

```jsx
for (const x of list.values()) {
  use(x);
}
```

#### Do

```jsx
for (const x of list) {
  use(x);
}
```

### TypeScript

#### Don't

```ts
for (const x of list.values()) {
  use(x);
}
```

#### Do

```ts
for (const x of list) {
  use(x);
}
```

### TSX

#### Don't

```tsx
for (const x of list.values()) {
  use(x);
}
```

#### Do

```tsx
for (const x of list) {
  use(x);
}
```
