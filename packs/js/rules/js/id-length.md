---
id: js/id-length
title: "A declared name is at least two characters long"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the longer name is the author's"
fix_exception: value
stability: preview
ported_from: [eslint/id-length]
since: 0.5.0
---

A name a file declares — a variable, function, class, parameter, catch parameter or import — is at least two characters long (ESLint's default `min: 2`).

**Why:** One letter says nothing about what the value is. A matter of taste, so in the opt-in `js/taste` preset.

```codelaw
match:
  kind: identifier
  regex: '^[A-Za-z$_]$'
where:
  $$MATCH: declaration
message: "Give `$$MATCH` a name that says what it holds"
```

## How to fix

1. Rename it to a word that says what it holds, and update its uses.

## Examples

### JavaScript

#### Don't

```js
const t = Date.now();
```

#### Do

```js
const startedAt = Date.now();
```


### JSX

#### Don't

```jsx
const t = Date.now();
```

#### Do

```jsx
const startedAt = Date.now();
```


### TypeScript

#### Don't

```ts
const t = Date.now();
```

#### Do

```ts
const startedAt = Date.now();
```


### TSX

#### Don't

```tsx
const t = Date.now();
```

#### Do

```tsx
const startedAt = Date.now();
```
