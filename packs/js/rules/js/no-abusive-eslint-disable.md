---
id: js/no-abusive-eslint-disable
title: "Name the rules a disable comment turns off"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which rules were meant is known only to the author"
stability: preview
ported_from: [eslint-plugin-unicorn/no-abusive-eslint-disable]
since: 0.5.0
---

A comment that disables linting names the rules it disables.

**Why:** A blanket disable hides every finding on the line, including ones nobody meant to accept.

```codelaw

match:
  kind: comment
  regex: '^(//|/\*)\s*(eslint-disable|eslint-disable-line|eslint-disable-next-line|codelaw-ignore|codelaw-ignore-next-line)\s*(\*/)?$'
message: "Name the rules this comment turns off"
```

## How to fix

1. List the rule ids after the directive, separated by commas.

## Examples

### JavaScript

#### Don't

```js
// eslint-disable-next-line
run(eval(code));
```

#### Do

```js
// eslint-disable-next-line no-eval
run(eval(code));
```

### JSX

#### Don't

```jsx
// eslint-disable-next-line
run(eval(code));
```

#### Do

```jsx
// eslint-disable-next-line no-eval
run(eval(code));
```

### TypeScript

#### Don't

```ts
// eslint-disable-next-line
run(eval(code));
```

#### Do

```ts
// eslint-disable-next-line no-eval
run(eval(code));
```

### TSX

#### Don't

```tsx
// eslint-disable-next-line
run(eval(code));
```

#### Do

```tsx
// eslint-disable-next-line no-eval
run(eval(code));
```
