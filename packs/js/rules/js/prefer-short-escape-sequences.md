---
id: js/prefer-short-escape-sequences
title: "Short escape rather than a hex or Unicode one for a control character"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-short-escape-sequences]
since: 0.5.0
---

A control character that has a short escape is written with it: `\n`, not `\x0A`, `\u000A` or `\u{A}` (and likewise `\t`, `\r`, `\b`, `\f`, `\v`).

**Why:** Everyone reads `\n`; few read `\x0A` at a glance.

```codelaw
match:
  kind: string
  regex: '(^|[^\\])(\\\\)*\\(x0[89a-dA-D]|u000[89a-dA-D]|u\{0*[89a-dA-D]\})'
  not: { inside: { kind: jsx_attribute } }
message: "Use the short escape"
fix:
  - edit-text: { pattern: '((?:^|[^\\])(?:\\\\)*)\\(?:x0[aA]|u000[aA]|u\{0*[aA]\})', replace: '${1}\n' }
  - edit-text: { pattern: '((?:^|[^\\])(?:\\\\)*)\\(?:x0[aA]|u000[aA]|u\{0*[aA]\})', replace: '${1}\n' }
  - edit-text: { pattern: '((?:^|[^\\])(?:\\\\)*)\\(?:x09|u0009|u\{0*9\})', replace: '${1}\t' }
  - edit-text: { pattern: '((?:^|[^\\])(?:\\\\)*)\\(?:x09|u0009|u\{0*9\})', replace: '${1}\t' }
  - edit-text: { pattern: '((?:^|[^\\])(?:\\\\)*)\\(?:x0[dD]|u000[dD]|u\{0*[dD]\})', replace: '${1}\r' }
  - edit-text: { pattern: '((?:^|[^\\])(?:\\\\)*)\\(?:x0[dD]|u000[dD]|u\{0*[dD]\})', replace: '${1}\r' }
  - edit-text: { pattern: '((?:^|[^\\])(?:\\\\)*)\\(?:x08|u0008|u\{0*8\})', replace: '${1}\b' }
  - edit-text: { pattern: '((?:^|[^\\])(?:\\\\)*)\\(?:x0[cC]|u000[cC]|u\{0*[cC]\})', replace: '${1}\f' }
  - edit-text: { pattern: '((?:^|[^\\])(?:\\\\)*)\\(?:x0[bB]|u000[bB]|u\{0*[bB]\})', replace: '${1}\v' }
```

## Examples

### JavaScript

#### Don't

```js
const line = "done\x0A";
const cell = "a\u0009b";
```

#### Do

```js
const line = "done\n";
const cell = "a\tb";
```


### JSX

#### Don't

```jsx
const line = "done\x0A";
const cell = "a\u0009b";
```

#### Do

```jsx
const line = "done\n";
const cell = "a\tb";
```


### TypeScript

#### Don't

```ts
const line = "done\x0A";
const cell = "a\u0009b";
```

#### Do

```ts
const line = "done\n";
const cell = "a\tb";
```


### TSX

#### Don't

```tsx
const line = "done\x0A";
const cell = "a\u0009b";
```

#### Do

```tsx
const line = "done\n";
const cell = "a\tb";
```
