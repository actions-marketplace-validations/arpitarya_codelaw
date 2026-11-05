---
id: js/regexp-no-trivially-nested-assertion
title: "No lookaround around a single assertion"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-trivially-nested-assertion]
since: 0.5.0
---

A lookaround doesn't hold just one assertion: `(?=$)` is `$`, `(?!\b)` is `\B`.

**Why:** The wrapper adds nothing.

```codelaw
native: js/regexp-no-trivially-nested-assertion
```

## Examples

### JavaScript

#### Don't

```js
const re = /a(?=$)/;
```

#### Do

```js
const re = /a$/;
```

### JSX

#### Don't

```jsx
const re = /a(?=$)/;
```

#### Do

```jsx
const re = /a$/;
```

### TypeScript

#### Don't

```ts
const re = /a(?=$)/;
```

#### Do

```ts
const re = /a$/;
```

### TSX

#### Don't

```tsx
const re = /a(?=$)/;
```

#### Do

```tsx
const re = /a$/;
```
