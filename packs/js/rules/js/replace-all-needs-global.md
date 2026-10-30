---
id: js/replace-all-needs-global
title: "replaceAll() needs a global regex"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the call throws today; adding the `g` flag makes it run"
stability: preview
ported_from: [oxlint/bad-replace-all-arg]
since: 0.5.0
---

A regular-expression literal passed to `replaceAll()` has the `g` flag.

**Why:** `replaceAll()` throws a TypeError when given a regular expression without the `g` flag.

```codelaw
match:
  pattern: $S.replaceAll($R, $X)
constraints:
  R: { kind: regex, not: { has: { kind: regex_flags, regex: g } } }
message: "`replaceAll()` throws on a regex without the `g` flag; add it"
fix:
  - insert-after: { target: $R, text: "g" }
```

## Examples

### JavaScript

#### Don't

```js
const out = text.replaceAll(/\s+/i, " ");
```

#### Do

```js
const out = text.replaceAll(/\s+/ig, " ");
```

### JSX

#### Don't

```jsx
const out = text.replaceAll(/\s+/i, " ");
```

#### Do

```jsx
const out = text.replaceAll(/\s+/ig, " ");
```

### TypeScript

#### Don't

```ts
const out = text.replaceAll(/\s+/i, " ");
```

#### Do

```ts
const out = text.replaceAll(/\s+/ig, " ");
```

### TSX

#### Don't

```tsx
const out = text.replaceAll(/\s+/i, " ");
```

#### Do

```tsx
const out = text.replaceAll(/\s+/ig, " ");
```
