---
id: js/no-sonar-comments
title: "No NOSONAR comment"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the comment silenced must be looked at"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-sonar-comments]
since: 0.5.0
---

A comment does not contain `NOSONAR`.

**Why:** It hides every finding on the line, including ones nobody looked at.

```codelaw

match:
  kind: comment
  regex: '(?i)\bNOSONAR\b'
message: "A NOSONAR comment silences every check on the line; fix the finding or suppress the one rule"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
run(x); // NOSONAR
```

#### Do

```js
run(x);
```

### JSX

#### Don't

```jsx
run(x); // NOSONAR
```

#### Do

```jsx
run(x);
```

### TypeScript

#### Don't

```ts
run(x); // NOSONAR
```

#### Do

```ts
run(x);
```

### TSX

#### Don't

```tsx
run(x); // NOSONAR
```

#### Do

```tsx
run(x);
```
