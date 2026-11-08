---
id: js/prefer-array-from-async
title: "await Array.fromAsync() rather than a for await loop that only pushes"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-array-from-async]
since: 0.5.0
---

An empty array declared and then filled by a `for await…of` loop whose only statement pushes the loop variable is `await Array.fromAsync(source)`. The declaration is directly before the loop.

**Why:** The built-in names the operation; the loop makes the reader see that it only collects.

```codelaw
match:
  kind: lexical_declaration
  any:
    - pattern: "const $R = [];"
    - pattern: "let $R = [];"
  precedes:
    all:
      - pattern: $LOOP
      - any:
          - pattern: "for await (const $X of $I) { $R.push($X); }"
          - pattern: "for await (const $X of $I) $R.push($X);"
  has: { kind: variable_declarator, has: { field: value, pattern: $INIT } }
message: "Collect with `await Array.fromAsync($I)`"
fix:
  - replace: { target: $INIT, with: "await Array.fromAsync($I)" }
  - remove: $LOOP
```

## Examples

### JavaScript

#### Don't

```js
const rows = [];
for await (const row of cursor) {
  rows.push(row);
}
```

#### Do

```js
const rows = await Array.fromAsync(cursor);
```


### JSX

#### Don't

```jsx
const rows = [];
for await (const row of cursor) {
  rows.push(row);
}
```

#### Do

```jsx
const rows = await Array.fromAsync(cursor);
```


### TypeScript

#### Don't

```ts
const rows = [];
for await (const row of cursor) {
  rows.push(row);
}
```

#### Do

```ts
const rows = await Array.fromAsync(cursor);
```


### TSX

#### Don't

```tsx
const rows = [];
for await (const row of cursor) {
  rows.push(row);
}
```

#### Do

```tsx
const rows = await Array.fromAsync(cursor);
```
