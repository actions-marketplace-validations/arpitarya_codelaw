---
id: js/regexp-control-character-escape
title: "Control characters are written with their control escape"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-regexp/control-character-escape]
since: 0.5.0
---

A tab, newline, vertical tab, form feed, carriage return or NUL in a pattern is written `\t`, `\n`, `\v`, `\f`, `\r` or `\0`, not as a hexadecimal, Unicode or `\c` escape.

**Why:** The control escapes are the forms every reader knows.

```codelaw
native: js/regexp-control-character-escape
```

## Examples

### JavaScript

#### Don't

```js
const re = /a\x09b/;
```

#### Do

```js
const re = /a\tb/;
```

### JSX

#### Don't

```jsx
const re = /a\x09b/;
```

#### Do

```jsx
const re = /a\tb/;
```

### TypeScript

#### Don't

```ts
const re = /a\x09b/;
```

#### Do

```ts
const re = /a\tb/;
```

### TSX

#### Don't

```tsx
const re = /a\x09b/;
```

#### Do

```tsx
const re = /a\tb/;
```
