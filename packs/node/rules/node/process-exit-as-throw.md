---
id: node/process-exit-as-throw
title: "Nothing runs after process.exit()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "deleting the code is right only when it was really dead, not misplaced"
stability: preview
ported_from: [eslint-plugin-n/process-exit-as-throw]
since: 0.5.0
---

Statements after a `process.exit(…)` call in the same block are reported, because the call ends the process as a `throw` ends a function. eslint-plugin-n's rule reports nothing itself: it makes ESLint's code-path rules (`no-unreachable`, `consistent-return`, `no-fallthrough`, …) read `process.exit()` as a `throw`. codelaw has no engine-wide code-path graph, so this rule carries the one consequence that stands alone in a file — what follows the call never runs. A block that already ends in a `return`, `throw`, `break` or `continue` before the call is `js/no-unreachable`'s; a file that makes its own `process` is skipped; hoisted function declarations are allowed, as there.

**Why:** Code written after a call that never returns misleads every reader who assumes it does.

```codelaw
native: node/process-exit-as-throw
```

## Examples

### JavaScript

#### Don't

```js
function stop(code) {
  process.exit(code);
  cleanup();
}
```

#### Do

```js
function stop(code) {
  process.exit(code);
}
```

#### Fixed

```js
function stop(code) {
  process.exit(code);
}
```

### JSX

#### Don't

```jsx
function stop(code) {
  process.exit(code);
  cleanup();
}
```

#### Do

```jsx
function stop(code) {
  process.exit(code);
}
```

#### Fixed

```jsx
function stop(code) {
  process.exit(code);
}
```

### TypeScript

#### Don't

```ts
function stop(code: number) {
  process.exit(code);
  cleanup();
}
```

#### Do

```ts
function stop(code: number) {
  process.exit(code);
}
```

#### Fixed

```ts
function stop(code: number) {
  process.exit(code);
}
```

### TSX

#### Don't

```tsx
function stop(code: number) {
  process.exit(code);
  cleanup();
}
```

#### Do

```tsx
function stop(code: number) {
  process.exit(code);
}
```

#### Fixed

```tsx
function stop(code: number) {
  process.exit(code);
}
```
