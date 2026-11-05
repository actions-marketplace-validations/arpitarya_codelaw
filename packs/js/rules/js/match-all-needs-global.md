---
id: js/match-all-needs-global
title: "matchAll() needs a global regex"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the call throws today; adding the `g` flag makes it run"
stability: preview
ported_from: [oxlint/bad-match-all-arg]
since: 0.5.0
---

A regular-expression literal passed to `matchAll()` has the `g` flag.

**Why:** `matchAll()` throws a TypeError when given a regular expression without the `g` flag.

```codelaw
match:
  pattern: $S.matchAll($R)
constraints:
  R: { kind: regex, not: { has: { kind: regex_flags, regex: g } } }
message: "`matchAll()` throws on a regex without the `g` flag; add it"
fix:
  - insert-after: { target: $R, text: "g" }
```

## Examples

### JavaScript

#### Don't

```js
for (const m of text.matchAll(/\d+/)) {
  use(m);
}
```

#### Do

```js
for (const m of text.matchAll(/\d+/g)) {
  use(m);
}
```

### JSX

#### Don't

```jsx
for (const m of text.matchAll(/\d+/)) {
  use(m);
}
```

#### Do

```jsx
for (const m of text.matchAll(/\d+/g)) {
  use(m);
}
```

### TypeScript

#### Don't

```ts
for (const m of text.matchAll(/\d+/)) {
  use(m);
}
```

#### Do

```ts
for (const m of text.matchAll(/\d+/g)) {
  use(m);
}
```

### TSX

#### Don't

```tsx
for (const m of text.matchAll(/\d+/)) {
  use(m);
}
```

#### Do

```tsx
for (const m of text.matchAll(/\d+/g)) {
  use(m);
}
```
