---
id: js/no-useless-string-raw
title: "No String.raw without an escape"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "without a backslash the cooked and raw strings are identical, so dropping the tag changes nothing"
stability: preview
ported_from: [biome/complexity/noUselessStringRaw]
since: 0.5.0
---

`String.raw` is used only on a template that holds a backslash.

**Why:** Without a backslash, `String.raw` returns the same string as a plain template, and the tag only makes the reader look for an escape that is not there.

```codelaw
match:
  kind: call_expression
  all:
  - has: {field: function, regex: ^String\.raw$}
  - has:
      field: arguments
      kind: template_string
      pattern: $T
      not: {regex: \\}
message: '`String.raw` has no backslash to keep here; use a plain template'
fix: "$T"
```

## How to fix

1. Remove `String.raw` and keep the template.

## Examples

### JavaScript

#### Don't

```js
const p = String.raw`/usr/local`;
```

#### Fixed

```js
const p = `/usr/local`;
```

#### Do

```js
const p = `/usr/local`;
```

### JSX

#### Don't

```jsx
const p = String.raw`/usr/local`;
```

#### Fixed

```jsx
const p = `/usr/local`;
```

#### Do

```jsx
const p = `/usr/local`;
```

### TypeScript

#### Don't

```ts
const p = String.raw`/usr/local`;
```

#### Fixed

```ts
const p = `/usr/local`;
```

#### Do

```ts
const p = `/usr/local`;
```

### TSX

#### Don't

```tsx
const p = String.raw`/usr/local`;
```

#### Fixed

```tsx
const p = `/usr/local`;
```

#### Do

```tsx
const p = `/usr/local`;
```
