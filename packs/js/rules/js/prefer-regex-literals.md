---
id: js/prefer-regex-literals
title: "A regex literal rather than new RegExp(\"…\")"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the literal is created once per evaluation of the enclosing code, a `RegExp` call once per call; with the `g` flag `lastIndex` then differs"
stability: preview
ported_from: [eslint/prefer-regex-literals]
since: 0.5.0
---

`new RegExp` and `RegExp` are not called with a single string literal (one without a backslash or slash, so it can be copied as is). With flags, the call is reported but not fixed.

**Why:** A literal is checked when the file parses, and needs no double escaping.

```codelaw

match:
  any:
    - pattern: new RegExp($S)
    - pattern: RegExp($S)
  has: { field: arguments, has: { nthChild: 1, kind: string, not: { regex: '\\\\' }, has: { kind: string_fragment, not: { regex: '/' }, pattern: $P } } }
message: "Write the pattern as a literal: `/$P/`"
fix: "/$P/"
```

## Examples

### JavaScript

#### Don't

```js
const word = new RegExp("abc+");
```

#### Do

```js
const word = /abc+/;
```

### JSX

#### Don't

```jsx
const word = new RegExp("abc+");
```

#### Do

```jsx
const word = /abc+/;
```

### TypeScript

#### Don't

```ts
const word = new RegExp("abc+");
```

#### Do

```ts
const word = /abc+/;
```

### TSX

#### Don't

```tsx
const word = new RegExp("abc+");
```

#### Do

```tsx
const word = /abc+/;
```
