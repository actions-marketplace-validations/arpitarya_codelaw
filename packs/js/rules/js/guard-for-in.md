---
id: js/guard-for-in
title: "for-in is filtered"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "looping over the own keys or guarding with `Object.hasOwn` both drop inherited keys; the author picks the form"
stability: preview
ported_from: [eslint/guard-for-in]
since: 0.5.0
---

The body of a `for…in` loop starts with an `if` that filters the keys.

**Why:** `for…in` also visits inherited enumerable properties.

```codelaw
match:
  kind: for_in_statement
  all:
  - has: {field: operator, regex: ^in$}
  - has:
      field: body
      not:
        any:
        - {kind: if_statement}
        - kind: statement_block
          has: {kind: if_statement, nthChild: 1}
        - {kind: statement_block, regex: '^\{\s*\}$'}
  - any:
    - { pattern: "for (const $K in $O) $B", all: [{ pattern: $CONST }] }
    - { pattern: "for (let $K in $O) $B", all: [{ pattern: $LET }] }
    - { pattern: "for (var $K in $O) $B", all: [{ pattern: $VAR }] }
    - { pattern: "for ($K in $O) $B", all: [{ pattern: $BARE }] }
message: Filter a `for…in` loop with an `if`, or loop over `Object.keys`
fix:
  choices:
    - label: loop over the own keys
      default: true
      fix:
        - replace: { target: $CONST, with: "for (const $K of Object.keys($O)) $B" }
        - replace: { target: $LET, with: "for (let $K of Object.keys($O)) $B" }
        - replace: { target: $VAR, with: "for (var $K of Object.keys($O)) $B" }
        - replace: { target: $BARE, with: "for ($K of Object.keys($O)) $B" }
    - label: guard with hasOwn
      fix:
        - replace: { target: $B, with: "{\n  if (Object.hasOwn($O, $K)) $B\n}" }
```

## How to fix

1. Loop over `Object.keys(obj)` instead, or start the body with `if (!Object.hasOwn(obj, key)) continue;`.

## Examples

### JavaScript

#### Don't

```js
for (const key in obj) {
  use(obj[key]);
}
```

#### Fixed (loop over the own keys)

```js
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

#### Fixed (guard with hasOwn)

```js
for (const key in obj) {
  if (Object.hasOwn(obj, key)) {
  use(obj[key]);
}
}
```

#### Do

```js
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

### JSX

#### Don't

```jsx
for (const key in obj) {
  use(obj[key]);
}
```

#### Fixed (loop over the own keys)

```jsx
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

#### Fixed (guard with hasOwn)

```jsx
for (const key in obj) {
  if (Object.hasOwn(obj, key)) {
  use(obj[key]);
}
}
```

#### Do

```jsx
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

### TypeScript

#### Don't

```ts
for (const key in obj) {
  use(obj[key]);
}
```

#### Fixed (loop over the own keys)

```ts
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

#### Fixed (guard with hasOwn)

```ts
for (const key in obj) {
  if (Object.hasOwn(obj, key)) {
  use(obj[key]);
}
}
```

#### Do

```ts
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

### TSX

#### Don't

```tsx
for (const key in obj) {
  use(obj[key]);
}
```

#### Fixed (loop over the own keys)

```tsx
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

#### Fixed (guard with hasOwn)

```tsx
for (const key in obj) {
  if (Object.hasOwn(obj, key)) {
  use(obj[key]);
}
}
```

#### Do

```tsx
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```
