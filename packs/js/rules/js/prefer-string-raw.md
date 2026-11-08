---
id: js/prefer-string-raw
title: "String.raw for a string of escaped backslashes"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-string-raw]
since: 0.5.0
---

A string whose only escapes are doubled backslashes (`"C:\\temp"`) is written with `String.raw`, where each backslash is written once.

**Why:** Doubled backslashes are hard to count; `String.raw` shows the text as it is.

```codelaw
match:
  kind: string
  regex: '^["''](?:[^\\\n`$"'']|\\\\)*\\\\(?:[^\\\n`$"'']|\\\\)*["'']$'
  not:
    any:
      - regex: '\\\\["'']$'
      - inside: { kind: jsx_attribute }
      - inside: { kind: pair, field: key }
      - inside: { kind: import_statement }
      - inside: { kind: export_statement, has: { field: source, kind: string } }
      - inside: { kind: literal_type }
      - inside: { kind: enum_body, stopBy: end }
      - inside: { kind: call_expression, has: { field: function, regex: '^require$' } , stopBy: { kind: call_expression } }
message: "Write the backslashes once with `String.raw`"
fix:
  - edit-text: { pattern: '\\\\', replace: '\' }
  - edit-text: { pattern: '^["''](.*)["'']$', replace: 'String.raw`${1}`' }
```

## Examples

### JavaScript

#### Don't

```js
const path = "C:\\temp\\logs";
```

#### Do

```js
const path = String.raw`C:\temp\logs`;
```


### JSX

#### Don't

```jsx
const path = "C:\\temp\\logs";
```

#### Do

```jsx
const path = String.raw`C:\temp\logs`;
```


### TypeScript

#### Don't

```ts
const path = "C:\\temp\\logs";
```

#### Do

```ts
const path = String.raw`C:\temp\logs`;
```


### TSX

#### Don't

```tsx
const path = "C:\\temp\\logs";
```

#### Do

```tsx
const path = String.raw`C:\temp\logs`;
```
