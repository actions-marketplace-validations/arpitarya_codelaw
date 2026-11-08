---
id: js/no-invalid-file-input-accept
title: "A file input's accept lists lowercase types and dotted extensions"
languages: [jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-file-input-accept]
since: 0.5.0
---

`accept` on `<input type="file">` takes MIME types and file extensions, case-insensitively. It is written in lowercase, `image/jpg` (not a registered type) is `image/jpeg`, and an extension starts with a dot (`png` is `.png`; a bare word matches nothing). Upstream also reports duplicate entries and non-literal values; those are not reported here.

**Why:** A misspelt entry silently filters out the files the user should be able to pick.

```codelaw
match:
  kind: jsx_attribute
  regex: '^accept\s*='
  inside:
    any: [{ kind: jsx_self_closing_element }, { kind: jsx_opening_element }]
    has: { field: name, regex: '^input$' }
    all:
      - has: { kind: jsx_attribute, regex: '^type\s*=\s*["'']file["'']$' }
  has:
    kind: string
    pattern: $V
    any:
      - regex: '[A-Z]'
      - regex: '(?i)image/jpg([,"''\s]|$)'
      - regex: '(^["'']|,)\s*[A-Za-z0-9]+\s*(,|["'']$)'
transform:
  L: { convert: { source: $V, toCase: lowerCase } }
  J: { replace: { source: $L, replace: 'image/jpg([,"''\s]|$)', by: 'image/jpeg${1}' } }
  D1: { replace: { source: $J, replace: '(^["'']|,)(\s*)([a-z0-9]+)(\s*)(,|["'']$)', by: '${1}${2}.${3}${4}${5}' } }
  D2: { replace: { source: $D1, replace: '(^["'']|,)(\s*)([a-z0-9]+)(\s*)(,|["'']$)', by: '${1}${2}.${3}${4}${5}' } }
message: "Write `accept` with lowercase types and dotted extensions"
fix:
  - replace: { target: $V, with: $D2 }
```

## Examples

### JSX

#### Don't

```jsx
const picker = <input type="file" accept="image/jpg" />;
```

#### Do

```jsx
const picker = <input type="file" accept="image/jpeg" />;
```


### TSX

#### Don't

```tsx
const picker = <input type="file" accept="image/jpg" />;
```

#### Do

```tsx
const picker = <input type="file" accept="image/jpeg" />;
```
