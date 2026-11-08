---
id: js/func-names
title: "A function expression has a name"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the name is the author's"
fix_exception: value
stability: preview
ported_from: [eslint/func-names]
since: 0.5.0
---

A `function` expression is given a name, as ESLint's default (`always`) asks.

**Why:** Named functions show in stack traces and can call themselves. A matter of taste, so in the opt-in `js/taste` preset.

```codelaw
match:
  kind: function_expression
  not:
    any:
      - has: { field: name, kind: identifier }
      - inside: { kind: export_statement }
message: "Give this function expression a name"
```

## How to fix

1. Write a name after `function` that says what it does.

## Examples

### JavaScript

#### Don't

```js
button.addEventListener("click", function () {
  submit();
});
```

#### Do

```js
button.addEventListener("click", function onClick() {
  submit();
});
```


### JSX

#### Don't

```jsx
button.addEventListener("click", function () {
  submit();
});
```

#### Do

```jsx
button.addEventListener("click", function onClick() {
  submit();
});
```


### TypeScript

#### Don't

```ts
button.addEventListener("click", function () {
  submit();
});
```

#### Do

```ts
button.addEventListener("click", function onClick() {
  submit();
});
```


### TSX

#### Don't

```tsx
button.addEventListener("click", function () {
  submit();
});
```

#### Do

```tsx
button.addEventListener("click", function onClick() {
  submit();
});
```
