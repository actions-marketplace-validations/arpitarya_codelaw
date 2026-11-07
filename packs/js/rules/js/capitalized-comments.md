---
id: js/capitalized-comments
title: "A comment starts with a capital letter"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/capitalized-comments]
since: 0.5.0
---

A comment whose first word is in lower case starts it with a capital. Directive comments (`eslint`, `jshint`, `jscs`, `istanbul`, `global`, `globals`, `exported`, and tool pragmas such as `@ts-`, `prettier`, `c8`, `webpack`) and comments that start with a URL or with no letter are left alone.

**Why:** A comment is a sentence; a capital marks where it starts. A matter of taste, so in the opt-in `js/taste` preset.

```codelaw
match:
  kind: comment
  pattern: $C
  all:
    - regex: '^(//|/\*+)[ \t]*[a-z]'
    - not:
        regex: '^(//|/\*+)[ \t]*(eslint|jshint|jslint|jscs|istanbul|global|globals|exported|c8|v8|prettier|biome|codelaw|tslint|webpack[A-Z]|vite-|https?:|www\.)'
transform:
  PRE:
    replace: { source: $C, replace: '^(//[ \t]*|/\*+[ \t]*)(?s:.*)$', by: '$1' }
  REST:
    replace: { source: $C, replace: '^(?://[ \t]*|/\*+[ \t]*)', by: '' }
message: "Start the comment with a capital letter"
fix: "$PRE$REST|upper-first"
```

## Examples

### JavaScript

#### Don't

```js
// load the settings first
const settings = load();
```

#### Do

```js
// Load the settings first
const settings = load();
```


### JSX

#### Don't

```jsx
// load the settings first
const settings = load();
```

#### Do

```jsx
// Load the settings first
const settings = load();
```


### TypeScript

#### Don't

```ts
// load the settings first
const settings = load();
```

#### Do

```ts
// Load the settings first
const settings = load();
```


### TSX

#### Don't

```tsx
// load the settings first
const settings = load();
```

#### Do

```tsx
// Load the settings first
const settings = load();
```
