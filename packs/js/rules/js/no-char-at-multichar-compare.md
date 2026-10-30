---
id: js/no-char-at-multichar-compare
title: "charAt() compared with a longer string"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the code meant `startsWith`, a `slice` or a one-character string is the author's; the code is wrong in each case differently"
stability: preview
ported_from: [oxlint/bad-char-at-comparison]
since: 0.5.0
---

The result of `charAt()` is compared only with one-character strings. A comparison with `""` (the out-of-range check) is fine.

**Why:** `charAt()` returns one character, so comparing it with a longer string is always false (or always true with `!==`).

```codelaw
match:
  any:
    - pattern: $S.charAt($I) === $STR
    - pattern: $STR === $S.charAt($I)
    - pattern: $S.charAt($I) !== $STR
    - pattern: $STR !== $S.charAt($I)
    - pattern: $S.charAt($I) == $STR
    - pattern: $STR == $S.charAt($I)
    - pattern: $S.charAt($I) != $STR
    - pattern: $STR != $S.charAt($I)
constraints:
  STR: { kind: string, regex: '^("((?:[^"\\]|\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\u\{[0-9a-fA-F]+\}|\\[^ux])){2,}"|''((?:[^''\\]|\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\u\{[0-9a-fA-F]+\}|\\[^ux])){2,}'')$' }
message: "`charAt()` returns one character and is never equal to $STR"
```

## How to fix

1. To test a prefix at a position, use `$S.startsWith(str, i)`.
2. To compare one character, compare with that one character.

## Examples

### JavaScript

#### Don't

```js
if (line.charAt(0) === "//") {
  skip();
}
```

#### Do

```js
if (line.startsWith("//", 0)) {
  skip();
}
```

### JSX

#### Don't

```jsx
if (line.charAt(0) === "//") {
  skip();
}
```

#### Do

```jsx
if (line.startsWith("//", 0)) {
  skip();
}
```

### TypeScript

#### Don't

```ts
if (line.charAt(0) === "//") {
  skip();
}
```

#### Do

```ts
if (line.startsWith("//", 0)) {
  skip();
}
```

### TSX

#### Don't

```tsx
if (line.charAt(0) === "//") {
  skip();
}
```

#### Do

```tsx
if (line.startsWith("//", 0)) {
  skip();
}
```
