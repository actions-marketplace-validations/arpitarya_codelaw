---
id: js/no-invalid-remove-event-listener
title: "removeEventListener takes a reference"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the listener must be stored where it was added, in other code"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-remove-event-listener]
since: 0.5.0
---

The listener passed to `removeEventListener` is not a new function (an inline function or a `.bind()` call).

**Why:** A new function is never the one that was added, so the call removes nothing and the listener stays.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    has:
      field: property
      regex: ^removeEventListener$
  all:
  - has:
      field: arguments
      has:
        nthChild: 2
        any:
        - kind: arrow_function
        - kind: function_expression
        - kind: call_expression
          regex: \.bind\(
message: A new function here was never added, so nothing is removed
```

## How to fix

1. Keep the listener in a variable, add that, and remove the same variable.

## Examples

### JavaScript

#### Don't

```js
button.removeEventListener("click", () => close());
```

#### Do

```js
button.removeEventListener("click", onClick);
```

### JSX

#### Don't

```jsx
button.removeEventListener("click", () => close());
```

#### Do

```jsx
button.removeEventListener("click", onClick);
```

### TypeScript

#### Don't

```ts
button.removeEventListener("click", () => close());
```

#### Do

```ts
button.removeEventListener("click", onClick);
```

### TSX

#### Don't

```tsx
button.removeEventListener("click", () => close());
```

#### Do

```tsx
button.removeEventListener("click", onClick);
```
