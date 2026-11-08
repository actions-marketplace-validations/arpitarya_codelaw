---
id: js/prefer-literal-ascii
title: "Printable ASCII written as itself, not as an escape"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-literal-ascii]
since: 0.5.0
---

A numeric escape (`\x41`, `\u0042`, `\u{43}`) of a printable ASCII character (space to `~`) is written as the character. The string's own quote, the backslash, and `$` and `{` in a template are left escaped, as are directive prologues, tagged templates, and a digit that would extend a preceding `\0`.

**Why:** An escape makes the reader decode a character they could just read.

```codelaw
native: js/prefer-literal-ascii
```

## Examples

### JavaScript

#### Don't

```js
const word = "\x41\u0042\u{43}";
```

#### Do

```js
const word = "ABC";
```


### JSX

#### Don't

```jsx
const word = "\x41\u0042\u{43}";
```

#### Do

```jsx
const word = "ABC";
```


### TypeScript

#### Don't

```ts
const word = "\x41\u0042\u{43}";
```

#### Do

```ts
const word = "ABC";
```


### TSX

#### Don't

```tsx
const word = "\x41\u0042\u{43}";
```

#### Do

```tsx
const word = "ABC";
```
