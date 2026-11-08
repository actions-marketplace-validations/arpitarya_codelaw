---
id: js/iteration-fallback-style
title: "Guard an optional loop source instead of iterating an empty fallback"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/iteration-fallback-style]
since: 0.5.0
---

A `for…of` loop over `items || []` (or `items ?? []`) is an `if (items)` around a loop over `items`: the guard skips the loop without making an empty array. The same holds for `Object.keys`, `values` and `entries` of `x || {}`. This is upstream's default style (`guard`); the source is a name or a property path, so reading it twice is safe.

**Why:** The guard says the source may be missing; the fallback hides that inside the loop header.

```codelaw
utils:
  path:
    any:
      - kind: identifier
      - kind: member_expression
        not: { has: { stopBy: end, any: [{ kind: call_expression }, { kind: optional_chain }] } }
match:
  kind: for_in_statement
  not: { regex: '^for\s+await' }
  any:
    - all:
        - any:
            - pattern: "for (const $X of $A || []) $B"
            - pattern: "for (const $X of $A ?? []) $B"
        - pattern: $W1
    - all:
        - any:
            - pattern: "for (const $Y of Object.$M($C || {})) $D"
            - pattern: "for (const $Y of Object.$M($C ?? {})) $D"
        - pattern: $W2
constraints:
  A: { matches: path }
  C: { matches: path }
  M: { regex: '^(keys|values|entries)$' }
message: "Guard the loop with `if` instead of a fallback"
fix:
  - replace: { target: $W1, with: "if ($A) {\nfor (const $X of $A) $B\n}" }
  - replace: { target: $W2, with: "if ($C) {\nfor (const $Y of Object.$M($C)) $D\n}" }
```

## Examples

### JavaScript

#### Don't

```js
for (const item of items || []) {
  use(item);
}
```

#### Do

```js
if (items) {
  for (const item of items) {
    use(item);
  }
}
```

#### Fixed

```js
if (items) {
for (const item of items) {
  use(item);
}
}
```


### JSX

#### Don't

```jsx
for (const item of items || []) {
  use(item);
}
```

#### Do

```jsx
if (items) {
  for (const item of items) {
    use(item);
  }
}
```

#### Fixed

```jsx
if (items) {
for (const item of items) {
  use(item);
}
}
```


### TypeScript

#### Don't

```ts
for (const item of items || []) {
  use(item);
}
```

#### Do

```ts
if (items) {
  for (const item of items) {
    use(item);
  }
}
```

#### Fixed

```ts
if (items) {
for (const item of items) {
  use(item);
}
}
```


### TSX

#### Don't

```tsx
for (const item of items || []) {
  use(item);
}
```

#### Do

```tsx
if (items) {
  for (const item of items) {
    use(item);
  }
}
```

#### Fixed

```tsx
if (items) {
for (const item of items) {
  use(item);
}
}
```
