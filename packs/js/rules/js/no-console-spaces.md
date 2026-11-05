---
id: js/no-console-spaces
title: "No leading or trailing space in console arguments"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "trimming the string is a computed edit a template cannot make"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint-plugin-unicorn/no-console-spaces]
since: 0.5.0
---

A string argument of a `console.log` (or `debug`, `info`, `warn`, `error`) call with more than one argument does not start or end with a space.

**Why:** `console` joins arguments with a space, so the extra one prints two.

```codelaw
match:
  kind: string_fragment
  any:
  - regex: '^ '
  - regex: ' $'
  inside:
    kind: string
    inside:
      kind: arguments
      inside:
        kind: call_expression
        has:
          field: function
          regex: ^console\.(log|debug|info|warn|error)$
        all:
        - has:
            field: arguments
            has:
              nthChild: 2
message: '`console` already puts a space between arguments'
```

## How to fix

1. Remove the leading or trailing space from the string.

## Examples

### JavaScript

#### Don't

```js
console.log("total:", n, " items");
```

#### Do

```js
console.log("total:", n, "items");
```

### JSX

#### Don't

```jsx
console.log("total:", n, " items");
```

#### Do

```jsx
console.log("total:", n, "items");
```

### TypeScript

#### Don't

```ts
console.log("total:", n, " items");
```

#### Do

```ts
console.log("total:", n, "items");
```

### TSX

#### Don't

```tsx
console.log("total:", n, " items");
```

#### Do

```tsx
console.log("total:", n, "items");
```
