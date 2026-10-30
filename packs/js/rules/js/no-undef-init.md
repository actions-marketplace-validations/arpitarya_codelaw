---
id: js/no-undef-init
title: "No initialising to undefined"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/no-undef-init]
since: 0.5.0
---

A `let` or `var` is not initialised to `undefined`.

**Why:** A variable declared without a value is already `undefined`.

```codelaw
match:
  kind: variable_declarator
  all:
  - has: {field: value, kind: undefined}
  - has: {field: name, kind: identifier, pattern: $X}
  - inside:
      any:
      - {kind: lexical_declaration, regex: ^let\b}
      - {kind: variable_declaration}
message: Do not initialise to `undefined`; it already is
fix: $X
```

## Examples

### JavaScript

#### Don't

```js
let x = undefined;
```

#### Do

```js
let x;
```

#### Fixed

```js
let x;
```

### JSX

#### Don't

```jsx
let x = undefined;
```

#### Do

```jsx
let x;
```

#### Fixed

```jsx
let x;
```

### TypeScript

#### Don't

```ts
let x = undefined;
```

#### Do

```ts
let x;
```

#### Fixed

```ts
let x;
```

### TSX

#### Don't

```tsx
let x = undefined;
```

#### Do

```tsx
let x;
```

#### Fixed

```tsx
let x;
```
