---
id: js/prefer-unicode-code-point-escapes
title: "Code point escapes rather than \\x and \\u escapes"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-unicode-code-point-escapes]
since: 0.5.0
---

In a string or template, a `\xHH` or `\uHHHH` escape is written `\u{…}`, and a surrogate pair written as two `\u` escapes is the one code point it encodes: `\uD83D\uDCA9` is `\u{1F4A9}`. Printable ASCII escapes are `js/prefer-literal-ascii`'s, and lone surrogates have no code point form; both are left. Regular expressions, which need the `u` flag for the form, are not rewritten here.

**Why:** The code point form names the character directly, astral ones included.

```codelaw
native: js/prefer-unicode-code-point-escapes
```

## Examples

### JavaScript

#### Don't

```js
const copyright = "\xA9";
```

#### Do

```js
const copyright = "\u{A9}";
```


### JSX

#### Don't

```jsx
const copyright = "\xA9";
```

#### Do

```jsx
const copyright = "\u{A9}";
```


### TypeScript

#### Don't

```ts
const copyright = "\xA9";
```

#### Do

```ts
const copyright = "\u{A9}";
```


### TSX

#### Don't

```tsx
const copyright = "\xA9";
```

#### Do

```tsx
const copyright = "\u{A9}";
```
