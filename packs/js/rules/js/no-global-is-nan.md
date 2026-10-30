---
id: js/no-global-is-nan
title: "Number.isNaN, not the global isNaN"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Number.isNaN` does not convert its argument, so `'abc'` now gives `false`; review the call"
stability: preview
ported_from: [biome/suspicious/noGlobalIsNan]
since: 0.5.0
---

`Number.isNaN` is used instead of the global `isNaN`.

**Why:** The global converts its argument first, so `isNaN('abc')` is `true`; `Number.isNaN` is true only for `NaN` itself.

```codelaw
match:
  pattern: $F($$$ARGS)
constraints:
  F: { kind: identifier, regex: '^isNaN$' }
where:
  $F: builtin
message: "Use `Number.isNaN` instead of the global `isNaN`"
fix: "Number.isNaN($$$ARGS)"
```

## Examples

### JavaScript

#### Don't

```js
if (isNaN(n)) fail();
```

#### Do

```js
if (Number.isNaN(n)) fail();
```

### JSX

#### Don't

```jsx
if (isNaN(n)) fail();
```

#### Do

```jsx
if (Number.isNaN(n)) fail();
```

### TypeScript

#### Don't

```ts
if (isNaN(n)) fail();
```

#### Do

```ts
if (Number.isNaN(n)) fail();
```

### TSX

#### Don't

```tsx
if (isNaN(n)) fail();
```

#### Do

```tsx
if (Number.isNaN(n)) fail();
```
