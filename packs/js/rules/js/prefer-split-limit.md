---
id: js/prefer-split-limit
title: "A limit on split() when only the first parts are read"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-split-limit]
since: 0.5.0
---

A `split` whose result is read at one small index, or destructured into one name, passes the limit it needs: `s.split(",")[0]` is `s.split(",", 1)[0]`.

**Why:** Without a limit `split` cuts the whole string, however long, to return parts nobody reads.

```codelaw
match:
  any:
    - pattern: "$S.split($D)[$I]"
      all: [{ pattern: $IDX }]
    - pattern: "const [$A] = $S.split($D)"
      all: [{ pattern: $ONE }]
    - pattern: "let [$A] = $S.split($D)"
      all: [{ pattern: $ONE }]
constraints:
  I: { regex: '^[0-4]$' }
  D: { any: [{ kind: string }, { kind: regex }] }
  A: { kind: identifier }
message: "Pass the limit to `split`"
fix:
  - replace: { target: $IDX, with: "$S.split($D, $I|map(0:1,1:2,2:3,3:4,4:5))[$I]" }
  - edit-text: { target: $ONE, pattern: '\.split\((.*)\)(;?)$', replace: '.split(${1}, 1)${2}' }
```

## Examples

### JavaScript

#### Don't

```js
const scheme = url.split(":")[0];
```

#### Do

```js
const scheme = url.split(":", 1)[0];
```


### JSX

#### Don't

```jsx
const scheme = url.split(":")[0];
```

#### Do

```jsx
const scheme = url.split(":", 1)[0];
```


### TypeScript

#### Don't

```ts
const scheme = url.split(":")[0];
```

#### Do

```ts
const scheme = url.split(":", 1)[0];
```


### TSX

#### Don't

```tsx
const scheme = url.split(":")[0];
```

#### Do

```tsx
const scheme = url.split(":", 1)[0];
```
