---
id: js/regexp-no-missing-g-flag
title: "`replaceAll` and `matchAll` get a regular expression with the `g` flag"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "adding `g` makes a call that throws work; the result is what was meant, but the behaviour changes"
stability: preview
ported_from: [eslint-plugin-regexp/no-missing-g-flag]
since: 0.5.0
---

The regular expression passed to `String#replaceAll` or `String#matchAll` has the `g` flag.

**Why:** Without it, both throw a TypeError.

```codelaw
native: js/regexp-no-missing-g-flag
```

## Examples

### JavaScript

#### Don't

```js
s.replaceAll(/a/, 'b');
```

#### Do

```js
s.replaceAll(/a/g, 'b');
```

### JSX

#### Don't

```jsx
s.replaceAll(/a/, 'b');
```

#### Do

```jsx
s.replaceAll(/a/g, 'b');
```

### TypeScript

#### Don't

```ts
s.replaceAll(/a/, 'b');
```

#### Do

```ts
s.replaceAll(/a/g, 'b');
```

### TSX

#### Don't

```tsx
s.replaceAll(/a/, 'b');
```

#### Do

```tsx
s.replaceAll(/a/g, 'b');
```
