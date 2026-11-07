---
id: js/prefer-global-this
title: "globalThis rather than global"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-global-this]
since: 0.5.0
---

The global object is named `globalThis`, not Node's `global`. Upstream also rewrites `window` and `self` outside their window- and worker-specific properties; this port covers `global`, which is `globalThis` wherever it exists.

**Why:** `globalThis` is the same object in every environment; `global` exists only in Node.

```codelaw
match:
  kind: identifier
  regex: '^global$'
where:
  $$MATCH: undeclared
message: "Use `globalThis`"
fix: "globalThis"
```

## Examples

### JavaScript

#### Don't

```js
global.cache = new Map();
```

#### Do

```js
globalThis.cache = new Map();
```


### JSX

#### Don't

```jsx
global.cache = new Map();
```

#### Do

```jsx
globalThis.cache = new Map();
```


### TypeScript

#### Don't

```ts
global.cache = new Map();
```

#### Do

```ts
globalThis.cache = new Map();
```


### TSX

#### Don't

```tsx
global.cache = new Map();
```

#### Do

```tsx
globalThis.cache = new Map();
```
