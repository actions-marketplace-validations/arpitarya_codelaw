---
id: js/regexp-prefer-named-replacement
title: "A named group is put in a replacement by name"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/prefer-named-replacement]
since: 0.5.0
---

A replacement refers to a named group with `$<name>`, not `$n`.

**Why:** The name survives a reordering of the groups.

```codelaw
native: js/regexp-prefer-named-replacement
```

## Examples

### JavaScript

#### Don't

```js
s.replace(/(?<y>\d+)/, '$1');
```

#### Do

```js
s.replace(/(?<y>\d+)/, '$<y>');
```

### JSX

#### Don't

```jsx
s.replace(/(?<y>\d+)/, '$1');
```

#### Do

```jsx
s.replace(/(?<y>\d+)/, '$<y>');
```

### TypeScript

#### Don't

```ts
s.replace(/(?<y>\d+)/, '$1');
```

#### Do

```ts
s.replace(/(?<y>\d+)/, '$<y>');
```

### TSX

#### Don't

```tsx
s.replace(/(?<y>\d+)/, '$1');
```

#### Do

```tsx
s.replace(/(?<y>\d+)/, '$<y>');
```
