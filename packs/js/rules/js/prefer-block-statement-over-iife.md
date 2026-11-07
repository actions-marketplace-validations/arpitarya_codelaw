---
id: js/prefer-block-statement-over-iife
title: "A block rather than an arrow IIFE used only for scope"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-block-statement-over-iife]
since: 0.5.0
---

An arrow function written and called at once as a statement, only to scope its `let` and `const`, is a plain block: `{ … }` scopes them the same.

**Why:** The block says the same thing without a function and a call.

```codelaw
match:
  kind: expression_statement
  pattern: "(() => { $$$B })();"
  not:
    any:
      - regex: '^\(\s*async'
      - has:
          stopBy: end
          any:
            - kind: return_statement
            - kind: variable_declaration
            - kind: function_declaration
            - kind: generator_function_declaration
message: "Use a block statement"
fix: "{\n  $$$B\n}"
```

## Examples

### JavaScript

#### Don't

```js
(() => {
  const tmp = load();
  use(tmp);
})();
```

#### Do

```js
{
  const tmp = load();
  use(tmp);
}
```


### JSX

#### Don't

```jsx
(() => {
  const tmp = load();
  use(tmp);
})();
```

#### Do

```jsx
{
  const tmp = load();
  use(tmp);
}
```


### TypeScript

#### Don't

```ts
(() => {
  const tmp = load();
  use(tmp);
})();
```

#### Do

```ts
{
  const tmp = load();
  use(tmp);
}
```


### TSX

#### Don't

```tsx
(() => {
  const tmp = load();
  use(tmp);
})();
```

#### Do

```tsx
{
  const tmp = load();
  use(tmp);
}
```
