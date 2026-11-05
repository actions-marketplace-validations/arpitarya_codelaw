---
id: js/no-promise-executor-return
title: "A promise executor returns nothing"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "dropping an ignored return value is exact; a call to `resolve` or `reject`, or a call taking one of them, is auto, and any other returned value (it may have been meant for `resolve`) reports suggest"
stability: preview
ported_from: [eslint/no-promise-executor-return]
since: 0.5.0
---

The function passed to `new Promise` does not return a value.

**Why:** The executor's return value is ignored; returning one usually means `resolve` was forgotten.

```codelaw
match:
  any:
  - kind: return_statement
    pattern: $S1
    has: { pattern: $V1 }
    regex: ^return\s*[^\s;]
    inside:
      stopBy:
        any:
        - {kind: function_declaration}
        - {kind: function_expression}
        - {kind: arrow_function}
        - {kind: method_definition}
      any:
      - {kind: arrow_function}
      - {kind: function_expression}
      inside:
        kind: arguments
        inside:
          kind: new_expression
          has: {field: constructor, regex: ^Promise$}
  - pattern: $E
    all:
    - inside:
        kind: arrow_function
        field: body
        inside:
          kind: arguments
          inside:
            kind: new_expression
            has: {field: constructor, regex: ^Promise$}
    - not: {kind: statement_block}
    - not: {kind: unary_expression, regex: ^void\b}
message: A promise executor's return value is ignored; call `resolve` instead
auto_when:
  any:
    - kind: return_statement
      has:
        kind: call_expression
        any:
          - has: { field: function, regex: '^(resolve|reject)$' }
          - has: { field: arguments, has: { kind: identifier, regex: '^(resolve|reject)$' } }
    - kind: call_expression
      any:
        - has: { field: function, regex: '^(resolve|reject)$' }
        - has: { field: arguments, has: { kind: identifier, regex: '^(resolve|reject)$' } }
fix:
  - replace: { target: $S1, with: "$V1; return;" }
  - replace: { target: $E, with: "{ $E; }" }
```

## How to fix

1. Call `resolve(value)` instead of returning it, or wrap an expression body in braces.

## Examples

### JavaScript

#### Don't

```js
new Promise((resolve) => setTimeout(resolve, 10));
```

#### Fixed

```js
new Promise((resolve) => { setTimeout(resolve, 10); });
```

#### Do

```js
new Promise((resolve) => {
  setTimeout(resolve, 10);
});
```

### JSX

#### Don't

```jsx
new Promise((resolve) => setTimeout(resolve, 10));
```

#### Fixed

```jsx
new Promise((resolve) => { setTimeout(resolve, 10); });
```

#### Do

```jsx
new Promise((resolve) => {
  setTimeout(resolve, 10);
});
```

### TypeScript

#### Don't

```ts
new Promise((resolve) => setTimeout(resolve, 10));
```

#### Fixed

```ts
new Promise((resolve) => { setTimeout(resolve, 10); });
```

#### Do

```ts
new Promise((resolve) => {
  setTimeout(resolve, 10);
});
```

### TSX

#### Don't

```tsx
new Promise((resolve) => setTimeout(resolve, 10));
```

#### Fixed

```tsx
new Promise((resolve) => { setTimeout(resolve, 10); });
```

#### Do

```tsx
new Promise((resolve) => {
  setTimeout(resolve, 10);
});
```
