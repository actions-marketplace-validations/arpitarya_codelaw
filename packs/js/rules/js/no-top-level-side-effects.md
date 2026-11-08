---
id: js/no-top-level-side-effects
title: "No changing a global when a module is imported"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the change has to move into a function the importer calls, which changes the module's API"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-top-level-side-effects]
since: 0.5.0
---

In a module with a runtime export (not `export {}` or type-only exports) and no `#!` line, code outside any function — the top level, conditionals, computed class keys, static blocks — does not assign, update or `delete` something rooted in a language global (`globalThis.app = …`), and does not pass one to `Object.assign`, `Object.defineProperty`, `Object.freeze`, … or `Reflect.set`, `Reflect.defineProperty`, …. Only the language's own globals count: environment globals (`window`, `process`) are not known here, and a local that shadows a global is ignored. ESLint's `checkUnknownCalls` mode is not taken.

**Why:** Importing a module for one export should not change the program for everyone else; such changes depend on import order and break tree-shaking.

```codelaw
native: js/no-top-level-side-effects
```

## How to fix

1. Export a function (`install()`) that makes the change, and call it where the change is wanted.

## Examples

### JavaScript

#### Don't

```js
export const version = 1;
globalThis.app = { version };
```

#### Do

```js
export const version = 1;
export function install() {
  globalThis.app = { version };
}
```


### JSX

#### Don't

```jsx
export const version = 1;
globalThis.app = { version };
```

#### Do

```jsx
export const version = 1;
export function install() {
  globalThis.app = { version };
}
```


### TypeScript

#### Don't

```ts
export const version = 1;
globalThis.app = { version };
```

#### Do

```ts
export const version = 1;
export function install() {
  globalThis.app = { version };
}
```


### TSX

#### Don't

```tsx
export const version = 1;
globalThis.app = { version };
```

#### Do

```tsx
export const version = 1;
export function install() {
  globalThis.app = { version };
}
```
