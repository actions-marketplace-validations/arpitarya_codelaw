---
id: js/expression-complexity
title: "No expression with more than three conditional operators"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the intermediate conditions need names, which are the author's"
fix_exception: value
options: { max: { type: "integer", default: 3, description: "the most `&&`, `||`, `??` and `?:` operators one expression may hold" } }
stability: preview
ported_from: [eslint-plugin-sonarjs/expression-complexity]
since: 0.5.0
---

One expression holds at most `max` (3) of the operators `&&`, `||`, `??` and `?:`. Functions written inside it have their own count.

**Why:** A long boolean expression is easier to read as named parts.

```codelaw
native: js/expression-complexity
```

## How to fix

1. Give parts of the expression names (`const isReady = a && b;`) and combine the names.

## Examples

### JavaScript

#### Don't

```js
if (a && b && c && d && e) {}
```

#### Do

```js
const ready = a && b;
if (ready && c && d) {}
```


### JSX

#### Don't

```jsx
if (a && b && c && d && e) {}
```

#### Do

```jsx
const ready = a && b;
if (ready && c && d) {}
```


### TypeScript

#### Don't

```ts
if (a && b && c && d && e) {}
```

#### Do

```ts
const ready = a && b;
if (ready && c && d) {}
```


### TSX

#### Don't

```tsx
if (a && b && c && d && e) {}
```

#### Do

```tsx
const ready = a && b;
if (ready && c && d) {}
```
