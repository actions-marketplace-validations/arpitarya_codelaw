---
id: js/no-unnecessary-string-trim
title: "trimStart() before startsWith(), trimEnd() before endsWith()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-string-trim]
since: 0.5.0
---

`value.trim().startsWith(text)` only looks at the start of the string, so trimming its end is wasted: `value.trimStart().startsWith(text)`. Likewise `trim().endsWith(text)` is `trimEnd().endsWith(text)`. The search value is a string literal that does not end (for `startsWith`) or start (for `endsWith`) with whitespace, and there is no position argument; otherwise the trimmed side could be seen.

**Why:** Trimming only the side that matters says what the check depends on.

```codelaw
match:
  kind: call_expression
  any:
    - all:
        - pattern: "$S.trim().startsWith($T)"
        - has: { field: function, has: { field: object, has: { field: function, has: { field: property, pattern: $P1 } } } }
    - all:
        - pattern: "$S.trim().endsWith($U)"
        - has: { field: function, has: { field: object, has: { field: function, has: { field: property, pattern: $P2 } } } }
constraints:
  T: { kind: string, regex: '^.[^"'']*[^\s"'']["'']$' }
  U: { kind: string, regex: '^["''][^\s"''][^"'']*["'']$' }
message: "Trim only the side the check reads"
fix:
  - replace: { target: $P1, with: trimStart }
  - replace: { target: $P2, with: trimEnd }
```

## Examples

### JavaScript

#### Don't

```js
const flag = arg.trim().startsWith("-");
```

#### Do

```js
const flag = arg.trimStart().startsWith("-");
```


### JSX

#### Don't

```jsx
const flag = arg.trim().startsWith("-");
```

#### Do

```jsx
const flag = arg.trimStart().startsWith("-");
```


### TypeScript

#### Don't

```ts
const flag = arg.trim().startsWith("-");
```

#### Do

```ts
const flag = arg.trimStart().startsWith("-");
```


### TSX

#### Don't

```tsx
const flag = arg.trim().startsWith("-");
```

#### Do

```tsx
const flag = arg.trimStart().startsWith("-");
```
