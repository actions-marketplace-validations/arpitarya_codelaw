---
id: js/regexp-no-legacy-features
title: "No legacy `RegExp` static properties"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "reading the match result instead of the global properties moves code to where the match is made"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-regexp/no-legacy-features]
since: 0.5.0
---

`RegExp.$1`–`$9`, `RegExp.lastMatch`, `RegExp.input`, `RegExp.leftContext` and the other legacy statics are not read.

**Why:** They are a global record of the last match anywhere: any other match overwrites them.

```codelaw
native: js/regexp-no-legacy-features
```

## How to fix

1. Keep the match result (`const m = re.exec(s)`) and read `m[1]`.

## Examples

### JavaScript

#### Don't

```js
const first = RegExp.$1;
```

#### Do

```js
const first = match[1];
```

### JSX

#### Don't

```jsx
const first = RegExp.$1;
```

#### Do

```jsx
const first = match[1];
```

### TypeScript

#### Don't

```ts
const first = RegExp.$1;
```

#### Do

```ts
const first = match[1];
```

### TSX

#### Don't

```tsx
const first = RegExp.$1;
```

#### Do

```tsx
const first = match[1];
```
