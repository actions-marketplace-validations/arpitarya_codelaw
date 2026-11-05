---
id: js/no-implied-eval
title: "No string code in setTimeout"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "turning the string into a function means writing that code"
fix_exception: authored
stability: preview
ported_from: [eslint/no-implied-eval]
since: 0.5.0
---

`setTimeout`, `setInterval` and `execScript` are not given code as a string.

**Why:** The string is evaluated like `eval`: slow, invisible to tools, and an injection risk.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: ^(window\.|globalThis\.|self\.)?(setTimeout|setInterval|execScript)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        any:
        - kind: string
        - kind: template_string
        - kind: binary_expression
          has:
            kind: string
message: A string here is evaluated as code; pass a function
```

## How to fix

1. Pass a function: `setTimeout(() => refresh(), 1000)`.

## Examples

### JavaScript

#### Don't

```js
setTimeout("refresh()", 1000);
```

#### Do

```js
setTimeout(refresh, 1000);
```

### JSX

#### Don't

```jsx
setTimeout("refresh()", 1000);
```

#### Do

```jsx
setTimeout(refresh, 1000);
```

### TypeScript

#### Don't

```ts
setTimeout("refresh()", 1000);
```

#### Do

```ts
setTimeout(refresh, 1000);
```

### TSX

#### Don't

```tsx
setTimeout("refresh()", 1000);
```

#### Do

```tsx
setTimeout(refresh, 1000);
```
