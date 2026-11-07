---
id: js/prefer-escaped-irregular-whitespace
title: "Irregular whitespace in a string is written as an escape"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-escaped-irregular-whitespace]
since: 0.5.0
---

A string does not hold an irregular whitespace character (no-break space, the en and em spaces, zero-width space, line and paragraph separators, ideographic space, byte-order mark, …) as itself; it is written as its `\u` escape.

**Why:** The character looks like a plain space, or like nothing, so a reader can't see it; the escape shows it is there.

```codelaw
match:
  kind: string
  regex: '[\x{0B}\x{0C}\x{A0}\x{85}\x{1680}\x{180E}\x{2000}-\x{200B}\x{2028}\x{2029}\x{202F}\x{205F}\x{3000}\x{FEFF}]'
  not: { inside: { kind: jsx_attribute } }
message: "Write the irregular whitespace as an escape"
fix:
  - edit-text: { pattern: '\x{0B}', replace: '\v' }
  - edit-text: { pattern: '\x{0C}', replace: '\f' }
  - edit-text: { pattern: '\x{A0}', replace: '\u00A0' }
  - edit-text: { pattern: '\x{85}', replace: '\u0085' }
  - edit-text: { pattern: '\x{1680}', replace: '\u1680' }
  - edit-text: { pattern: '\x{180E}', replace: '\u180E' }
  - edit-text: { pattern: '\x{2000}', replace: '\u2000' }
  - edit-text: { pattern: '\x{2001}', replace: '\u2001' }
  - edit-text: { pattern: '\x{2002}', replace: '\u2002' }
  - edit-text: { pattern: '\x{2003}', replace: '\u2003' }
  - edit-text: { pattern: '\x{2004}', replace: '\u2004' }
  - edit-text: { pattern: '\x{2005}', replace: '\u2005' }
  - edit-text: { pattern: '\x{2006}', replace: '\u2006' }
  - edit-text: { pattern: '\x{2007}', replace: '\u2007' }
  - edit-text: { pattern: '\x{2008}', replace: '\u2008' }
  - edit-text: { pattern: '\x{2009}', replace: '\u2009' }
  - edit-text: { pattern: '\x{200A}', replace: '\u200A' }
  - edit-text: { pattern: '\x{200B}', replace: '\u200B' }
  - edit-text: { pattern: '\x{2028}', replace: '\u2028' }
  - edit-text: { pattern: '\x{2029}', replace: '\u2029' }
  - edit-text: { pattern: '\x{202F}', replace: '\u202F' }
  - edit-text: { pattern: '\x{205F}', replace: '\u205F' }
  - edit-text: { pattern: '\x{3000}', replace: '\u3000' }
  - edit-text: { pattern: '\x{FEFF}', replace: '\uFEFF' }
```

## Examples

### JavaScript

#### Don't

```js
const price = "10 EUR";
```

#### Do

```js
const price = "10\u00A0EUR";
```


### JSX

#### Don't

```jsx
const price = "10 EUR";
```

#### Do

```jsx
const price = "10\u00A0EUR";
```


### TypeScript

#### Don't

```ts
const price = "10 EUR";
```

#### Do

```ts
const price = "10\u00A0EUR";
```


### TSX

#### Don't

```tsx
const price = "10 EUR";
```

#### Do

```tsx
const price = "10\u00A0EUR";
```
