---
id: js/no-global-is-finite
title: "Number.isFinite, not the global isFinite"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Number.isFinite` does not convert its argument, so a numeric string now gives `false`; review the call"
stability: preview
ported_from: [biome/suspicious/noGlobalIsFinite]
since: 0.5.0
---

`Number.isFinite` is used instead of the global `isFinite`.

**Why:** The global converts its argument first, so `isFinite('12')` is `true`; `Number.isFinite` checks the value as it is.

```codelaw
match:
  pattern: $F($$$ARGS)
constraints:
  F: { kind: identifier, regex: '^isFinite$' }
where:
  $F: builtin
message: "Use `Number.isFinite` instead of the global `isFinite`"
fix: "Number.isFinite($$$ARGS)"
```

## Examples

### JavaScript

#### Don't

```js
if (isFinite(n)) use(n);
```

#### Do

```js
if (Number.isFinite(n)) use(n);
```

### JSX

#### Don't

```jsx
if (isFinite(n)) use(n);
```

#### Do

```jsx
if (Number.isFinite(n)) use(n);
```

### TypeScript

#### Don't

```ts
if (isFinite(n)) use(n);
```

#### Do

```ts
if (Number.isFinite(n)) use(n);
```

### TSX

#### Don't

```tsx
if (isFinite(n)) use(n);
```

#### Do

```tsx
if (Number.isFinite(n)) use(n);
```
