---
id: js/no-selector-as-dom-name
title: "No selector syntax where a DOM name is expected"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-selector-as-dom-name]
since: 0.5.0
---

An id or a class name is passed without its selector prefix: `getElementById("#app")` is `getElementById("app")`, and `classList.add(".open")` is `classList.add("open")`.

**Why:** These APIs take the name itself; with `#` or `.` they look for an id or class that has that character, and find nothing.

```codelaw
match:
  any:
    - pattern: "$D.getElementById($ID)"
    - pattern: "$D.getElementsByClassName($CL)"
    - pattern: "$E.classList.$M($TOK)"
    - pattern: "$E.classList.$M($TOK, $$$R)"
constraints:
  ID: { kind: string, regex: '^["'']#[^"''\s.#\[]+["'']$' }
  CL: { kind: string, regex: '^["'']\.[A-Za-z_-][^"''\s.#\[]*["'']$' }
  TOK: { kind: string, regex: '^["'']\.[A-Za-z_-][^"''\s.#\[]*["'']$' }
  M: { regex: '^(add|remove|toggle|contains|replace)$' }
message: "Pass the name without `#` or `.`"
fix:
  - edit-text: { target: $ID, pattern: '^(["''])#', replace: '${1}' }
  - edit-text: { target: $CL, pattern: '^(["''])\.', replace: '${1}' }
  - edit-text: { target: $TOK, pattern: '^(["''])\.', replace: '${1}' }
```

## Examples

### JavaScript

#### Don't

```js
const app = document.getElementById("#app");
```

#### Do

```js
const app = document.getElementById("app");
```


### JSX

#### Don't

```jsx
const app = document.getElementById("#app");
```

#### Do

```jsx
const app = document.getElementById("app");
```


### TypeScript

#### Don't

```ts
const app = document.getElementById("#app");
```

#### Do

```ts
const app = document.getElementById("app");
```


### TSX

#### Don't

```tsx
const app = document.getElementById("#app");
```

#### Do

```tsx
const app = document.getElementById("app");
```
