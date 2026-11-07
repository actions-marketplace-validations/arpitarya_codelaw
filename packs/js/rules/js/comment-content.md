---
id: js/comment-content
title: "Product and language names in comments are spelled their own way"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/comment-content]
since: 0.5.0
---

A comment writes `JavaScript`, `TypeScript`, `Node.js`, `GitHub`, `GitLab`, `ECMAScript`, `WebAssembly`, `GraphQL`, `PostgreSQL`, `MongoDB`, `macOS` and `YouTube` as their owners do — not `javascript`, `Github` or `TYPESCRIPT`. A name that is part of a path, a domain, a file name or code (next to `.`, `/`, `@`, `-` or `_`) is left, and so is any comment holding backticks or a URL. This list is codelaw's own; upstream's is longer and configurable.

**Why:** Consistent names read as care; a lowercase brand in prose reads as a typo.

```codelaw
match:
  kind: comment
  regex: '(^|[^.\w/@`-])(javascript|Javascript|javaScript|JAVASCRIPT|typescript|Typescript|typeScript|TYPESCRIPT|nodejs|NodeJS|Nodejs|NODEJS|node\.JS|github|Github|GITHUB|gitlab|Gitlab|GITLAB|ecmascript|Ecmascript|ECMAscript|webassembly|Webassembly|WebAssmebly|graphql|Graphql|GraphQl|postgresql|Postgresql|PostgresQL|mongodb|Mongodb|MongoDb|macos|MacOS|MacOs|Macos|youtube|Youtube|YOUTUBE)([^.\w/@-]|$)'
  not: { any: [{ regex: '`' }, { regex: '://' }] }
message: "Spell this name the way its owner does"
fix:
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:javascript|Javascript|javaScript|JAVASCRIPT)([^.\w/@-]|$)', replace: '${1}JavaScript${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:typescript|Typescript|typeScript|TYPESCRIPT)([^.\w/@-]|$)', replace: '${1}TypeScript${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:nodejs|NodeJS|Nodejs|NODEJS)([^.\w/@-]|$)', replace: '${1}Node.js${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:github|Github|GITHUB)([^.\w/@-]|$)', replace: '${1}GitHub${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:gitlab|Gitlab|GITLAB)([^.\w/@-]|$)', replace: '${1}GitLab${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:ecmascript|Ecmascript|ECMAscript)([^.\w/@-]|$)', replace: '${1}ECMAScript${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:webassembly|Webassembly|WebAssmebly)([^.\w/@-]|$)', replace: '${1}WebAssembly${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:graphql|Graphql|GraphQl)([^.\w/@-]|$)', replace: '${1}GraphQL${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:postgresql|Postgresql|PostgresQL)([^.\w/@-]|$)', replace: '${1}PostgreSQL${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:mongodb|Mongodb|MongoDb)([^.\w/@-]|$)', replace: '${1}MongoDB${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:macos|MacOS|MacOs|Macos)([^.\w/@-]|$)', replace: '${1}macOS${2}' }
  - edit-text: { pattern: '(^|[^.\w/@`-])(?:youtube|Youtube|YOUTUBE)([^.\w/@-]|$)', replace: '${1}YouTube${2}' }
```

## Examples

### JavaScript

#### Don't

```js
// Report the bug on the github issue tracker.
```

#### Do

```js
// Report the bug on the GitHub issue tracker.
```


### JSX

#### Don't

```jsx
// Report the bug on the github issue tracker.
```

#### Do

```jsx
// Report the bug on the GitHub issue tracker.
```


### TypeScript

#### Don't

```ts
// Report the bug on the github issue tracker.
```

#### Do

```ts
// Report the bug on the GitHub issue tracker.
```


### TSX

#### Don't

```tsx
// Report the bug on the github issue tracker.
```

#### Do

```tsx
// Report the bug on the GitHub issue tracker.
```
