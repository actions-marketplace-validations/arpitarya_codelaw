---
id: node/no-process-exit
title: "No process.exit()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "code after the call now runs and open handles keep the process alive, so the reviewer picks setting the exit code or throwing (both offered for a call with an exit code)"
stability: preview
ported_from: [eslint-plugin-n/no-process-exit, eslint-plugin-unicorn/no-process-exit]
since: 0.1.0
---

Code doesn't call `process.exit()`.

**Why:** `process.exit()` ends the process at once, dropping pending writes and skipping cleanup;
libraries that call it take that choice away from the program.

```codelaw
match:
  pattern: process.exit($$$ARGS)
message: "Don't end the process from here; throw, or set `process.exitCode`"
fix:
  choices:
    - label: set exit code
      default: true
      fix: "process.exitCode = $$$ARGS"
    - label: throw
      fix: 'throw new Error("process exit " + $$$ARGS)'
```

## How to fix

1. Set `process.exitCode` and let the program finish, or throw an error the caller handles.

## Examples

### JavaScript

#### Don't

```js
if (failed) {
  process.exit(1);
}
```

#### Fixed (set exit code)

```js
if (failed) {
  process.exitCode = 1;
}
```

#### Fixed (throw)

```js
if (failed) {
  throw new Error("process exit " + 1);
}
```

#### Do

```js
if (failed) {
  process.exitCode = 1;
}
```

### JSX

#### Don't

```jsx
if (failed) {
  process.exit(1);
}
```

#### Fixed (set exit code)

```jsx
if (failed) {
  process.exitCode = 1;
}
```

#### Fixed (throw)

```jsx
if (failed) {
  throw new Error("process exit " + 1);
}
```

#### Do

```jsx
if (failed) {
  process.exitCode = 1;
}
```

### TypeScript

#### Don't

```ts
if (failed) {
  process.exit(1);
}
```

#### Fixed (set exit code)

```ts
if (failed) {
  process.exitCode = 1;
}
```

#### Fixed (throw)

```ts
if (failed) {
  throw new Error("process exit " + 1);
}
```

#### Do

```ts
if (failed) {
  process.exitCode = 1;
}
```

### TSX

#### Don't

```tsx
if (failed) {
  process.exit(1);
}
```

#### Fixed (set exit code)

```tsx
if (failed) {
  process.exitCode = 1;
}
```

#### Fixed (throw)

```tsx
if (failed) {
  throw new Error("process exit " + 1);
}
```

#### Do

```tsx
if (failed) {
  process.exitCode = 1;
}
```
