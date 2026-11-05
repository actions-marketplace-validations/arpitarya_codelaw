---
id: js/explicit-length-check
title: "Compare length explicitly"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "an object whose `length` is not a number (a custom class) compares differently"
stability: preview
ported_from: [eslint-plugin-unicorn/explicit-length-check]
since: 0.5.0
---

A `.length` used as the whole condition of an `if` or `while` is compared with 0 explicitly. Upstream also covers `!x.length`, `.size` and other boolean positions; this port covers the plain condition.

**Why:** An explicit comparison says that the count is being checked, not the truthiness of some value.

```codelaw

match:
  pattern: $X.length
  inside:
    kind: parenthesized_expression
    inside: { any: [{kind: if_statement}, {kind: while_statement}], field: condition }
message: "Compare `$X.length` with 0 explicitly"
fix: "$X.length > 0"
```

## Examples

### JavaScript

#### Don't

```js
if (items.length) {
  render(items);
}
```

#### Do

```js
if (items.length > 0) {
  render(items);
}
```

### JSX

#### Don't

```jsx
if (items.length) {
  render(items);
}
```

#### Do

```jsx
if (items.length > 0) {
  render(items);
}
```

### TypeScript

#### Don't

```ts
if (items.length) {
  render(items);
}
```

#### Do

```ts
if (items.length > 0) {
  render(items);
}
```

### TSX

#### Don't

```tsx
if (items.length) {
  render(items);
}
```

#### Do

```tsx
if (items.length > 0) {
  render(items);
}
```
