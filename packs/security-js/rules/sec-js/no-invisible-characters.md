---
id: sec-js/no-invisible-characters
title: "No invisible filler characters in code"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a comment is removed whole, since its hidden text cannot be told from the rest; a string or name with such a character has no mechanical fix"
security: { cwe: [116], owasp: ["A08:2025"], severity: high }
stability: preview
ported_from: [eslint-plugin-security/detect-invisible-characters]
since: 0.5.0
---

Source code and comments hold no HANGUL FILLER (U+3164) or HALFWIDTH HANGUL FILLER (U+FFA0): valid in a string, a comment or an identifier, drawn as nothing. A visible escape (`"\u3164"`) is not the character and is not reported. Where a codebase uses U+3164 as a blank placeholder on purpose, turn the rule off for that file.

**Why:** These characters render as empty space, so a reviewer sees nothing where a token lives: `admin` and `admㅤin` are two names that look alike, and the one reviewed is not the one used.

```codelaw
match:
  any:
    - pattern: $C
      kind: comment
      regex: '[\u3164\uFFA0]'
    - kind: string
      regex: '[\u3164\uFFA0]'
    - kind: template_string
      regex: '[\u3164\uFFA0]'
    - kind: identifier
      regex: '[\u3164\uFFA0]'
message: "This code holds an invisible filler character, which a reviewer cannot see"
fix:
  - remove: $C
```

## How to fix

1. Delete the character: show hidden characters in the editor (or run `git diff --word-diff-regex=.`) and remove each one.
2. If a string needs the character, write it as an escape (`\u3164`) so the source does not hide it.

## Examples

### JavaScript

#### Don't

```js
const level = readLevel();
// check the access level ㅤ admin
```

#### Do

```js
const level = readLevel();
// check the access level admin
```

#### Fixed

```js
const level = readLevel();
```


### JSX

#### Don't

```jsx
const level = readLevel();
// check the access level ㅤ admin
```

#### Do

```jsx
const level = readLevel();
// check the access level admin
```

#### Fixed

```jsx
const level = readLevel();
```


### TypeScript

#### Don't

```ts
const level = readLevel();
// check the access level ㅤ admin
```

#### Do

```ts
const level = readLevel();
// check the access level admin
```

#### Fixed

```ts
const level = readLevel();
```


### TSX

#### Don't

```tsx
const level = readLevel();
// check the access level ㅤ admin
```

#### Do

```tsx
const level = readLevel();
// check the access level admin
```

#### Fixed

```tsx
const level = readLevel();
```
