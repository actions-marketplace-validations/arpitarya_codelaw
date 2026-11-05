---
id: js/no-os-command-from-path
title: "Run commands by absolute path"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the absolute path differs between machines"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/no-os-command-from-path]
since: 0.5.0
---

A child process is not started with a bare command name.

**Why:** Whoever controls `PATH` (an environment variable, a writable directory on it) decides which program runs.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: (^|\.)(exec|execSync|execFile|execFileSync|spawn|spawnSync)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        kind: string
        regex: ^.[a-zA-Z]
message: The command is looked up on `PATH`, which an attacker may control; use its absolute path
```

## How to fix

1. Pass the absolute path of the program, or resolve it once from a trusted configuration.

## Examples

### JavaScript

#### Don't

```js
execFileSync("git", ["status"]);
```

#### Do

```js
execFileSync("/usr/bin/git", ["status"]);
```

### JSX

#### Don't

```jsx
execFileSync("git", ["status"]);
```

#### Do

```jsx
execFileSync("/usr/bin/git", ["status"]);
```

### TypeScript

#### Don't

```ts
execFileSync("git", ["status"]);
```

#### Do

```ts
execFileSync("/usr/bin/git", ["status"]);
```

### TSX

#### Don't

```tsx
execFileSync("git", ["status"]);
```

#### Do

```tsx
execFileSync("/usr/bin/git", ["status"]);
```
