---
id: js/camelcase
title: "camelCase names"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the rename covers every use in this file; names read from other files or from strings are for the reviewer"
stability: preview
ported_from: [eslint/camelcase, eslint-plugin-sonarjs/variable-name]
since: 0.5.0
---

A variable, function or parameter declared with a lower-case first letter is named in camelCase, not snake_case: `user_name` is `userName`. Constants in ALL_CAPS, leading and trailing underscores, and PascalCase class names are left alone. ESLint also checks property names; this port checks the names a file declares.

**Why:** One naming convention lets a reader tell a local from a property from a constant at a glance.

```codelaw
match:
  kind: identifier
  pattern: $N
  all:
    - regex: '^[a-z$][A-Za-z0-9$]*(_[A-Za-z0-9$]+)+$'
    - any:
        - inside: { kind: variable_declarator, field: name }
        - inside: { kind: function_declaration, field: name }
        - inside: { kind: formal_parameters }
        - inside: { kind: required_parameter, field: pattern }
transform:
  CAMEL:
    convert: { source: $N, toCase: camelCase }
message: "Name `$N` in camelCase"
fix:
  - rename-symbol-local: { target: $N, to: $CAMEL }
```

## Examples

### JavaScript

#### Don't

```js
const user_name = read();
print(user_name);
```

#### Do

```js
const userName = read();
print(userName);
```


### JSX

#### Don't

```jsx
const user_name = read();
print(user_name);
```

#### Do

```jsx
const userName = read();
print(userName);
```


### TypeScript

#### Don't

```ts
const user_name = read();
print(user_name);
```

#### Do

```ts
const userName = read();
print(userName);
```


### TSX

#### Don't

```tsx
const user_name = read();
print(user_name);
```

#### Do

```tsx
const userName = read();
print(userName);
```
