---
id: js/no-object-as-default-parameter
title: "No object literal as a parameter default"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "destructuring the parameter changes its name inside the function"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-object-as-default-parameter]
since: 0.5.0
---

A parameter default is not an object literal with properties.

**Why:** Passing `{ size: 2 }` drops the `color` default too, which is rarely meant.

```codelaw

match:
  any:
    - { kind: assignment_pattern, has: { field: right, kind: object, has: { kind: pair } } }
    - { kind: required_parameter, has: { field: value, kind: object, has: { kind: pair } } }
  inside: { kind: formal_parameters }
message: "A default object is replaced whole when the caller passes one; default each property instead"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
function draw(opts = { size: 1, color: "red" }) {}
```

#### Do

```js
function draw({ size = 1, color = "red" } = {}) {}
```

### JSX

#### Don't

```jsx
function draw(opts = { size: 1, color: "red" }) {}
```

#### Do

```jsx
function draw({ size = 1, color = "red" } = {}) {}
```

### TypeScript

#### Don't

```ts
function draw(opts = { size: 1, color: "red" }) {}
```

#### Do

```ts
function draw({ size = 1, color = "red" } = {}) {}
```

### TSX

#### Don't

```tsx
function draw(opts = { size: 1, color: "red" }) {}
```

#### Do

```tsx
function draw({ size = 1, color = "red" } = {}) {}
```
