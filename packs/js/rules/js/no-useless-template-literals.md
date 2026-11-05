---
id: js/no-useless-template-literals
title: "No template literal without substitutions"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "changing the quotes may need re-escaping the text"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-template-literals]
since: 0.5.0
---

A one-line template literal without substitutions, not used as a tag's argument, is a plain string.

**Why:** Backticks signal interpolation that is not there.

```codelaw
match:
  kind: template_string
  not:
    has:
      kind: template_substitution
  all:
  - not:
      regex: \n
  - not:
      inside:
        kind: call_expression
        field: arguments
  - not:
      inside:
        kind: call_expression
message: This template literal has no `${…}`; a plain string says the same
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const title = `Settings`;
```

#### Do

```js
const title = "Settings";
```

### JSX

#### Don't

```jsx
const title = `Settings`;
```

#### Do

```jsx
const title = "Settings";
```

### TypeScript

#### Don't

```ts
const title = `Settings`;
```

#### Do

```ts
const title = "Settings";
```

### TSX

#### Don't

```tsx
const title = `Settings`;
```

#### Do

```tsx
const title = "Settings";
```
