---
id: js/jsdoc-ts-method-signature-style
title: "Function properties, not method signatures, in JSDoc object types"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the two forms are checked differently by TypeScript (a method's parameters bivariantly, a function property's strictly), so switching one changes what type-checks"
options: { style: { type: "string", default: "property", description: "which form to ask for: property or method" }, enable_fixer: { type: "boolean", default: true, description: "offer the rewrite" } }
stability: preview
ported_from: [eslint-plugin-jsdoc/ts-method-signature-style]
since: 0.5.0
---

An object type in a tag's `{type}` writes a member that is a function as a property — `{run: (a: string) => void}` — and not as a method signature `{run(a: string): void}` (the plugin's default; the `style` option set to `"method"` asks for the opposite). The fix rewrites the member; a method that is overloaded (the same name twice), or has no return type, is reported without one.

**Why:** The two forms differ in TypeScript: a method signature's parameters are checked bivariantly, a function property's strictly, so one form throughout is the safer one.

```codelaw
native: js/jsdoc-ts-method-signature-style
```

## How to fix

1. Write the member as `name: (params) => return` (the fix does).

## Examples

### JavaScript

#### Don't

```js
/**
 * @param {{run(job: string): void}} worker
 */
function start(worker) {}
```

#### Do

```js
/**
 * @param {{run: (job: string) => void}} worker
 */
function start(worker) {}
```

### JSX

#### Don't

```jsx
/**
 * @param {{run(job: string): void}} worker
 */
function start(worker) {}
```

#### Do

```jsx
/**
 * @param {{run: (job: string) => void}} worker
 */
function start(worker) {}
```

### TypeScript

#### Don't

```ts
/**
 * @param {{run(job: string): void}} worker
 */
function start(worker) {}
```

#### Do

```ts
/**
 * @param {{run: (job: string) => void}} worker
 */
function start(worker) {}
```

### TSX

#### Don't

```tsx
/**
 * @param {{run(job: string): void}} worker
 */
function start(worker) {}
```

#### Do

```tsx
/**
 * @param {{run: (job: string) => void}} worker
 */
function start(worker) {}
```
