---
id: js/sort-vars
title: "Variables of one declaration in ascending order"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/sort-vars]
since: 0.5.0
---

The names declared by one `let`, `const` or `var` are in ascending (code-unit) order, ESLint's default. The fix sorts them when every declarator is a name with no value or a literal value, so nothing observable moves.

**Why:** One order for a list of names makes it easy to scan.

```codelaw
native: js/sort-vars
```

## Examples

### JavaScript

#### Don't

```js
let b, a;
```

#### Do

```js
let a, b;
```


### JSX

#### Don't

```jsx
let b, a;
```

#### Do

```jsx
let a, b;
```


### TypeScript

#### Don't

```ts
let b, a;
```

#### Do

```ts
let a, b;
```


### TSX

#### Don't

```tsx
let b, a;
```

#### Do

```tsx
let a, b;
```
