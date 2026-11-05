---
id: js/consistent-json-file-read
title: "Read JSON files as text"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the file's encoding is not in the code"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-json-file-read]
since: 0.5.0
---

`JSON.parse(await readFile(path))` passes the encoding, so `JSON.parse` gets a string, not a `Buffer` it converts implicitly.

**Why:** Parsing a `Buffer` relies on its `toString()`; naming the encoding makes the conversion explicit.

```codelaw
match:
  pattern: JSON.parse($$$A)
  has:
    field: arguments
    has:
      nthChild: 1
      kind: await_expression
      has:
        kind: call_expression
        pattern: $C
        has:
          field: function
          regex: (^|\.)readFile$
        all:
        - has:
            field: arguments
            not:
              has:
                kind: string
message: Read the file as text (`readFile(path, 'utf8')`) before `JSON.parse`
```

## How to fix

1. Pass the file's encoding, usually `'utf8'`, as `readFile`'s second argument.

## Examples

### JavaScript

#### Don't

```js
const pkg = JSON.parse(await readFile(path));
```

#### Do

```js
const pkg = JSON.parse(await readFile(path, 'utf8'));
```

### JSX

#### Don't

```jsx
const pkg = JSON.parse(await readFile(path));
```

#### Do

```jsx
const pkg = JSON.parse(await readFile(path, 'utf8'));
```

### TypeScript

#### Don't

```ts
const pkg = JSON.parse(await readFile(path));
```

#### Do

```ts
const pkg = JSON.parse(await readFile(path, 'utf8'));
```

### TSX

#### Don't

```tsx
const pkg = JSON.parse(await readFile(path));
```

#### Do

```tsx
const pkg = JSON.parse(await readFile(path, 'utf8'));
```
