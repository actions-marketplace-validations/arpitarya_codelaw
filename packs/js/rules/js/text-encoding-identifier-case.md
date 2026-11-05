---
id: js/text-encoding-identifier-case
title: "Write the encoding as \"utf8\""
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "an API outside Node (such as `TextDecoder` in old engines) may expect the hyphenated name"
stability: preview
ported_from: [eslint-plugin-unicorn/text-encoding-identifier-case]
since: 0.5.0
---

An encoding name passed as an argument is spelled `utf8`.

**Why:** One spelling makes the encoding greppable; Node accepts them all.

```codelaw

match:
  kind: string_fragment
  regex: '^(UTF-8|utf-8|UTF8|Utf8|Utf-8)$'
  inside: { kind: string, inside: { kind: arguments } }
message: "Write the encoding as `utf8`"
fix: "utf8"
```

## Examples

### JavaScript

#### Don't

```js
const text = await readFile(path, "UTF-8");
```

#### Do

```js
const text = await readFile(path, "utf8");
```

### JSX

#### Don't

```jsx
const text = await readFile(path, "UTF-8");
```

#### Do

```jsx
const text = await readFile(path, "utf8");
```

### TypeScript

#### Don't

```ts
const text = await readFile(path, "UTF-8");
```

#### Do

```ts
const text = await readFile(path, "utf8");
```

### TSX

#### Don't

```tsx
const text = await readFile(path, "UTF-8");
```

#### Do

```tsx
const text = await readFile(path, "utf8");
```
