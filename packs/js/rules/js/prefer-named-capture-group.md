---
id: js/prefer-named-capture-group
title: "Name capture groups"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the group name is the author's choice"
fix_exception: value
stability: preview
ported_from: [eslint/prefer-named-capture-group]
since: 0.5.0
---

A capture group in a regular-expression literal is named (`(?<year>…)`) or non-capturing (`(?:…)`).

**Why:** A named group is read by name, so adding a group earlier in the pattern does not shift every index.

```codelaw

match:
  kind: regex
  has: { kind: regex_pattern, regex: '(^|[^\\])\([^?]' }
message: "Name the capture group: `(?<name>…)`, or make it `(?:…)`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const m = /(\d+)-(\d+)/.exec(s);
```

#### Do

```js
const m = /(?<from>\d+)-(?<to>\d+)/.exec(s);
```

### JSX

#### Don't

```jsx
const m = /(\d+)-(\d+)/.exec(s);
```

#### Do

```jsx
const m = /(?<from>\d+)-(?<to>\d+)/.exec(s);
```

### TypeScript

#### Don't

```ts
const m = /(\d+)-(\d+)/.exec(s);
```

#### Do

```ts
const m = /(?<from>\d+)-(?<to>\d+)/.exec(s);
```

### TSX

#### Don't

```tsx
const m = /(\d+)-(\d+)/.exec(s);
```

#### Do

```tsx
const m = /(?<from>\d+)-(?<to>\d+)/.exec(s);
```
