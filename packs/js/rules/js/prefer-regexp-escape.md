---
id: js/prefer-regexp-escape
title: "RegExp.escape() rather than a hand-written escape"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-regexp-escape]
since: 0.5.0
---

Text is made safe for a regular expression with `RegExp.escape(text)`, not with a `replace` of the special characters by `\\$&`.

**Why:** The built-in covers every special character and every position; a hand-written class is easy to get subtly wrong.

```codelaw
match:
  pattern: "$S.$M($RE, $REP)"
constraints:
  M: { regex: '^(replace|replaceAll)$' }
  RE:
    kind: regex
    regex: '^/\[.*\]/[a-z]*g[a-z]*$'
    all:
      - regex: '\*'
      - regex: '\+'
      - regex: '\?'
      - regex: '\^'
      - regex: '\$'
      - regex: '\{'
      - regex: '\('
      - regex: '\|'
      - regex: '\.'
  REP: { regex: '^["'']\\\\\$&["'']$' }
message: "Use `RegExp.escape($S)`"
fix: "RegExp.escape($S)"
```

## Examples

### JavaScript

#### Don't

```js
const safe = input.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
```

#### Do

```js
const safe = RegExp.escape(input);
```


### JSX

#### Don't

```jsx
const safe = input.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
```

#### Do

```jsx
const safe = RegExp.escape(input);
```


### TypeScript

#### Don't

```ts
const safe = input.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
```

#### Do

```ts
const safe = RegExp.escape(input);
```


### TSX

#### Don't

```tsx
const safe = input.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
```

#### Do

```tsx
const safe = RegExp.escape(input);
```
