---
id: js/regexp-no-useless-dollar-replacements
title: "No replacement reference to a group that doesn't exist"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/no-useless-dollar-replacements]
since: 0.5.0
---

A `$n` or `$<name>` in a replacement string names a group the regular expression has.

**Why:** It inserts the text literally, a silent bug.

```codelaw
native: js/regexp-no-useless-dollar-replacements
```

## How to fix

1. Escape the dollar sign as `$$` (what the code does now), or reference an existing group.

## Examples

### JavaScript

#### Don't

```js
s.replace(/(a)/, '$2');
```

#### Do

```js
s.replace(/(a)/, '$$2');
```

### JSX

#### Don't

```jsx
s.replace(/(a)/, '$2');
```

#### Do

```jsx
s.replace(/(a)/, '$$2');
```

### TypeScript

#### Don't

```ts
s.replace(/(a)/, '$2');
```

#### Do

```ts
s.replace(/(a)/, '$$2');
```

### TSX

#### Don't

```tsx
s.replace(/(a)/, '$2');
```

#### Do

```tsx
s.replace(/(a)/, '$$2');
```
