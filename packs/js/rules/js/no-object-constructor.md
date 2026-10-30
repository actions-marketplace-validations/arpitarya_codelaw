---
id: js/no-object-constructor
title: "{} rather than new Object()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`{}` is equivalent unless `Object` is shadowed; review it"
stability: preview
ported_from: [eslint/no-object-constructor]
since: 0.5.0
---

An empty object is written `{}`, not `Object()` or `new Object()`.

**Why:** The literal is shorter and cannot be redirected by a shadowed `Object`.

```codelaw
match:
  any:
    - pattern: new $O()
    - pattern: $O()
constraints:
  O: { kind: identifier, regex: '^Object$' }
where:
  $O: builtin
message: "Use `{}` instead of the `Object` constructor"
fix: "({})"
```

## Examples

### JavaScript

#### Don't

```js
const o = new Object();
```

#### Do

```js
const o = {};
```

#### Fixed

```js
const o = ({});
```

### JSX

#### Don't

```jsx
const o = new Object();
```

#### Do

```jsx
const o = {};
```

#### Fixed

```jsx
const o = ({});
```

### TypeScript

#### Don't

```ts
const o = new Object();
```

#### Do

```ts
const o = {};
```

#### Fixed

```ts
const o = ({});
```

### TSX

#### Don't

```tsx
const o = new Object();
```

#### Do

```tsx
const o = {};
```

#### Fixed

```tsx
const o = ({});
```
