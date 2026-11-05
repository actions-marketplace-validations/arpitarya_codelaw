---
id: js/no-char-at-multichar-compare
title: "charAt() compared with a longer string"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`startsWith` is one reading of the comparison; a `slice` or a one-character string is another, so the author confirms"
stability: preview
ported_from: [oxlint/bad-char-at-comparison, eslint-plugin-unicorn/no-invalid-character-comparison]
since: 0.5.0
---

The result of `charAt()` is compared only with one-character strings. A comparison with `""` (the out-of-range check) is fine.

**Why:** `charAt()` returns one character, so comparing it with a longer string is always false (or always true with `!==`).

```codelaw
match:
  any:
    - all:
        - { pattern: $POS }
        - any:
            - pattern: $S.charAt($I) === $STR
            - pattern: $STR === $S.charAt($I)
            - pattern: $S.charAt($I) == $STR
            - pattern: $STR == $S.charAt($I)
    - all:
        - { pattern: $NEG }
        - any:
            - pattern: $S.charAt($I) !== $STR
            - pattern: $STR !== $S.charAt($I)
            - pattern: $S.charAt($I) != $STR
            - pattern: $STR != $S.charAt($I)
constraints:
  STR: { kind: string, regex: '^("((?:[^"\\]|\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\u\{[0-9a-fA-F]+\}|\\[^ux])){2,}"|''((?:[^''\\]|\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\u\{[0-9a-fA-F]+\}|\\[^ux])){2,}'')$' }
message: "`charAt()` returns one character and is never equal to $STR"
fix:
  - replace: { target: $POS, with: "$S.startsWith($STR, $I)" }
  - replace: { target: $NEG, with: "!$S.startsWith($STR, $I)" }
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

#### Fixed

```js
if (line.startsWith("//", 0)) {
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

#### Fixed

```jsx
if (line.startsWith("//", 0)) {
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

#### Fixed

```ts
if (line.startsWith("//", 0)) {
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

#### Fixed

```tsx
if (line.startsWith("//", 0)) {
  skip();
}
```

#### Do

```tsx
if (line.startsWith("//", 0)) {
  skip();
}
```
