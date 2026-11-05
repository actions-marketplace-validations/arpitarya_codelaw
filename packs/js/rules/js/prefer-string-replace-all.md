---
id: js/prefer-string-replace-all
title: "replaceAll() rather than replace with a /g regex"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-string-replace-all]
since: 0.5.0
---

`.replace()` with a regex whose only flag is `g` is written `.replaceAll()`. Upstream also turns a plain regex into a string; this port keeps the regex.

**Why:** `replaceAll` says "every occurrence" in its name instead of in a flag.

```codelaw

match:
  pattern: $S.replace($R, $T)
constraints:
  R: { kind: regex, has: { kind: regex_flags, regex: '^g$' } }
message: "Use `.replaceAll()` for a global replace"
fix: "$S.replaceAll($R, $T)"
```

## Examples

### JavaScript

#### Don't

```js
const out = text.replace(/,/g, ";");
```

#### Do

```js
const out = text.replaceAll(/,/g, ";");
```

### JSX

#### Don't

```jsx
const out = text.replace(/,/g, ";");
```

#### Do

```jsx
const out = text.replaceAll(/,/g, ";");
```

### TypeScript

#### Don't

```ts
const out = text.replace(/,/g, ";");
```

#### Do

```ts
const out = text.replaceAll(/,/g, ";");
```

### TSX

#### Don't

```tsx
const out = text.replace(/,/g, ";");
```

#### Do

```tsx
const out = text.replaceAll(/,/g, ";");
```
