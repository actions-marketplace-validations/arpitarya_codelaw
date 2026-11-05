---
id: js/symbol-description
title: "A Symbol has a description"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the description is the author's; the fix offers the binding or property name and the reviewer confirms it"
stability: preview
ported_from: [eslint/symbol-description]
since: 0.5.0
---

`Symbol()` is given a description.

**Why:** The description is what a debugger and `String(symbol)` show; without it every symbol looks the same.

```codelaw
match:
  pattern: Symbol()
  any:
    - inside: { kind: variable_declarator, has: { field: name, kind: identifier, pattern: $N } }
      has: { field: arguments, pattern: $ARGS }
    - inside: { kind: pair, has: { field: key, kind: property_identifier, pattern: $N } }
      has: { field: arguments, pattern: $ARGS }
    - kind: call_expression
message: "Give the symbol a description"
fix:
  - replace: { target: $ARGS, with: "('$N')" }
```

## How to fix

1. Pass a short description: `Symbol('cache key')`.

## Examples

### JavaScript

#### Don't

```js
const key = Symbol();
```

#### Fixed

```js
const key = Symbol('key');
```

#### Do

```js
const key = Symbol('key');
```

### JSX

#### Don't

```jsx
const key = Symbol();
```

#### Fixed

```jsx
const key = Symbol('key');
```

#### Do

```jsx
const key = Symbol('key');
```

### TypeScript

#### Don't

```ts
const key = Symbol();
```

#### Fixed

```ts
const key = Symbol('key');
```

#### Do

```ts
const key = Symbol('key');
```

### TSX

#### Don't

```tsx
const key = Symbol();
```

#### Fixed

```tsx
const key = Symbol('key');
```

#### Do

```tsx
const key = Symbol('key');
```
