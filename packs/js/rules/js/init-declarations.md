---
id: js/init-declarations
title: "A variable is initialised where it is declared"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the starting value is the author's"
fix_exception: value
stability: preview
ported_from: [eslint/init-declarations]
since: 0.5.0
---

A `let` or `var` declaration gives its variable a value (ESLint's default, `always`); the loop variable of `for … in` and `for … of` and TypeScript's `declare` are left alone.

**Why:** A variable that starts as `undefined` invites a read before the write. A matter of taste, so in the opt-in `js/taste` preset.

```codelaw
match:
  kind: variable_declarator
  not:
    any:
      - has: { field: value }
      - inside: { kind: for_in_statement, stopBy: end }
      - inside: { kind: ambient_declaration, stopBy: end }
message: "Give this variable its starting value here"
```

## How to fix

1. Assign the value where the variable is declared, moving the declaration down to where it is known if need be.

## Examples

### JavaScript

#### Don't

```js
let total;
total = sum(items);
```

#### Do

```js
let total = sum(items);
```


### JSX

#### Don't

```jsx
let total;
total = sum(items);
```

#### Do

```jsx
let total = sum(items);
```


### TypeScript

#### Don't

```ts
let total;
total = sum(items);
```

#### Do

```ts
let total = sum(items);
```


### TSX

#### Don't

```tsx
let total;
total = sum(items);
```

#### Do

```tsx
let total = sum(items);
```
