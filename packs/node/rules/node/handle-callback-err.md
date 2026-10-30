---
id: node/handle-callback-err
title: "Handle the err argument"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what to do with the error (pass it on, log it, retry) is the author's decision"
stability: preview
ported_from: [eslint-plugin-n/handle-callback-err]
since: 0.5.0
---

A function whose first parameter is `err` reads it (the upstream default pattern, `"err"`).

**Why:** A callback that never looks at `err` carries on with `undefined` results when the operation fails, and the failure is lost.

```codelaw
match:
  all:
    - any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}]
    - any:
        - has: { field: parameter, kind: identifier, regex: ^err$, pattern: $E }
        - has:
            field: parameters
            has:
              nthChild: 1
              any:
                - { kind: identifier, regex: ^err$, pattern: $E }
                - { kind: required_parameter, has: { field: pattern, kind: identifier, regex: ^err$, pattern: $E } }
                - { kind: optional_parameter, has: { field: pattern, kind: identifier, regex: ^err$, pattern: $E } }
where:
  $E: unused
message: "`err` is never read; handle the error or pass it on"
```

## How to fix

1. Check `err` first, and return early: pass it to your own callback, throw it, or log it.
2. If the error truly cannot happen, name the parameter `_err` to say so.

## Examples

### JavaScript

#### Don't

```js
fs.readFile(path, function (err, data) {
  show(data);
});
```

#### Do

```js
fs.readFile(path, function (err, data) {
  if (err) {
    return report(err);
  }
  show(data);
});
```

### JSX

#### Don't

```jsx
fs.readFile(path, function (err, data) {
  show(data);
});
```

#### Do

```jsx
fs.readFile(path, function (err, data) {
  if (err) {
    return report(err);
  }
  show(data);
});
```

### TypeScript

#### Don't

```ts
fs.readFile(path, function (err, data) {
  show(data);
});
```

#### Do

```ts
fs.readFile(path, function (err, data) {
  if (err) {
    return report(err);
  }
  show(data);
});
```

### TSX

#### Don't

```tsx
fs.readFile(path, function (err, data) {
  show(data);
});
```

#### Do

```tsx
fs.readFile(path, function (err, data) {
  if (err) {
    return report(err);
  }
  show(data);
});
```
