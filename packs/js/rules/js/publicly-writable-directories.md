---
id: js/publicly-writable-directories
title: "No files in /tmp"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the private directory's lifetime is the author's choice"
stability: preview
ported_from: [eslint-plugin-sonarjs/publicly-writable-directories]
since: 0.5.0
---

A string does not name a file directly in `/tmp`, `/var/tmp`, `/dev/shm` or `/usr/tmp`.

**Why:** Other users can predict the name, read the file, or plant a symlink there first.

```codelaw

match:
  kind: string_fragment
  regex: '^(/tmp|/var/tmp|/dev/shm|/usr/tmp)(/|$)'
message: "A shared temp directory can be read or raced by other users; use `fs.mkdtemp` under `os.tmpdir()`"
```

## How to fix

1. Create a private directory with `fs.mkdtemp(path.join(os.tmpdir(), "prefix-"))` and write inside it.

## Examples

### JavaScript

#### Don't

```js
const out = "/tmp/report.json";
```

#### Do

```js
const dir = await fs.mkdtemp(path.join(os.tmpdir(), "report-"));
```

### JSX

#### Don't

```jsx
const out = "/tmp/report.json";
```

#### Do

```jsx
const dir = await fs.mkdtemp(path.join(os.tmpdir(), "report-"));
```

### TypeScript

#### Don't

```ts
const out = "/tmp/report.json";
```

#### Do

```ts
const dir = await fs.mkdtemp(path.join(os.tmpdir(), "report-"));
```

### TSX

#### Don't

```tsx
const out = "/tmp/report.json";
```

#### Do

```tsx
const dir = await fs.mkdtemp(path.join(os.tmpdir(), "report-"));
```
