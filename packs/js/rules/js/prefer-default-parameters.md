---
id: js/prefer-default-parameters
title: "A default parameter rather than reassigning it"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`||` replaces every falsy argument while a default replaces only `undefined`; offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-default-parameters]
since: 0.5.0
---

A function whose first statement gives its one parameter a fallback (`name = name || "guest"`) declares it as a default parameter (`name = "guest"`).

**Why:** The default is in the signature, where callers and readers look for it.

```codelaw
match:
  kind: function_declaration
  pattern: "function $N($P) { $P = $P || $D; $$$B }"
constraints:
  P: { kind: identifier }
message: "Declare `$P`'s fallback as a default parameter"
fix: "function $N($P = $D) {\n  $$$B\n}"
```

## Examples

### JavaScript

#### Don't

```js
function greet(name) {
  name = name || "guest";
  return `Hi ${name}`;
}
```

#### Do

```js
function greet(name = "guest") {
  return `Hi ${name}`;
}
```


### JSX

#### Don't

```jsx
function greet(name) {
  name = name || "guest";
  return `Hi ${name}`;
}
```

#### Do

```jsx
function greet(name = "guest") {
  return `Hi ${name}`;
}
```


### TypeScript

#### Don't

```ts
function greet(name) {
  name = name || "guest";
  return `Hi ${name}`;
}
```

#### Do

```ts
function greet(name = "guest") {
  return `Hi ${name}`;
}
```


### TSX

#### Don't

```tsx
function greet(name) {
  name = name || "guest";
  return `Hi ${name}`;
}
```

#### Do

```tsx
function greet(name = "guest") {
  return `Hi ${name}`;
}
```
