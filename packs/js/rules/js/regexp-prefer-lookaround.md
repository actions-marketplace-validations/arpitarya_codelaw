---
id: js/regexp-prefer-lookaround
title: "A captured text put back unchanged is matched with a lookaround"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "matching with a lookaround changes which text a global replacement consumes, so overlapping matches can differ"
stability: preview
ported_from: [eslint-plugin-regexp/prefer-lookaround]
since: 0.5.0
---

A replacement that starts or ends with `$1`, where group 1 is the pattern's first or last element, uses a lookbehind or lookahead instead: `s.replace(/(a)b/, '$1c')` is `s.replace(/(?<=a)b/, 'c')`.

**Why:** The capture and the reinsertion say less than a lookaround.

```codelaw
native: js/regexp-prefer-lookaround
```

## How to fix

1. Turn the group into a lookbehind (or lookahead) and drop the `$1`.

## Examples

### JavaScript

#### Don't

```js
s.replace(/(a)b/g, '$1c');
```

#### Do

```js
s.replace(/(?<=a)b/g, 'c');
```

### JSX

#### Don't

```jsx
s.replace(/(a)b/g, '$1c');
```

#### Do

```jsx
s.replace(/(?<=a)b/g, 'c');
```

### TypeScript

#### Don't

```ts
s.replace(/(a)b/g, '$1c');
```

#### Do

```ts
s.replace(/(?<=a)b/g, 'c');
```

### TSX

#### Don't

```tsx
s.replace(/(a)b/g, '$1c');
```

#### Do

```tsx
s.replace(/(?<=a)b/g, 'c');
```
