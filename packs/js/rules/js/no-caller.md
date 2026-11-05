---
id: js/no-caller
title: "No arguments.callee or arguments.caller"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "naming the function to refer to it changes its declaration"
fix_pending: "auto-subset; needs scope-rename"
stability: preview
ported_from: [eslint/no-caller]
since: 0.5.0
---

`arguments.callee` and `arguments.caller` are not used.

**Why:** Both are forbidden in strict mode and block engine optimisations.

```codelaw
match:
  kind: member_expression
  regex: '^arguments\.(callee|caller)$'
message: "Do not use `$$MATCH`; name the function instead"
```

## How to fix

1. Give the function a name and refer to it by that name.

## Examples

### JavaScript

#### Don't

```js
const f = function () { return arguments.callee; };
```

#### Do

```js
const f = function self() { return self; };
```

### JSX

#### Don't

```jsx
const f = function () { return arguments.callee; };
```

#### Do

```jsx
const f = function self() { return self; };
```

### TypeScript

#### Don't

```ts
const f = function () { return arguments.callee; };
```

#### Do

```ts
const f = function self() { return self; };
```

### TSX

#### Don't

```tsx
const f = function () { return arguments.callee; };
```

#### Do

```tsx
const f = function self() { return self; };
```
